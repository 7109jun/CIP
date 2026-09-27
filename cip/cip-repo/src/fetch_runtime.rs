use std::path::Path;
use std::process::Command;

pub const DEFAULT_PYODIDE_VERSION: &str = "0.26.2";

/// Files actually needed to run vanilla Python + micropip in the browser.
/// The full Pyodide release also ships prebuilt wheels for numpy/pandas/
/// etc under packages/ -- those aren't needed unless a project's
/// requirements.txt asks for them, and `cip build`'s dependency vendoring
/// (deps.rs) fetches specific wheels on demand instead of bundling the
/// entire package repository.
const CORE_MEMBERS: &[&str] = &[
    "pyodide/pyodide.js",
    "pyodide/pyodide.mjs",
    "pyodide/pyodide.asm.js",
    "pyodide/pyodide.asm.wasm",
    "pyodide/python_stdlib.zip",
    "pyodide/pyodide-lock.json",
    // Bundled micropip wheel version tracks the Pyodide release above; if
    // you change DEFAULT_PYODIDE_VERSION, check this filename against
    // `tar tjf pyodide-<ver>.tar.bz2 | grep micropip`.
    "pyodide/micropip-0.6.0-py3-none-any.whl",
];

/// Downloads the official Pyodide release tarball from GitHub Releases
/// (never a CDN) into a per-project cache, then extracts just the core
/// runtime files `cip build` needs. This is a build-time-only fetch on the
/// developer's own machine -- the resulting files are what gets vendored
/// into build/cip/pyodide/, so the *shipped extension* never talks to the
/// network to load its own runtime (required by Chrome/Edge store policy).
pub fn run(project_dir: &Path, version: &str) -> anyhow::Result<()> {
    let cache_dir = project_dir.join(".cip/pyodide-vendor").join(version);
    let marker = cache_dir.join("pyodide.js");
    if marker.exists() {
        println!("이미 캐시되어 있습니다: {}", cache_dir.display());
        return Ok(());
    }
    std::fs::create_dir_all(&cache_dir)?;

    let url = format!(
        "https://github.com/pyodide/pyodide/releases/download/{v}/pyodide-{v}.tar.bz2",
        v = version
    );
    let tarball = project_dir
        .join(".cip/pyodide-vendor")
        .join(format!("pyodide-{}.tar.bz2", version));

    println!("Pyodide {} 다운로드 중...\n  {}", version, url);
    let status = Command::new("curl")
        .args(["-L", "-f", "--progress-bar", "-o"])
        .arg(&tarball)
        .arg(&url)
        .status()?;
    if !status.success() {
        anyhow::bail!(
            "Pyodide 릴리즈를 다운로드하지 못했습니다 (버전 \"{}\" 확인 필요): {}",
            version,
            url
        );
    }

    println!("코어 런타임 파일 추출 중...");
    let mut tar_cmd = Command::new("tar");
    tar_cmd
        .arg("xjf")
        .arg(&tarball)
        .arg("-C")
        .arg(project_dir.join(".cip/pyodide-vendor"));
    for member in CORE_MEMBERS {
        tar_cmd.arg(member);
    }
    let status = tar_cmd.status()?;
    if !status.success() {
        anyhow::bail!("tar 추출 실패 (일부 파일이 이 버전에 없을 수 있습니다).");
    }

    // The tarball extracts to .cip/pyodide-vendor/pyodide/*; move it into
    // the version-keyed cache dir so multiple versions can coexist.
    let extracted = project_dir.join(".cip/pyodide-vendor/pyodide");
    if extracted.exists() {
        for entry in std::fs::read_dir(&extracted)? {
            let entry = entry?;
            let dest = cache_dir.join(entry.file_name());
            std::fs::rename(entry.path(), dest)?;
        }
        std::fs::remove_dir_all(&extracted).ok();
    }
    std::fs::remove_file(&tarball).ok();

    println!("완료: {}", cache_dir.display());
    println!("`cip build` 실행 시 자동으로 build/cip/pyodide/ 에 복사됩니다.");
    Ok(())
}

/// Copies a cached, already-fetched runtime into the build output. Returns
/// true if a cached runtime was found and copied.
pub fn vendor_into_build(project_dir: &Path, build_cip_dir: &Path, version: &str) -> bool {
    let cache_dir = project_dir.join(".cip/pyodide-vendor").join(version);
    if !cache_dir.join("pyodide.js").exists() {
        return false;
    }
    let dest = build_cip_dir.join("pyodide");
    std::fs::create_dir_all(&dest).ok();
    let mut copied = false;
    if let Ok(rd) = std::fs::read_dir(&cache_dir) {
        for entry in rd.filter_map(|e| e.ok()) {
            let dest_file = dest.join(entry.file_name());
            if std::fs::copy(entry.path(), &dest_file).is_ok() {
                copied = true;
            }
        }
    }
    copied
}
