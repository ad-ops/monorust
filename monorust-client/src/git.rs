use std::path::PathBuf;
use std::process::Command;
use std::str;
use anyhow::Result;

/// - Sanity check
/// - git clone --filter=blob:none --no-checkout https://github.com/ad-ops/monorust
/// - cd monorust
/// - git sparse-checkout init --cone
/// - git sparse-checkout set monorepo-example/module1
pub fn checkout(target_dir: &PathBuf, module_name: &str) -> Result<String> {
    let git_org = "https://github.com/ad-ops";
    let repo = "monorust";
    if !target_dir.exists() {
        return Ok(format!("target dir ({target_dir:?} does not exist!)"));
    } 
    if target_dir.join(repo).exists() {
        return Ok(format!("already exists dir ({repo})!"));
    }

    let args = vec![
        format!("git clone --filter=blob:none --no-checkout {git_org}/{repo}"),
        format!("git -C {repo} sparse-checkout init --no-cone"),
        format!("git -C {repo} sparse-checkout set monorepo-example/{module_name}"),
        format!("git -C {repo} checkout"),
    ];
    let clone_arg = args.join(" && ");
    println!("{clone_arg}");

    let output = Command::new("sh")
        .current_dir(target_dir)
        .arg("-c")
        .arg(clone_arg)
        .output()?;
    

    let hello = output.stdout;
    let hello = str::from_utf8(&hello)?
        .to_string();
    Ok(hello)
}

