use std::{
    fs::{self, File},
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use env_logger::Target;

fn log_file_path<P>(dir: P, index: u32) -> PathBuf
where
    P: AsRef<Path>,
{
    let mut result = PathBuf::new();
    let dir = dir.as_ref();
    result.push(dir.join(format!("{index}.log")));
    result
}

fn ensure_log_dir<P>(dir: P) -> Result<()>
where
    P: AsRef<Path>,
{
    let dir = dir.as_ref();
    fs::create_dir_all(log_file_path(dir, 0).as_path().parent().unwrap())
        .with_context(|| format!("Could not create log dir '{dir:?}'."))
}

fn log_index<P>(dir: P) -> Result<u32>
where
    P: AsRef<Path>,
{
    Ok((0..)
        .find(|&log_index| {
            let file_path = log_file_path(dir.as_ref(), log_index);
            match file_path.as_path().metadata() {
                Ok(_) => false,
                Err(error) => match error.kind() {
                    std::io::ErrorKind::NotFound => true,
                    _ => panic!("Could not create log file. Error: {error:?}"),
                },
            }
        })
        .unwrap())
}

pub fn init_logging() -> Result<()> {
    let log_dir = Path::new("local").join("logging").join("server");
    ensure_log_dir(log_dir.as_path())?;
    let log_index = log_index(&log_dir)
        .with_context(|| format!("Failed to get log index for dir '{log_dir:?}'."))?;
    let log_file_path = log_file_path(log_dir, log_index);

    let log_file_target = Box::new(
        File::create(log_file_path.as_path())
            .with_context(|| format!("Could not create log file '{log_file_path:?}'."))?,
    );
    env_logger::builder()
        .filter_level(log::LevelFilter::Off)
        .target(Target::Pipe(log_file_target))
        .try_init()
        .context("Logger already initialized.")
}
