use std::path::Path;
use std::process::Command;

/// Local dev dependencies (for editor support / `cip check`'s import
/// validation) live in a project-local target dir, never the system
/// site-packages. This is separate from the *browser-side* dependency
/// vendoring in deps.rs, which fetches wheels for micropip at build time.
const LOCAL_SITE: &str = ".cip/pysite";

pub fn install(project_dir: &Path) -> anyhow::Result<()> {
    let req = project_dir.join("requirements.txt");
    if !req.exists() {
        println!("requirements.txt 가 없습니다. 설치할 의존성이 없습니다.");
        return Ok(());
    }
    let content = std::fs::read_to_string(&req)?;
    if content.trim().is_empty() {
        println!("requirements.txt 가 비어 있습니다.");
        return Ok(());
    }

    let target = project_dir.join(LOCAL_SITE);
    std::fs::create_dir_all(&target)?;

    println!("cip: 프로젝트 의존성 설치 중 (로컬 dev용, {})...", target.display());
    let status = Command::new("python3")
        .args(["-m", "pip", "install", "--target"])
        .arg(&target)
        .args(["-r"])
        .arg(&req)
        .status()?;

    if !status.success() {
        anyhow::bail!("pip install 실패 (exit code: {:?})", status.code());
    }
    println!("완료. (참고: 실제 브라우저 런타임의 패키지는 `cip build`가 별도로 micropip용 wheel을 받아옵니다.)");
    Ok(())
}

pub fn pip_passthrough(args: &[String]) -> anyhow::Result<()> {
    let status = Command::new("python3")
        .arg("-m")
        .arg("pip")
        .args(args)
        .status()?;
    if !status.success() {
        anyhow::bail!("pip 명령 실패 (exit code: {:?})", status.code());
    }
    Ok(())
}
