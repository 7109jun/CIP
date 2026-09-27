use serde::Deserialize;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// Chrome permissions CIP knows how to translate into manifest.json
/// `permissions` (as opposed to host_permissions).
pub const KNOWN_CHROME_PERMISSIONS: &[&str] = &[
    "tabs",
    "windows",
    "storage",
    "cookies",
    "scripting",
    "notifications",
    "commands",
    "alarms",
    "webNavigation",
    "activeTab",
    "contextMenus",
    "downloads",
    "history",
    "bookmarks",
    "clipboardRead",
    "clipboardWrite",
    "idle",
    "sessions",
    "topSites",
    "webRequest",
];

#[derive(Debug, Deserialize, Clone)]
pub struct DocsToml {
    pub name: String,
    pub version: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub icons: Option<BTreeMap<String, String>>,
    #[serde(default)]
    pub permissions: Permissions,
    pub extension: Extension,
    #[serde(default)]
    pub dependencies: Dependencies,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Permissions {
    #[serde(default)]
    pub webpages: Vec<String>,
    #[serde(default)]
    pub chrome: BTreeMap<String, bool>,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Dependencies {
    /// If true, cip build will try to include WASM-compiled C-extension
    /// packages (via Pyodide's micropip) in addition to pure python ones.
    #[serde(default)]
    pub allow_native: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Extension {
    pub entry: String,
    #[serde(default)]
    pub popup: Option<String>,
    #[serde(default)]
    pub content: Option<String>,
    #[serde(default)]
    pub background: Option<String>,
    #[serde(default)]
    pub options: Option<String>,
}

/// A single validation problem, formatted the way section 14 of the spec
/// requires: file, location, message, hint.
#[derive(Debug, Clone)]
pub struct CipDiagnostic {
    pub file: String,
    pub location: Option<String>,
    pub message: String,
    pub hint: Option<String>,
}

impl CipDiagnostic {
    pub fn render(&self) -> String {
        let mut out = String::new();
        out.push_str("CIP Error\n\n");
        match &self.location {
            Some(loc) => out.push_str(&format!("{}:{}\n", self.file, loc)),
            None => out.push_str(&format!("{}\n", self.file)),
        }
        out.push_str(&format!("{}\n", self.message));
        if let Some(hint) = &self.hint {
            out.push_str(&format!("\nHint:\n{}\n", hint));
        }
        out
    }
}

/// Loads and parses docs.toml from `path`. Returns a structured TOML error
/// (with line/column) rather than a generic parse failure.
pub fn load(path: &Path) -> Result<DocsToml, CipDiagnostic> {
    let content = std::fs::read_to_string(path).map_err(|_| CipDiagnostic {
        file: path.display().to_string(),
        location: None,
        message: "docs.toml을 찾을 수 없습니다.".to_string(),
        hint: Some("cip init <name> 으로 새 프로젝트를 생성하거나, docs.toml이 있는 디렉터리에서 실행하세요.".to_string()),
    })?;

    toml::from_str::<DocsToml>(&content).map_err(|e| {
        let loc = e.span().map(|span| {
            let line = content[..span.start].matches('\n').count() + 1;
            format!("{}", line)
        });
        CipDiagnostic {
            file: path.display().to_string(),
            location: loc,
            message: format!("TOML 파싱 오류: {}", e.message()),
            hint: Some("docs.toml 문법을 확인하세요. (예: 문자열은 따옴표로 감싸야 함)".to_string()),
        }
    })
}

/// Chrome/Edge extension match-pattern validation:
/// <scheme>://<host><path>  where scheme is *, http, https, file, ftp
/// and host may start with a wildcard subdomain like *.example.com,
/// or be a bare "*" for all hosts.
pub fn is_valid_match_pattern(pattern: &str) -> bool {
    if pattern == "<all_urls>" {
        return true;
    }
    let re = regex::Regex::new(
        r"^(\*|http|https|file|ftp)://(\*|(\*\.)?[a-zA-Z0-9\-\.]+|)(/.*)$",
    )
    .unwrap();
    re.is_match(pattern)
}

/// Runs every static check from spec section 11 against an already-parsed
/// DocsToml plus the project directory it lives in. Returns all problems
/// found (not just the first) so the user gets a full report from
/// `cip check`.
pub fn validate(doc: &DocsToml, project_dir: &Path, docs_toml_path: &Path) -> Vec<CipDiagnostic> {
    let mut diags = Vec::new();
    let file = docs_toml_path.display().to_string();

    if doc.name.trim().is_empty() {
        diags.push(CipDiagnostic {
            file: file.clone(),
            location: None,
            message: "name 필드가 비어 있습니다.".into(),
            hint: Some("name = \"My Extension\" 형태로 확장 프로그램 이름을 지정하세요.".into()),
        });
    }

    if doc.version.trim().is_empty() {
        diags.push(CipDiagnostic {
            file: file.clone(),
            location: None,
            message: "version 필드가 비어 있습니다.".into(),
            hint: Some("version = \"1.0.0\" 형태의 semver 문자열을 지정하세요.".into()),
        });
    } else if !regex::Regex::new(r"^\d+\.\d+\.\d+$").unwrap().is_match(&doc.version) {
        diags.push(CipDiagnostic {
            file: file.clone(),
            location: None,
            message: format!("version \"{}\" 은 유효한 semver 형식이 아닙니다.", doc.version),
            hint: Some("예: version = \"1.0.0\"".into()),
        });
    }

    check_py_file_exists(project_dir, &doc.extension.entry, "entry", &file, &mut diags);
    if let Some(p) = &doc.extension.popup {
        check_py_file_exists(project_dir, p, "popup", &file, &mut diags);
    }
    if let Some(p) = &doc.extension.content {
        check_py_file_exists(project_dir, p, "content", &file, &mut diags);
    }
    if let Some(p) = &doc.extension.background {
        check_py_file_exists(project_dir, p, "background", &file, &mut diags);
    }
    if let Some(p) = &doc.extension.options {
        check_py_file_exists(project_dir, p, "options", &file, &mut diags);
    }

    for pattern in &doc.permissions.webpages {
        if !is_valid_match_pattern(pattern) {
            let suggestion = suggest_match_pattern(pattern);
            diags.push(CipDiagnostic {
                file: file.clone(),
                location: Some("permissions.webpages".into()),
                message: format!("Invalid webpage permission:\n\n{}\n\nExpected a Chrome match pattern.", pattern),
                hint: Some(suggestion),
            });
        }
    }

    for key in doc.permissions.chrome.keys() {
        if !KNOWN_CHROME_PERMISSIONS.contains(&key.as_str()) {
            diags.push(CipDiagnostic {
                file: file.clone(),
                location: Some("permissions.chrome".into()),
                message: format!("알 수 없는 chrome permission: \"{}\"", key),
                hint: Some(format!(
                    "지원되는 permission: {}",
                    KNOWN_CHROME_PERMISSIONS.join(", ")
                )),
            });
        }
    }

    let req_path = project_dir.join("requirements.txt");
    if req_path.exists() {
        if let Ok(content) = std::fs::read_to_string(&req_path) {
            let re = regex::Regex::new(r"^[A-Za-z0-9_\-\.]+(\[[A-Za-z0-9_,\-]+\])?\s*((==|>=|<=|~=|!=|>|<)\s*[0-9A-Za-z\.\-]+)?\s*$").unwrap();
            for (i, line) in content.lines().enumerate() {
                let l = line.trim();
                if l.is_empty() || l.starts_with('#') {
                    continue;
                }
                if !re.is_match(l) {
                    diags.push(CipDiagnostic {
                        file: "requirements.txt".into(),
                        location: Some(format!("{}", i + 1)),
                        message: format!("requirements.txt 문법 오류: \"{}\"", l),
                        hint: Some("예: requests>=2.32".into()),
                    });
                }
            }
        }
    }

    // Python-level checks (syntax + decorator + import) live in pysource.rs;
    // callers combine those diagnostics with this list.

    diags
}

fn check_py_file_exists(
    project_dir: &Path,
    rel: &str,
    role: &str,
    file: &str,
    diags: &mut Vec<CipDiagnostic>,
) {
    let full = project_dir.join(rel);
    if !full.exists() {
        diags.push(CipDiagnostic {
            file: file.to_string(),
            location: Some(format!("extension.{}", role)),
            message: format!("존재하지 않는 {} Python 파일: \"{}\"", role, rel),
            hint: Some(format!("{} 파일을 프로젝트 루트에 생성하세요.", rel)),
        });
    } else if full.extension().and_then(|e| e.to_str()) != Some("py") {
        diags.push(CipDiagnostic {
            file: file.to_string(),
            location: Some(format!("extension.{}", role)),
            message: format!("{} 은(는) .py 파일이어야 합니다: \"{}\"", role, rel),
            hint: None,
        });
    }
}

fn suggest_match_pattern(pattern: &str) -> String {
    if let Some(rest) = pattern.strip_prefix("https://") {
        if !rest.contains('/') {
            return format!("https://{}/*", rest);
        }
    }
    if let Some(rest) = pattern.strip_prefix("http://") {
        if !rest.contains('/') {
            return format!("http://{}/*", rest);
        }
    }
    if !pattern.contains("://") {
        return format!("https://{}/*", pattern);
    }
    format!("{}/*", pattern.trim_end_matches('/'))
}

/// Walks up from `start_dir` looking for docs.toml, the way `cip build`
/// (or any other command) is allowed to run from any subdirectory of a
/// project (spec section 12).
pub fn discover_project(start_dir: &Path) -> Result<PathBuf, CipDiagnostic> {
    let mut dir = start_dir.to_path_buf();
    loop {
        let candidate = dir.join("docs.toml");
        if candidate.exists() {
            return Ok(candidate);
        }
        match dir.parent() {
            Some(parent) => dir = parent.to_path_buf(),
            None => {
                return Err(CipDiagnostic {
                    file: start_dir.display().to_string(),
                    location: None,
                    message: "docs.toml을 현재 디렉터리나 상위 디렉터리에서 찾을 수 없습니다.".into(),
                    hint: Some("CIP 프로젝트 디렉터리 안에서 실행하거나 cip init으로 새 프로젝트를 만드세요.".into()),
                })
            }
        }
    }
}
