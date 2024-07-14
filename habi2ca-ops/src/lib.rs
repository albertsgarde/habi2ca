use anyhow::{Context, Result};
use std::{path::{Path, PathBuf}, process::Command};

pub fn workspace_dir() -> Result<PathBuf> {
    let output = Command::new(env!("CARGO"))
        .arg("locate-project")
        .arg("--workspace")
        .arg("--message-format=plain")
        .output()
        .context("Failed to run cargo locate-project command")?
        .stdout;
    let cargo_path = Path::new(std::str::from_utf8(&output).with_context(|| 
        format!("Failed to parse 'cargo locate-project' output as valid utf8. Output: {output:?}")
    )?.trim());
    let workspace_dir = cargo_path.parent().expect("Cargo.toml is a file and should therefore always have a parent.");
    Ok(workspace_dir.to_path_buf())
}

pub fn trunk_build(frontend_dir: impl AsRef<Path>, release: bool) -> Result<()> {
    let mut command = Command::new("trunk");
    command
        .arg("build")
        .current_dir(frontend_dir.as_ref());
    if release {
        command.arg("--release");
    }
    command.status()
        .context("Failed to run trunk build command")?;
    Ok(())
}