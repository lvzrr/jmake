mod packages;
mod paths;
mod config;
mod compile;
mod parser;
mod hashing;
mod runner;
mod dependencies;

use std::{env, path::PathBuf};
use crate::config::CONFIG;
use crate::parser::*;
use crate::compile::*;
use crate::packages::*;
use crate::runner::*;

fn main() -> Result<(), Box<dyn std::error::Error>>
{
    let mut conf: CONFIG = CONFIG
    {
        pre : Vec::new(),
        src : String::from("src"),
        bin : String::from("bin"),
        lib : String::from("lib"),
        cache: PathBuf::from(env::var("HOME").unwrap())
        .join(".cache/jmake")
        .to_string_lossy()
        .to_string(),
        comp_flags : String::new(),
        runner_flags : String::new(),
        args : String::new(),
        classpath : String::from("bin:lib/*"),
        post : Vec::new(),
        threads : std::thread::available_parallelism().unwrap().get(),
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
                let cmd: String = create_compile_command(target, &conf); 
                if cmd.is_empty()
                {
                    println!("[COMPILER] Nothing to compile.");
                }
                else
                {
                    commands.push(cmd);
                    launch_commands(conf.pre.clone())
                        .map_err(|e| format!("Failed running PRE commands: {}", e))?;
                    launch_commands(commands)
                        .map_err(|e| format!("Compilation failed: {}", e))?;
                    launch_commands(conf.post.clone())
                        .map_err(|e| format!("Failed running POST commands: {}", e))?;
                }
                if args.contains(&"-r".to_string())
                    || args.contains(&"--release".to_string())
                    || args.contains(&"--cache".to_string())
                {
                    if args.len() < 4
                    {
                        return Err("Missing entry point for `build --release`".into());
                    }
                    let entry_point = &args[3];
                    create_release(target, &conf, entry_point);
                }
            }
            "run" =>
            {
                if args.len() < 3
                {
                    return Err("Missing main class for `run`".into());
                }
                let target = &args[2];
                if args.len() > 3 {
                    conf.args = args[3..].join(" ");
                }
                run(&target, &conf);
            }
            _ => print_help(),
        }
        Ok(())
    }

    fn print_help() {
        println!(
            "Usage (omit bin & src folders):

            jmake [COMMAND] [TARGET] [FLAGS]

            jmake init <package>  <flags>        - Initialize a new package
            jmake build <target>  <flags>        - Build a Java package
            jmake run <MainClass> <flags>        - Run a compiled Java program"
        );
    }
