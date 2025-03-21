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

fn main()
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
        return ();
    }
    match args[1].as_str() {
            "init" =>
            {
                if args.len() == 2
                {
                    if let Err(e) = init_pkg("", &conf)
                    {
                        eprintln!("{}", e);
                    }
                    return ();
                }
                let package = &args[2];
                if let Err(e) = init_pkg(package, &conf)
                {
                    eprintln!("{}", e);
                }
            }
            "build" =>
            {
                let target = match args.len()
                {
                    2 => "",
                    _ => &args[2],
                };
                let commands = create_compile_commands(target, &conf);
                if commands.is_empty()
                {
                    ()
                }
                let _ = launch_commands(conf.pre.clone(), &conf);
                let _ = launch_commands(commands, &conf);
                let _ = launch_commands(conf.post.clone(), &conf);
                if args.contains(&"-r".to_string())
                    || args.contains(&"--release".to_string())
                    || args.contains(&"--cache".to_string())
                {
                    if args.len() < 4 
                    {
                        eprintln!("Error: Missing entry point for `build --release`");
                        return;
                    }
                    let entry_point = &args[3];
                    create_release(&target, &conf, &entry_point);
                }
            }
            "run" =>
            {
                if args.len() < 3
                {
                    eprintln!("Error: Missing main class for `run`");
                    return;
                }
                let target = &args[2];
                if args.len() > 3 {
                    conf.args = args[3..].join(" ");
                }
                run(&target, &conf);
            }
            _ => print_help(),
        }
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
