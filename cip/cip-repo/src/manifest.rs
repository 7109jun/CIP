use serde_json::{json, Map, Value};

use crate::docs_toml::DocsToml;
use crate::pysource::PyFileInfo;

/// Maps a docs.toml chrome-permission key to the actual manifest.json
/// permission string(s) it implies. Most are 1:1; a couple need extra
/// permissions to actually work (e.g. scripting needs activeTab in MV3
/// for host-less injection, but we keep this conservative and 1:1 so
/// behavior is exactly "what you declared").
fn permission_manifest_name(key: &str) -> &str {
    match key {
        "web_navigation" => "webNavigation",
        other => other,
    }
}

pub struct ManifestInputs<'a> {
    pub doc: &'a DocsToml,
    pub background_info: Option<&'a PyFileInfo>,
    pub content_info: Option<&'a PyFileInfo>,
    pub popup_info: Option<&'a PyFileInfo>,
}

pub fn build_manifest(inputs: &ManifestInputs) -> Value {
    let doc = inputs.doc;
    let mut manifest = Map::new();

    manifest.insert("manifest_version".into(), json!(3));
    manifest.insert("name".into(), json!(doc.name));
    manifest.insert("version".into(), json!(doc.version));
    if let Some(desc) = &doc.description {
        manifest.insert("description".into(), json!(desc));
    }
    if let Some(icons) = &doc.icons {
        manifest.insert("icons".into(), json!(icons));
    }

    // permissions (API permissions, not host access)
    let mut perms: Vec<String> = doc
        .permissions
        .chrome
        .iter()
        .filter(|(_, enabled)| **enabled)
        .map(|(k, _)| permission_manifest_name(k).to_string())
        .collect();
    perms.sort();
    if !perms.is_empty() {
        manifest.insert("permissions".into(), json!(perms));
    }

    // host_permissions from webpages
    if !doc.permissions.webpages.is_empty() {
        manifest.insert(
            "host_permissions".into(),
            json!(doc.permissions.webpages),
        );
    }

    // background (service worker, MV3)
    if doc.extension.background.is_some() {
        manifest.insert(
            "background".into(),
            json!({
                "service_worker": "background.js",
                "type": "module"
            }),
        );
    }

    // content script
    if doc.extension.content.is_some() {
        let matches = if !doc.permissions.webpages.is_empty() {
            doc.permissions.webpages.clone()
        } else {
            vec!["<all_urls>".to_string()]
        };
        manifest.insert(
            "content_scripts".into(),
            json!([{
                "matches": matches.clone(),
                "js": ["cip/pyodide/pyodide.js", "cip/runtime.js", "content.js"],
                "run_at": "document_idle"
            }]),
        );
        // Content scripts fetch() the project's .py files and the vendored
        // Pyodide assets by chrome-extension:// URL; MV3 requires those be
        // declared web-accessible to be readable from the page's isolated
        // world.
        manifest.insert(
            "web_accessible_resources".into(),
            json!([{
                "resources": ["cip/**", "cip_pkg/**", "*.py"],
                "matches": matches
            }]),
        );
    }

    // popup (action)
    if doc.extension.popup.is_some() {
        manifest.insert(
            "action".into(),
            json!({ "default_popup": "popup.html" }),
        );
    }

    // options page
    if doc.extension.options.is_some() {
        manifest.insert(
            "options_page".into(),
            json!("options.html"),
        );
    }

    // commands, gathered from @chrome.command(...) decorators found while
    // scanning background.py (commands are handled in the background
    // context per Chrome's own model).
    if let Some(info) = inputs.background_info {
        if !info.commands.is_empty() {
            let mut cmds = Map::new();
            for name in &info.commands {
                cmds.insert(
                    name.clone(),
                    json!({ "description": format!("CIP command: {}", name) }),
                );
            }
            manifest.insert("commands".into(), Value::Object(cmds));
        }
    }

    Value::Object(manifest)
}
