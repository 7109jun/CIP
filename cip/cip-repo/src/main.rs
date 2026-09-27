mod build;
mod deps;
mod docs_toml;
mod errors;
mod fetch_runtime;
mod init;
mod manifest;
mod pip_cmd;
mod pysource;

use std::path::PathBuf;
use std::process::Command;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let cmd = args.first().map(|s| s.as_str()).unwrap_or("help");
    let rest = &args[1.min(args.len())..];

    let result = match cmd {
        "init" => cmd_init(rest),
        "install" => cmd_install(),
        "pip" => cmd_pip(rest),
        "run" => cmd_run(),
        "build" => cmd_build(),
        "package" => cmd_package(),
        "check" => cmd_check(),
        "clean" => cmd_clean(),
        "fetch-runtime" => cmd_fetch_runtime(rest),
        "version" | "--version" | "-v" => {
            println!("cip {}", VERSION);
            Ok(())
        }
        "help" | "--help" | "-h" | _ => {
            print_help();
            Ok(())
        }
    };

    if let Err(e) = result {
        eprintln!("cip: {}", e);
        std::process::exit(1);
    }
}

fn print_help() {
    println!(
        r#"CIP (Chrome in Python) — Python으로 Chrome/Edge 확장 만들기

사용법:
  cip init <name>        새 프로젝트 생성
  cip install            requirements.txt 의존성 설치 (로컬 dev용)
  cip pip <args...>      pip 명령 그대로 전달
  cip check              docs.toml + Python 소스 검증만 수행
  cip build              build/ 디렉터리에 실제 확장 프로그램 생성
  cip run                검증 + 빌드 후 Load unpacked 안내 출력
  cip package            build/ 를 zip으로 패키징 (스토어 업로드용)
  cip fetch-runtime      Pyodide 런타임을 로컬에 받아옴 (build 전 1회)
  cip clean              build/ 산출물 삭제
  cip version            버전 출력
  cip help               도움말
"#
    );
}

fn find_project() -> anyhow::Result<(PathBuf, docs_toml::DocsToml)> {
    let cwd = std::env::current_dir()?;
    match docs_toml::discover_project(&cwd) {
        Ok(path) => match docs_toml::load(&path) {
            Ok(doc) => Ok((path, doc)),
            Err(diag) => {
                eprint!("{}", diag.render());
                std::process::exit(1);
            }
        },
        Err(diag) => {
            eprint!("{}", diag.render());
            std::process::exit(1);
        }
    }
}

fn cmd_init(rest: &[String]) -> anyhow::Result<()> {
    let name = rest
        .first()
        .ok_or_else(|| anyhow::anyhow!("사용법: cip init <name>"))?;
    init::run(name)?;
    Ok(())
}

fn cmd_check() -> anyhow::Result<()> {
    let (docs_toml_path, doc) = find_project()?;
    let project_dir = docs_toml_path.parent().unwrap().to_path_buf();
    let (diags, file_info) = build::collect_diagnostics(&project_dir, &docs_toml_path, &doc);
    if diags.is_empty() {
        let commands: Vec<String> = file_info
            .values()
            .flat_map(|i| i.commands.clone())
            .collect();
        println!("✓ {} — 문제없음", doc.name);
        if !commands.is_empty() {
            println!("  commands: {}", commands.join(", "));
        }
        Ok(())
    } else {
        for d in &diags {
            eprintln!("{}", d.render());
        }
        anyhow::bail!("{}개의 문제를 찾았습니다.", diags.len());
    }
}

fn cmd_build() -> anyhow::Result<()> {
    let (docs_toml_path, doc) = find_project()?;
    let project_dir = docs_toml_path.parent().unwrap().to_path_buf();
    match build::run_build(&project_dir, &docs_toml_path, &doc) {
        Ok(out) => {
            println!("✓ 빌드 완료: {}", out.build_dir.display());
            for w in &out.warnings {
                println!("⚠ {}", w);
            }
            println!();
            println!("chrome://extensions → 개발자 모드 → \"압축해제된 확장 프로그램을 로드합니다\"");
            println!("  {}", out.build_dir.display());
            Ok(())
        }
        Err(diags) => {
            for d in &diags {
                eprintln!("{}", d.render());
            }
            anyhow::bail!("빌드 실패: {}개의 문제를 해결하세요.", diags.len());
        }
    }
}

fn cmd_run() -> anyhow::Result<()> {
    cmd_build()
}

fn cmd_package() -> anyhow::Result<()> {
    let (docs_toml_path, doc) = find_project()?;
    let project_dir = docs_toml_path.parent().unwrap().to_path_buf();
    let build_dir = project_dir.join("build");
    if !build_dir.join("manifest.json").exists() {
        println!("build/ 가 없어 먼저 빌드합니다...");
        cmd_build()?;
    }
    let zip_name = format!("{}-{}.zip", slug(&doc.name), doc.version);
    let zip_path = project_dir.join(&zip_name);
    if zip_path.exists() {
        std::fs::remove_file(&zip_path)?;
    }
    let status = Command::new("zip")
        .arg("-r")
        .arg(project_dir.join(&zip_name))
        .arg(".")
        .current_dir(&build_dir)
        .status()?;
    if !status.success() {
        anyhow::bail!("zip 패키징 실패");
    }
    println!("✓ 패키징 완료: {}", zip_path.display());
    Ok(())
}

fn cmd_clean() -> anyhow::Result<()> {
    let (docs_toml_path, _doc) = find_project()?;
    let project_dir = docs_toml_path.parent().unwrap().to_path_buf();
    let build_dir = project_dir.join("build");
    if build_dir.exists() {
        std::fs::remove_dir_all(&build_dir)?;
        println!("삭제됨: {}", build_dir.display());
    } else {
        println!("build/ 없음 (이미 clean 상태)");
    }
    Ok(())
}

fn cmd_install() -> anyhow::Result<()> {
    let (docs_toml_path, _doc) = find_project()?;
    let project_dir = docs_toml_path.parent().unwrap().to_path_buf();
    pip_cmd::install(&project_dir)
}

fn cmd_pip(rest: &[String]) -> anyhow::Result<()> {
    pip_cmd::pip_passthrough(rest)
}

fn cmd_fetch_runtime(rest: &[String]) -> anyhow::Result<()> {
    let (docs_toml_path, _doc) = find_project()?;
    let project_dir = docs_toml_path.parent().unwrap().to_path_buf();
    let version = rest
        .first()
        .cloned()
        .unwrap_or_else(|| fetch_runtime::DEFAULT_PYODIDE_VERSION.to_string());
    fetch_runtime::run(&project_dir, &version)
}

fn slug(s: &str) -> String {
    s.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
}
