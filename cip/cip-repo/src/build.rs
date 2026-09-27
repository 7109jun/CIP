use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use crate::docs_toml::{self, CipDiagnostic, DocsToml};
use crate::manifest::{self, ManifestInputs};
use crate::pysource::{self, PyFileInfo};

// Embedded at compile time so the cip binary is fully self-contained (no
// separate "templates/" directory needs to ship alongside it).
const TPL_RUNTIME_JS: &str = include_str!("../templates/runtime.js");
const TPL_BACKGROUND_LOADER: &str = include_str!("../templates/background_loader.js");
const TPL_CONTENT_LOADER: &str = include_str!("../templates/content_loader.js");
const TPL_POPUP_LOADER: &str = include_str!("../templates/popup_loader.js");
const TPL_OPTIONS_LOADER: &str = include_str!("../templates/options_loader.js");
const TPL_POPUP_HTML: &str = include_str!("../templates/popup.html");
const TPL_OPTIONS_HTML: &str = include_str!("../templates/options.html");

// The `cip` Python package itself, vendored into every build so
// `from cip import chrome` works inside Pyodide with no extra install step.
const CIP_PY_INIT: &str = include_str!("../cip-python/cip/__init__.py");
const CIP_PY_BRIDGE: &str = include_str!("../cip-python/cip/_bridge.py");
const CIP_PY_CHROME: &str = include_str!("../cip-python/cip/chrome.py");
const CIP_PY_TABS: &str = include_str!("../cip-python/cip/tabs.py");
const CIP_PY_WINDOWS: &str = include_str!("../cip-python/cip/windows.py");
const CIP_PY_STORAGE: &str = include_str!("../cip-python/cip/storage.py");
const CIP_PY_RUNTIME: &str = include_str!("../cip-python/cip/runtime.py");
const CIP_PY_COOKIES: &str = include_str!("../cip-python/cip/cookies.py");
const CIP_PY_SCRIPTING: &str = include_str!("../cip-python/cip/scripting.py");
const CIP_PY_NOTIFICATIONS: &str = include_str!("../cip-python/cip/notifications.py");
const CIP_PY_COMMANDS: &str = include_str!("../cip-python/cip/commands.py");
const CIP_PY_ALARMS: &str = include_str!("../cip-python/cip/alarms.py");
const CIP_PY_WEBNAV: &str = include_str!("../cip-python/cip/web_navigation.py");

pub struct BuildOutput {
    pub build_dir: PathBuf,
    pub diagnostics: Vec<CipDiagnostic>,
    pub warnings: Vec<String>,
}

/// Full pipeline: validate -> scan python sources -> generate manifest.json
/// -> emit JS loaders -> vendor the cip package + project sources.
/// Returns Err(diagnostics) if validation fails (nothing is written in that
/// case, matching `cip check`'s semantics of "report everything, change
/// nothing").
/// Runs every static check (docs.toml + python source) without writing
/// anything -- what `cip check` uses directly, and what `cip build`
/// runs first before touching disk.
pub fn collect_diagnostics(
    project_dir: &Path,
    docs_toml_path: &Path,
    doc: &DocsToml,
) -> (Vec<CipDiagnostic>, BTreeMap<String, PyFileInfo>) {
    let mut diags = docs_toml::validate(doc, project_dir, docs_toml_path);

    let mut file_info: BTreeMap<String, PyFileInfo> = BTreeMap::new();
    for (role, rel) in [
        ("entry", Some(doc.extension.entry.clone())),
        ("popup", doc.extension.popup.clone()),
        ("content", doc.extension.content.clone()),
        ("background", doc.extension.background.clone()),
        ("options", doc.extension.options.clone()),
    ] {
        if let Some(rel) = rel {
            let full = project_dir.join(&rel);
            if full.exists() {
                let (mut d, info) = pysource::check_file(&full);
                diags.append(&mut d);
                file_info.insert(role.to_string(), info);
            }
        }
    }
    (diags, file_info)
}

