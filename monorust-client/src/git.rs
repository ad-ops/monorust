use anyhow::Result;
use std::path::PathBuf;
use std::process::Command;
use std::str;

use crate::server::checkout_code;

pub fn checkout(target_dir: &PathBuf, module_name: &str, dry_run: bool) -> Result<String> {
    let git_org = "https://github.com/ad-ops";
    let repo = "monorust";

    let checkout_result = checkout_code("test_user", module_name, "fake_env");
    println!("Checked out code response: {checkout_result:?}");

    let args = vec![
        format!("git clone --filter=blob:none --no-checkout {git_org}/{repo}"),
        format!("git -C {repo} sparse-checkout init --no-cone"),
        format!("git -C {repo} sparse-checkout set monorepo-example/{module_name}"),
        format!("git -C {repo} checkout"),
    ];
    let clone_arg = args.join(" && ");
    println!("{clone_arg}");

    let mut command = Command::new("sh");
    let command = command.current_dir(target_dir).arg("-c").arg(clone_arg);

    let text = if dry_run {
        format!("cd {:?}\n{}", target_dir, &args.join("\n"))
    } else {
        if !target_dir.exists() {
            return Ok(format!("target dir ({target_dir:?} does not exist!)"));
        }
        if target_dir.join(repo).exists() {
            return Ok(format!("already exists dir ({repo})!"));
        }

        let output = command.output()?;

        let text = output.stdout;
        str::from_utf8(&text)?.to_string()
    };
    Ok(text)
}
