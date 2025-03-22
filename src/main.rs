mod packages;
mod native;
mod paths;
mod config;
mod compile;
mod parser;
mod hashing;
mod runner;

use std::{env, path::PathBuf};
use jni;
use crate::config::CONFIG;
use crate::parser::*;
use crate::compile::*;
use crate::packages::*;
use crate::runner::*;

fn main() -> Result<(), Box<dyn std::error::Error>>
{
    let home_dir = env::var("HOME")
        .or_else(|_| env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".to_string());
    let mut conf: CONFIG = CONFIG
    {
        pre:           Vec::new(),
        src:           String::from("src"),
        bin:           String::from("bin"),
        lib:           String::from("lib"),
        test:          String::from("test"),
        cache:         PathBuf::from(home_dir)
                        .join(".cache")
                        .join("jmake")
                        .to_string_lossy()
                        .to_string(),

        jvm_options:    Vec::new(),
        jvm_version:    jni::JNIVersion::V8,
        comp_flags:     String::new(),
        run_args:       Vec::new(),
        classpath:      if cfg!(windows) { "bin;lib;lib\\*".to_string() } else { "bin:lib:lib/*".to_string() },
        post:           Vec::new(),
        threads:        std::thread::available_parallelism().unwrap().get(),
    };
    conf = parse_file(conf);
    let args: Vec<String> = env::args().collect();
    if args.len() == 1
    {
        print_help();
        return Ok(());
    }
    match args[1].as_str() {
            "init" =>
            {
                if args.len() == 2
                {
                    if let Err(_e) = init_pkg("", &conf)
                    {
                        return Err("Couldnt initialize non-packaged project".into());
                    }
                    return Ok(());
                }
                let package = &args[2];
                if let Err(_e) = init_pkg(package, &conf)
                {
                    return Err(format!("Couldnt initialize {}", &package).into());
                }
            }
            "build" =>
            {
                let target = if args.len() == 2 { "" } else { &args[2] };

                let mut commands: Vec<String> = Vec::new();
                let cmd: String = create_compile_command(target, &conf, PathType::SRC); 
                if cmd.is_empty()
                {
                    println!("[COMPILER] Nothing to compile.");
                }
                else
                {
                    commands.push(cmd);
                    launch_commands(conf.pre.clone(), &conf, "PRE")
                        .map_err(|e| format!("Failed running PRE commands: {}", e))?;
                    launch_commands(commands, &conf, "COMPILER")
                        .map_err(|e| format!("Compilation failed: {}", e))?;
                    launch_commands(conf.post.clone(), &conf, "POST")
                        .map_err(|e| format!("Failed running POST commands: {}", e))?;
                }
                if args.contains(&"-r".to_string())
                    || args.contains(&"--release".to_string())
                    || args.contains(&"--cache".to_string())
                {
                    let entry_point = &args[3];
                    create_release(target, &conf, entry_point);
                }
            }
            "test" =>
            {
                let target = if args.len() == 2 { "" } else { &args[2] };

                let mut commands: Vec<String> = Vec::new();
                let cmd: String = create_compile_command(target, &conf, PathType::TESTS); 
                if cmd.is_empty()
                {
                    println!("[COMPILER] Nothing to compile.");
                }
                else
                {
                    commands.push(cmd);
                    launch_commands(conf.pre.clone(), &conf, "PRE")
                        .map_err(|e| format!("Failed running PRE commands: {}", e))?;
                    launch_commands(commands, &conf, "COMPILER")
                        .map_err(|e| format!("Compilation failed: {}", e))?;
                    launch_commands(conf.post.clone(), &conf, "POST")
                        .map_err(|e| format!("Failed running POST commands: {}", e))?;
                }
                return run_tests(&target, &conf);
            }
            "run" =>
            {
                if args.len() < 3
                {
                    return Err("Missing main class for `run`".into());
                }
                let target = &args[2];
                if args.len() > 3 {
                    conf.run_args = args[3..].to_vec();
                }
               return run(&target, &conf);
            }
            "clean" =>
            {
                let bin_path = PathBuf::from(&conf.bin);
                if bin_path.exists()
                {
                    if let Err(e) = std::fs::remove_dir_all(&bin_path)
                    {
                        return Err(format!("Failed to clean '{}': {}", conf.bin, e).into());
                    }
                    println!("[CLEAN] Deleted directory '{}'", conf.bin);
                }
                else
                {
                    println!("[CLEAN] Directory '{}' does not exist", conf.bin);
                }
            }
            _ => print_help(),
        }
        Ok(())
    }

    fn print_help() {
        println!(
            "Usage (omit bin & src folders):

        jmake [COMMAND] [TARGET] [FLAGS]

        Commands:
        init <package>              Initialize a new Java package
        build <target>              Compile Java files from src/
                                    Use --release <MainClass> to create a .jar
        test <target>               Compile and run tests from test/
                                    Will look for classes like <target>.TestsMain
        run <MainClass> [args...]   Run the given class from bin/

        Examples:
        jmake init mypkg
        jmake build mypkg
        jmake build mypkg --release mypkg.Main
        jmake test testpkg
        jmake run mypkg.Main arg1 arg2"
        );
    }
