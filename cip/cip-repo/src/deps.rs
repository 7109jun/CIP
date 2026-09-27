use std::path::Path;
use std::process::Command;

/// A dependency as it ends up usable by the browser side: either a locally
/// bundled wheel file (relative path under the build dir, resolved through
/// chrome.runtime.getURL at runtime -- fully compliant with the "no
/// remotely hosted code" policy), or, if vendoring failed, the bare pip
/// spec as a last-resort fallback that micropip will try to resolve over
/// the network at runtime (this should be treated as a build warning, not
/// a silent success).
#[derive(Debug, Clone)]
pub struct VendoredDep {
    pub spec: String,
    pub local_path: Option<String>,
}

/// Downloads every requirement into `packages_dir` as an actual wheel file,
/// at *build time* (on the developer's machine, with real pip + network
/// access) so the shipped extension never fetches code at runtime.
///
/// Strategy per package:
///   1. Try the Pyodide-tagged wasm wheel first (works for packages that
///      publish one, e.g. numpy/pandas/etc on recent PyPI releases) -- this
///      is what lets C-extension packages work at all under Pyodide.
///   2. Fall back to a universal pure-python wheel (py3-none-any).
///   3. If both fail, the dependency is reported back as unvendored so the
///      caller can surface a clear warning instead of silently shipping a
///      package that will try to hit the network from inside the
///      extension.
pub fn vendor_dependencies(specs: &[String], packages_dir: &Path) -> Vec<VendoredDep> {
    if specs.is_empty() {
        return vec![];
    }
    std::fs::create_dir_all(packages_dir).ok();

    specs
        .iter()
        .map(|spec| {
            let before: Vec<_> = list_wheels(packages_dir);

            let pyodide_ok = Command::new("pip")
                .args([
                    "download",
                    "--no-deps",
                    "--only-binary=:all:",
                    "--platform",
                    "pyodide_2024_0_wasm32",
                    "--python-version",
                    "312",
                    "--implementation",
                    "cp",
                    "-d",
                ])
                .arg(packages_dir)
                .arg(spec)
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false);

            let ok = pyodide_ok
                || Command::new("pip")
                    .args(["download", "--no-deps", "-d"])
                    .arg(packages_dir)
                    .arg(spec)
                    .output()
                    .map(|o| o.status.success())
                    .unwrap_or(false);

            let after = list_wheels(packages_dir);
            let new_file = after.iter().find(|f| !before.contains(f)).cloned();

            VendoredDep {
                spec: spec.clone(),
                local_path: if ok { new_file.map(|f| format!("cip/packages/{}", f)) } else { None },
            }
        })
        .collect()
}

fn list_wheels(dir: &Path) -> Vec<String> {
    std::fs::read_dir(dir)
        .map(|rd| {
            rd.filter_map(|e| e.ok())
                .filter_map(|e| e.file_name().into_string().ok())
                .filter(|n| n.ends_with(".whl"))
                .collect()
        })
        .unwrap_or_default()
}
