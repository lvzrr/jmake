use std::{path::PathBuf};
use crate::native::*;
use crate::packages::*;
use crate::paths::*;
use crate::config::*;

pub fn run(target: &str, conf: &CONFIG) -> Result<(), Box<dyn std::error::Error>>
{
    let class = package_to_path(target, PathType::CLASS, &conf);
    let mut passtorunner: Vec<PathBuf> = Vec::new();
    passtorunner.push(class);
    native_runner(passtorunner, &conf, PathType::CLASS)?;
    Ok(())
}

pub fn run_tests(target: &str, conf: &CONFIG) -> Result<(), Box<dyn std::error::Error>>
{
    let files: Vec<PathBuf> = get_target_files(target, conf, false, PathType::TESTS)
        .expect("Couldn't get target files");
    native_runner(files, &conf, PathType::TESTS)?;
    Ok(())
}
