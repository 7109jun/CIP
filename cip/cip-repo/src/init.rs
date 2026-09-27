use std::fs;
use std::path::Path;

pub fn run(name: &str) -> anyhow::Result<()> {
    let dir = Path::new(name);
    if dir.exists() {
        anyhow::bail!("\"{}\" 디렉터리가 이미 존재합니다.", name);
    }
    fs::create_dir_all(dir)?;

    fs::write(
        dir.join("docs.toml"),
        format!(
            r#"name = "{title}"
version = "1.0.0"
description = "A CIP extension"

[permissions]
webpages = []

[permissions.chrome]
storage = true

[extension]
entry = "main.py"
"#,
            title = titlecase(name)
        ),
    )?;

    fs::write(dir.join("requirements.txt"), "")?;

    fs::write(
        dir.join("main.py"),
        r#"from cip import chrome


@chrome.startup
def startup():
    print("Hello CIP")
"#,
    )?;

    println!("생성됨:");
    println!("{}/", name);
    println!("├── docs.toml");
    println!("├── requirements.txt");
    println!("└── main.py");
    println!();
    println!("다음 단계:");
    println!("  cd {}", name);
    println!("  cip build");

    Ok(())
}

fn titlecase(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
        None => s.to_string(),
    }
}
