use std::fs;
use std::path::Path;

fn unique_tmp_dir(tag: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "cip_test_{}_{}",
        tag,
        std::process::id()
    ));
    if dir.exists() {
        fs::remove_dir_all(&dir).ok();
    }
    fs::create_dir_all(&dir).unwrap();
    dir
}

#[path = "../src/docs_toml.rs"]
mod docs_toml;
#[path = "../src/manifest.rs"]
mod manifest;
#[path = "../src/pysource.rs"]
mod pysource;

#[test]
fn match_pattern_validation() {
    assert!(docs_toml::is_valid_match_pattern("https://example.com/*"));
    assert!(docs_toml::is_valid_match_pattern("https://*.github.com/*"));
    assert!(docs_toml::is_valid_match_pattern("<all_urls>"));
    assert!(docs_toml::is_valid_match_pattern("*://*/*"));
    assert!(!docs_toml::is_valid_match_pattern("https://example.com"));
    assert!(!docs_toml::is_valid_match_pattern("example.com/*"));
    assert!(!docs_toml::is_valid_match_pattern("not a url"));
}

#[test]
fn project_discovery_walks_up_directories() {
    let dir = unique_tmp_dir("discovery");
    fs::write(
        dir.join("docs.toml"),
        "name=\"x\"\nversion=\"1.0.0\"\n[extension]\nentry=\"main.py\"\n",
    )
    .unwrap();
    let nested = dir.join("a/b/c");
    fs::create_dir_all(&nested).unwrap();
    let found = docs_toml::discover_project(&nested).unwrap();
    assert_eq!(found, dir.join("docs.toml"));
}

#[test]
fn project_discovery_fails_without_docs_toml() {
    let dir = unique_tmp_dir("discovery_fail");
    // Deliberately no docs.toml anywhere under this fresh tmp dir tree.
    let result = docs_toml::discover_project(&dir);
    assert!(result.is_err());
}

#[test]
fn validate_reports_missing_name_and_version() {
    let dir = unique_tmp_dir("validate_missing");
    fs::write(dir.join("main.py"), "").unwrap();
    let toml_path = dir.join("docs.toml");
    let content = "name = \"\"\nversion = \"\"\n[extension]\nentry = \"main.py\"\n";
    fs::write(&toml_path, content).unwrap();
    let doc: docs_toml::DocsToml = toml::from_str(content).unwrap();
    let diags = docs_toml::validate(&doc, &dir, &toml_path);
    assert!(diags.iter().any(|d| d.message.contains("name")));
    assert!(diags.iter().any(|d| d.message.contains("version")));
}

#[test]
fn validate_reports_missing_entry_file() {
    let dir = unique_tmp_dir("validate_entry");
    let toml_path = dir.join("docs.toml");
    let content = "name = \"X\"\nversion = \"1.0.0\"\n[extension]\nentry = \"does_not_exist.py\"\n";
    fs::write(&toml_path, content).unwrap();
    let doc: docs_toml::DocsToml = toml::from_str(content).unwrap();
    let diags = docs_toml::validate(&doc, &dir, &toml_path);
    assert!(diags.iter().any(|d| d.message.contains("존재하지 않는")));
}

#[test]
fn validate_reports_invalid_webpage_pattern() {
    let dir = unique_tmp_dir("validate_webpage");
    fs::write(dir.join("main.py"), "").unwrap();
    let toml_path = dir.join("docs.toml");
    let content = r#"
name = "X"
version = "1.0.0"
[permissions]
webpages = ["https://example.com"]
[extension]
entry = "main.py"
"#;
    fs::write(&toml_path, content).unwrap();
    let doc: docs_toml::DocsToml = toml::from_str(content).unwrap();
    let diags = docs_toml::validate(&doc, &dir, &toml_path);
    assert!(diags.iter().any(|d| d.message.contains("Invalid webpage permission")));
    // and the hint should suggest the fixed-up pattern
    let hint = diags
        .iter()
        .find(|d| d.message.contains("Invalid webpage permission"))
        .unwrap()
        .hint
        .clone()
        .unwrap();
    assert_eq!(hint, "https://example.com/*");
}

