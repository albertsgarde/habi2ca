use std::process::Command;

use anyhow::{Context, Result};
use habi2ca_ops as ops;

pub fn main() -> Result<()> {
    let workspace_dir = ops::workspace_dir()?;
    ops::trunk_build(workspace_dir.join("habi2ca-frontend"))?;
    Command::new(env!("CARGO"))
        .args(["run", "--bin", "habi2ca-server"])
        .current_dir(workspace_dir.as_path())
        .spawn()
        .with_context(|| {
            format!("Failed to start habi2ca-server with workspace path {workspace_dir:?}.")
        })?
        .wait()?;
    Ok(())
}
