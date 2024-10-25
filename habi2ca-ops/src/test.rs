use anyhow::{bail, Context, Result};
use clap::Args;
use std::{
    path::{Path, PathBuf},
    process::Command,
};

#[derive(Args, Debug, Clone)]
pub struct Test {
    /// Whether to fix issues
    #[arg(short, long, default_value = "false")]
    fix: bool,
    #[arg(short, long, default_value = "false")]
    quick: bool,
    /// Input paths
    paths: Vec<String>,
}

impl Test {
    pub fn run(&self, workspace_dir: impl AsRef<Path>) -> Result<()> {
        let workspace_dir = workspace_dir.as_ref();

        let paths = self.paths.iter().map(PathBuf::from).collect::<Vec<_>>();
        let frontend_paths: Vec<_> = paths
            .iter()
            .flat_map(|path| path.strip_prefix("habi2ca-frontend"))
            .collect();

        let rust_paths: Vec<_> = paths
            .iter()
            .filter(|path| path.extension().map_or(false, |ext| ext == "rs"))
            .collect();

        let skip_backend = !paths.is_empty() && frontend_paths.is_empty();

        let mut command = Command::new(env!("CARGO"));
        if self.fix {
            command.args([
                "clippy",
                "--no-deps",
                "--fix",
                "--allow-staged",
                "--",
                "-Dwarnings",
            ]);
        } else {
            command.args(["clippy", "--no-deps", "--", "-Dwarnings"]);
        }
        let status = command
            .current_dir(workspace_dir)
            .spawn()
            .context("Failed to run clippy.")?
            .wait()?;
        if !status.success() {
            bail!("Clippy found warnings.");
        }

        if !rust_paths.is_empty() {
            let mut command = Command::new(env!("CARGO"));
            if self.fix {
                command.args(["fmt"]);
            } else {
                command.args(["fmt", "--check"]);
            }
            command.args(["--"]).args(rust_paths.as_slice());
            let status = command
                .current_dir(workspace_dir)
                .spawn()
                .context("Failed to check formatting.")?
                .wait()?;
            if !status.success() {
                bail!("Formatting incorrect.");
            }
        }

        let mut command = Command::new(env!("CARGO"));
        command.args(["nextest", "run"]).current_dir(workspace_dir);
        let status = command.spawn().context("Failed to run tests.")?.wait()?;
        if !status.success() {
            bail!("Some tests failed.");
        }

        // "check-all": "npm run check && npm run lint && npm run format && npm run test"

        if !skip_backend {
            let mut command = Command::new("npm");
            let status = command
                .args(["run", "check"])
                .current_dir(workspace_dir.join("habi2ca-frontend"))
                .spawn()?
                .wait()?;
            if !status.success() {
                bail!("Frontend check failed.");
            }

            let mut command = Command::new("npm");
            let status = command
                .args(["run", "lint"])
                .current_dir(workspace_dir.join("habi2ca-frontend"))
                .spawn()?
                .wait()?;
            if !status.success() {
                bail!("Frontend lint failed.");
            }

            let mut command = Command::new("npx");
            if self.fix {
                command.args(["prettier", "--write"]);
            } else {
                command.args(["prettier", "--check"]);
            }
            if frontend_paths.is_empty() {
                command.arg(".");
            } else {
                command.args(frontend_paths.as_slice());
            }
            let status = command
                .current_dir(workspace_dir.join("habi2ca-frontend"))
                .spawn()?
                .wait()?;
            if !status.success() {
                bail!("Frontend formatting incorrect.");
            }

            if !self.quick {
                let mut command = Command::new("npm");
                let status = command
                    .args(["run", "test"])
                    .current_dir(workspace_dir.join("habi2ca-frontend"))
                    .spawn()?
                    .wait()?;
                if !status.success() {
                    bail!("Frontend tests failed.");
                }
            }
        }

        Ok(())
    }
}