#[test]
fn validate_rejects_unknown_chrome_permission() {
    let dir = unique_tmp_dir("validate_perm");
    fs::write(dir.join("main.py"), "").unwrap();
    let toml_path = dir.join("docs.toml");
    let content = r#"
name = "X"
version = "1.0.0"
[permissions.chrome]
not_a_real_permission = true
[extension]
entry = "main.py"
"#;
    fs::write(&toml_path, content).unwrap();
    let doc: docs_toml::DocsToml = toml::from_str(content).unwrap();
    let diags = docs_toml::validate(&doc, &dir, &toml_path);
    assert!(diags.iter().any(|d| d.message.contains("알 수 없는 chrome permission")));
}

#[test]
fn validate_passes_for_a_correct_project() {
    let dir = unique_tmp_dir("validate_ok");
    fs::write(dir.join("main.py"), "").unwrap();
    let toml_path = dir.join("docs.toml");
    let content = r#"
name = "X"
version = "1.0.0"
[permissions]
webpages = ["https://example.com/*"]
[permissions.chrome]
storage = true
[extension]
entry = "main.py"
"#;
    fs::write(&toml_path, content).unwrap();
    let doc: docs_toml::DocsToml = toml::from_str(content).unwrap();
    let diags = docs_toml::validate(&doc, &dir, &toml_path);
    assert!(diags.is_empty(), "unexpected diagnostics: {:?}", diags);
}

#[test]
fn manifest_generation_maps_permissions_and_contexts() {
    let content = r#"
name = "My Extension"
version = "1.0.0"
[permissions]
webpages = ["https://example.com/*"]
[permissions.chrome]
tabs = true
storage = true
[extension]
entry = "main.py"
popup = "popup.py"
content = "content.py"
background = "background.py"
"#;
    let doc: docs_toml::DocsToml = toml::from_str(content).unwrap();
    let inputs = manifest::ManifestInputs {
        doc: &doc,
        background_info: None,
        content_info: None,
        popup_info: None,
    };
    let m = manifest::build_manifest(&inputs);
    assert_eq!(m["manifest_version"], 3);
    assert_eq!(m["name"], "My Extension");
    assert_eq!(m["background"]["service_worker"], "background.js");
    assert_eq!(m["action"]["default_popup"], "popup.html");
    let perms: Vec<String> = m["permissions"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_str().unwrap().to_string())
        .collect();
    assert!(perms.contains(&"tabs".to_string()));
    assert!(perms.contains(&"storage".to_string()));
    assert_eq!(m["host_permissions"][0], "https://example.com/*");
    assert!(m.get("web_accessible_resources").is_some());
}

#[test]
fn manifest_generation_omits_optional_contexts_when_undeclared() {
    let content = "name = \"X\"\nversion = \"1.0.0\"\n[extension]\nentry = \"main.py\"\n";
    let doc: docs_toml::DocsToml = toml::from_str(content).unwrap();
    let inputs = manifest::ManifestInputs {
        doc: &doc,
        background_info: None,
        content_info: None,
        popup_info: None,
    };
    let m = manifest::build_manifest(&inputs);
    assert!(m.get("background").is_none());
    assert!(m.get("action").is_none());
    assert!(m.get("content_scripts").is_none());
}

#[test]
fn pysource_extracts_commands_and_rejects_unknown_decorator() {
    let dir = unique_tmp_dir("pysource");
    let path = dir.join("background.py");
    fs::write(
        &path,
        "from cip import chrome\n\n@chrome.command(\"open\")\ndef open_page():\n    pass\n",
    )
    .unwrap();
    let (diags, info) = pysource::check_file(&path);
    assert!(diags.is_empty(), "unexpected diagnostics: {:?}", diags);
    assert_eq!(info.commands, vec!["open".to_string()]);
}

#[test]
fn pysource_rejects_unknown_decorator_and_module() {
    let dir = unique_tmp_dir("pysource_bad");
    let path = dir.join("background.py");
    fs::write(
        &path,
        "from cip import chrome, not_a_real_module\n\n@chrome.not_a_real_decorator\ndef f():\n    pass\n",
    )
    .unwrap();
    let (diags, _info) = pysource::check_file(&path);
    assert!(diags.iter().any(|d| d.message.contains("not_a_real_module")));
    assert!(diags.iter().any(|d| d.message.contains("not_a_real_decorator")));
}

#[test]
fn pysource_catches_real_syntax_errors() {
    let dir = unique_tmp_dir("pysource_syntax");
    let path = dir.join("broken.py");
    fs::write(&path, "def f(:\n    pass\n").unwrap();
    let (diags, _info) = pysource::check_file(&path);
    assert!(!diags.is_empty());
}

fn _use_path(_p: &Path) {}
