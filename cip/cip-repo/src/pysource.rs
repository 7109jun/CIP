use std::path::Path;
use std::process::Command;

use crate::docs_toml::CipDiagnostic;

/// Decorators CIP's runtime bridge understands. Anything else used as
/// `@chrome.xxx` is a validation error (spec section 11: "잘못된 CIP decorator").
pub const KNOWN_DECORATORS: &[&str] = &[
    "startup",
    "command",
    "on_tab_created",
    "on_tab_updated",
    "on_tab_removed",
    "page_load",
    "on_message",
    "on_alarm",
    "on_installed",
    "persistent_state",
];

/// Chrome API surface areas CIP ships a Python module for. Used to flag
/// `from cip import <unsupported>` (spec section 11: "지원하지 않는 Chrome API").
pub const KNOWN_CIP_MODULES: &[&str] = &[
    "chrome",
    "tabs",
    "windows",
    "storage",
    "runtime",
    "cookies",
    "scripting",
    "notifications",
    "commands",
    "alarms",
    "web_navigation",
];

#[derive(Debug, Default, Clone)]
pub struct PyFileInfo {
    /// Names passed to @chrome.command("name")
    pub commands: Vec<String>,
    pub uses_startup: bool,
    pub uses_page_load: bool,
    pub uses_on_installed: bool,
    pub uses_on_alarm: bool,
    pub uses_on_message: bool,
}

/// Runs `python3 -m py_compile` on a file to catch real Python syntax
/// errors, and does a light regex scan for decorator/import misuse. This is
/// intentionally not a full AST parser (CIP ships in Rust and does not
/// bundle a Python parser); the syntax check is delegated to the system
/// Python, which is authoritative anyway.
pub fn check_file(path: &Path) -> (Vec<CipDiagnostic>, PyFileInfo) {
    let mut diags = Vec::new();
    let mut info = PyFileInfo::default();

    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => {
            diags.push(CipDiagnostic {
                file: path.display().to_string(),
                location: None,
                message: "파일을 읽을 수 없습니다.".into(),
                hint: None,
            });
            return (diags, info);
        }
    };

    // 1. Real syntax check via system Python.
    if let Ok(output) = Command::new("python3")
        .args(["-m", "py_compile", &path.display().to_string()])
        .output()
    {
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            diags.push(CipDiagnostic {
                file: path.display().to_string(),
                location: None,
                message: format!("Python 문법 오류:\n{}", stderr.trim()),
                hint: Some("python3 -m py_compile 로 직접 확인해보세요.".into()),
            });
        }
    }

    // 2. import cip.<module> / from cip import <module> validation.
    let import_re = regex::Regex::new(r"(?m)^[ \t]*from\s+cip\s+import\s+([A-Za-z0-9_, \t]+)$").unwrap();
    for cap in import_re.captures_iter(&content) {
        for name in cap[1].split(',') {
            let name = name.trim();
            if !name.is_empty() && !KNOWN_CIP_MODULES.contains(&name) {
                diags.push(CipDiagnostic {
                    file: path.display().to_string(),
                    location: None,
                    message: format!("지원하지 않는 Chrome API 모듈: \"cip.{}\"", name),
                    hint: Some(format!("지원 모듈: {}", KNOWN_CIP_MODULES.join(", "))),
                });
            }
        }
    }

    // 3. @chrome.<decorator> validation + command name extraction.
    let deco_re = regex::Regex::new(r#"(?m)^\s*@chrome\.([A-Za-z_]+)(?:\(\s*"([^"]*)"\s*\))?"#).unwrap();
    for cap in deco_re.captures_iter(&content) {
        let deco = &cap[1];
        if !KNOWN_DECORATORS.contains(&deco) {
            diags.push(CipDiagnostic {
                file: path.display().to_string(),
                location: None,
                message: format!("잘못된 CIP decorator: \"@chrome.{}\"", deco),
                hint: Some(format!("지원되는 decorator: {}", KNOWN_DECORATORS.join(", "))),
            });
            continue;
        }
        match deco {
            "command" => {
                if let Some(m) = cap.get(2) {
                    info.commands.push(m.as_str().to_string());
                } else {
                    diags.push(CipDiagnostic {
                        file: path.display().to_string(),
                        location: None,
                        message: "@chrome.command 는 이름 인자가 필요합니다.".into(),
                        hint: Some("예: @chrome.command(\"hello\")".into()),
                    });
                }
            }
            "startup" => info.uses_startup = true,
            "page_load" => info.uses_page_load = true,
            "on_installed" => info.uses_on_installed = true,
            "on_alarm" => info.uses_on_alarm = true,
            "on_message" => info.uses_on_message = true,
            _ => {}
        }
    }

    (diags, info)
}