pub fn run_build(
    project_dir: &Path,
    docs_toml_path: &Path,
    doc: &DocsToml,
) -> Result<BuildOutput, Vec<CipDiagnostic>> {
    let (diags, file_info) = collect_diagnostics(project_dir, docs_toml_path, doc);

    if !diags.is_empty() {
        return Err(diags);
    }

    let mut build_warnings: Vec<String> = Vec::new();

    let build_dir = project_dir.join("build");
    if build_dir.exists() {
        fs::remove_dir_all(&build_dir).ok();
    }
    fs::create_dir_all(&build_dir).unwrap();
    fs::create_dir_all(build_dir.join("cip")).unwrap();
    fs::create_dir_all(build_dir.join("cip_pkg/cip")).unwrap();

    // 1. manifest.json
    let inputs = ManifestInputs {
        doc,
        background_info: file_info.get("background"),
        content_info: file_info.get("content"),
        popup_info: file_info.get("popup"),
    };
    let manifest_value = manifest::build_manifest(&inputs);
    fs::write(
        build_dir.join("manifest.json"),
        serde_json::to_string_pretty(&manifest_value).unwrap(),
    )
    .unwrap();

    // 2. shared runtime.js
    fs::write(build_dir.join("cip/runtime.js"), TPL_RUNTIME_JS).unwrap();
    let vendored_pyodide = crate::fetch_runtime::vendor_into_build(
        project_dir,
        &build_dir.join("cip"),
        crate::fetch_runtime::DEFAULT_PYODIDE_VERSION,
    );
    if !vendored_pyodide {
        fs::write(
            build_dir.join("cip/README_PYODIDE.txt"),
            "Pyodide가 아직 vendoring되지 않았습니다.\n\n\
             `cip fetch-runtime` 을 실행해 공식 Pyodide 릴리즈를 로컬에 받아온 뒤\n\
             다시 `cip build` 를 실행하면 이 디렉터리에 pyodide.js / .wasm 등이 채워집니다.\n\
             Chrome/Edge 정책상 확장 프로그램은 원격 코드를 실행할 수 없으므로,\n\
             Pyodide는 반드시 로컬에 번들되어야 합니다.\n",
        )
        .unwrap();
        build_warnings.push(
            "Pyodide 런타임이 아직 vendoring되지 않았습니다 -- 이 빌드는 chrome://extensions 에 로드해도 실제로 동작하지 않습니다. `cip fetch-runtime` 실행 후 다시 build 하세요.".to_string()
        );
    }

    // 3. vendor the cip python package
    let cip_pkg_dir = build_dir.join("cip_pkg/cip");
    for (name, content) in [
        ("__init__.py", CIP_PY_INIT),
        ("_bridge.py", CIP_PY_BRIDGE),
        ("chrome.py", CIP_PY_CHROME),
        ("tabs.py", CIP_PY_TABS),
        ("windows.py", CIP_PY_WINDOWS),
        ("storage.py", CIP_PY_STORAGE),
        ("runtime.py", CIP_PY_RUNTIME),
        ("cookies.py", CIP_PY_COOKIES),
        ("scripting.py", CIP_PY_SCRIPTING),
        ("notifications.py", CIP_PY_NOTIFICATIONS),
        ("commands.py", CIP_PY_COMMANDS),
        ("alarms.py", CIP_PY_ALARMS),
        ("web_navigation.py", CIP_PY_WEBNAV),
    ] {
        fs::write(cip_pkg_dir.join(name), content).unwrap();
    }

    // 4. copy every top-level *.py file in the project (spec: file count /
    // names aren't fixed) into the project source tree that gets mounted.
    let src_dir = build_dir.join("cip_pkg/project");
    fs::create_dir_all(&src_dir).unwrap();
    let mut project_py_files = Vec::new();
    for entry in fs::read_dir(project_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) == Some("py") {
            let name = path.file_name().unwrap().to_str().unwrap().to_string();
            fs::copy(&path, src_dir.join(&name)).unwrap();
            project_py_files.push(format!("cip_pkg/project/{}", name));
        }
    }
    let mut cip_pkg_files: Vec<String> = vec![
        "cip_pkg/cip/__init__.py",
        "cip_pkg/cip/_bridge.py",
        "cip_pkg/cip/chrome.py",
        "cip_pkg/cip/tabs.py",
        "cip_pkg/cip/windows.py",
        "cip_pkg/cip/storage.py",
        "cip_pkg/cip/runtime.py",
        "cip_pkg/cip/cookies.py",
        "cip_pkg/cip/scripting.py",
        "cip_pkg/cip/notifications.py",
        "cip_pkg/cip/commands.py",
        "cip_pkg/cip/alarms.py",
        "cip_pkg/cip/web_navigation.py",
    ]
    .into_iter()
    .map(String::from)
    .collect();
    cip_pkg_files.extend(project_py_files);
    // sys.path needs cip_pkg/ (for `import cip`) and cip_pkg/project (for
    // `import background`, `import content`, etc, since those files use
    // module-style names, not package-qualified ones). runtime.js's cipInit
    // adds both directories to sys.path once the files below are mounted.
    let all_py_files = cip_pkg_files.clone();

    let specs = read_requirements(project_dir);
    let packages_dir = build_dir.join("cip/packages");
    let vendored = crate::deps::vendor_dependencies(&specs, &packages_dir);
    let dep_targets: Vec<String> = vendored
        .iter()
        .map(|v| match &v.local_path {
            Some(p) => p.clone(),
            None => {
                build_warnings.push(format!(
                    "의존성 \"{}\" 을(를) 빌드 시점에 로컬 wheel로 받아오지 못했습니다. \
                     런타임에 micropip이 네트워크에서 직접 설치를 시도하며, \
                     이는 Chrome Web Store/Edge Add-ons의 \"원격 코드 실행 금지\" 정책에 \
                     위배될 수 있습니다.",
                    v.spec
                ));
                v.spec.clone()
            }
        })
        .collect();
    let deps_json = serde_json::to_string(&dep_targets).unwrap();
    let files_json = serde_json::to_string(&all_py_files).unwrap();

    let entry_module = |rel: &str| -> String {
        Path::new(rel)
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
    };

    // 5. background.js
    if let Some(rel) = &doc.extension.background {
        let module = entry_module(rel);
        let loader = TPL_BACKGROUND_LOADER
            .replace("__CIP_PY_FILES__", &files_json)
            .replace("__CIP_DEPENDENCIES__", &deps_json)
            .replace("\"background\"", &format!("\"{}\"", module));
        fs::write(build_dir.join("background.js"), loader).unwrap();
    }

    // 6. content.js
    if let Some(rel) = &doc.extension.content {
        let module = entry_module(rel);
        let loader = TPL_CONTENT_LOADER
            .replace("__CIP_PY_FILES__", &files_json)
            .replace("__CIP_DEPENDENCIES__", &deps_json)
            .replace("\"content\"", &format!("\"{}\"", module));
        fs::write(build_dir.join("content.js"), loader).unwrap();
    }

    // 7. popup.html + popup.js
    if let Some(rel) = &doc.extension.popup {
        let module = entry_module(rel);
        let loader = TPL_POPUP_LOADER
            .replace("__CIP_PY_FILES__", &files_json)
            .replace("__CIP_DEPENDENCIES__", &deps_json)
            .replace("\"popup\"", &format!("\"{}\"", module));
        fs::write(build_dir.join("popup.js"), loader).unwrap();
        fs::write(build_dir.join("popup.html"), TPL_POPUP_HTML).unwrap();
    }

    // 8. options.html + options.js
    if let Some(rel) = &doc.extension.options {
        let module = entry_module(rel);
        let loader = TPL_OPTIONS_LOADER
            .replace("__CIP_PY_FILES__", &files_json)
            .replace("__CIP_DEPENDENCIES__", &deps_json)
            .replace("\"options\"", &format!("\"{}\"", module));
        fs::write(build_dir.join("options.js"), loader).unwrap();
        fs::write(build_dir.join("options.html"), TPL_OPTIONS_HTML).unwrap();
    }

    Ok(BuildOutput {
        build_dir,
        diagnostics: vec![],
        warnings: build_warnings,
    })
}

fn read_requirements(project_dir: &Path) -> Vec<String> {
    let path = project_dir.join("requirements.txt");
    let mut deps = Vec::new();
    if let Ok(content) = fs::read_to_string(path) {
        for line in content.lines() {
            let l = line.trim();
            if !l.is_empty() && !l.starts_with('#') {
                deps.push(l.to_string());
            }
        }
    }
    deps
}
