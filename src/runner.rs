use std::{process::Command};
use crate::config::*;

pub fn run(target: &str, conf: &CONFIG)
{
    let cmd = format!("java -cp {} {} {} {}", 
                      format!("\"{}\"", &conf.classpath), conf.comp_flags, &target, &conf.args);
    println!("[RUNNER] {}", &cmd);
    let status = Command::new("bash")
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
