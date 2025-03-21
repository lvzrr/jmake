use std::{process::Command, path::PathBuf};
use crate::packages::*;
use crate::compile::*;
use crate::paths::*;
use crate::config::*;

pub fn run(target: &str, conf: &CONFIG)
{
    let cmd = format!("java -cp {} {} {} {}", 
                      format!("\"{}\"", &conf.classpath), &conf.runner_flags, &target, &conf.args);
    println!("[RUNNER] {}", &cmd);
    let status = Command::new("sh")
        .arg("-c")
        .arg(&cmd)
        .status();
    if let Ok(status) = status {
        if !status.success() {
            eprintln!("Error: Java program `{}` failed to run", &target);
        }
    } else {
        eprintln!("Error: Could not start Java program `{}`", &target);
    }
}

pub fn run_tests(target: &str, conf: &CONFIG)
{
    let mut cmds: Vec<String> = Vec::new();
    let files: Vec<PathBuf> = get_target_files(target, conf, false, PathType::TESTS)
        .expect("Couldn't get target files");
    for file in files {
        if let Some(name) = file.file_stem().and_then(|s| s.to_str())
        {
            let target_class = if target.is_empty()
            {
                name.to_string()
            }
            else
            {
                format!("{}.{}", target, name)
            };
            cmds.push(format!(
                "java -cp \"{}\" {} {} {}",
                conf.classpath,
                conf.runner_flags,
                target_class,
                conf.args
            ));
        }
    }
    if let Err(e) = launch_commands(cmds, conf)
    {
        eprintln!("Test run failed: {}", e);
    }
}
