use std::{thread, process::Command, fs::create_dir_all, path::PathBuf, time};
use crate::paths::*;
use crate::packages::*;
use crate::config::*;
use crate::hashing::*;
use crate::dependencies::*;

pub fn  force_build_dir(package: &str, conf: &CONFIG) -> Result<(), String>
{
    if package.is_empty()
    {
        create_dir_all(&conf.bin)
            .map_err(|e|format!("Couldn't create '{}' : {}", &conf.bin, e))?;
        return Ok(());
    }
    if !validate_package(&package)
    {
        return  Err(format!("Stopping, package name not valid\n"));
    }
    let pkg_path = package_to_path(&package, PathType::CLASS, &conf); 
    create_dir_all(&pkg_path)
        .map_err(|e|format!("Couldn't create package '{}' : {}", pkg_path.display(), e))?;
    Ok(())
}

pub fn  create_compile_commands(target: &str, conf: &CONFIG) -> Vec<String>
{
    match force_build_dir(&target, &conf)
    {
        Ok(()) => (),
        Err(_) => return Vec::new(),
    }
    let mut files: Vec<PathBuf> = match get_target_files(target, &conf)
    {
        Ok(f) => f,
        Err(e) => 
        {
            eprintln!("Error: {}", e);
            return Vec::new();
        }
    };
    if files.is_empty()
    {
        println!("[COMPILER] Nothing to do");
        return Vec::new();
    }
    let mut commands: Vec<String> = Vec::new();
    files = topological_sort(files);
    for file in files
    {
        commands.push(format!("javac -cp {} -d {} {} {}", 
                      format!("\"{}\"", conf.classpath), conf.bin, conf.comp_flags, file.display()));
    }
    commands
}

pub fn  launch_commands(commands: Vec<String>, conf: &CONFIG) -> Result<(), std::io::Error>
{
    for chunk in commands.chunks(conf.threads as usize)
    {
        let chunk = chunk.to_vec();
        for cmd in chunk
        {
            println!("[COMPILER] {}", &cmd);
            let handle = thread::spawn(move ||{
                let status = Command::new("sh")
                    .arg("-c")
                    .arg(&cmd)
                    .status();
                if let Ok(status) = status
                {
                    if !status.success()
                    {
                        eprintln!("Command `{}` failed to run", &cmd);
                    }
                }
                else
                {
                    eprintln!("Error executing `{}`", &cmd);
                }
            });
            handle.join().expect("Error handling join");
        }
    }
    Ok(())
}

pub fn create_release(target: &str, conf: &CONFIG, entry: &str) {
    let files = match get_target_files(target, conf) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };
    let hash = match create_hash(&files) {
        Ok(h) => h,
        Err(e) => {
            eprintln!("Error generating hash: {}", e);
            return;
        }
    };
    let time = match time::SystemTime::now().duration_since(time::SystemTime::UNIX_EPOCH) {
        Ok(duration) => duration.as_secs(),
        Err(_) => {
            eprintln!("Error: Could not get system time");
            return;
        }
    };
    let pkgname = format!("{}-{}{}", target, time, hash);
    let cache_path = PathBuf::from(&conf.cache).join(&pkgname);
    create_dir_all(&cache_path).unwrap_or_else(|e| eprintln!("Error creating cache directory: {}", e));
    let jar_path = cache_path.join(format!("{}.jar", target));
    let status = Command::new("jar")
        .arg("cfe")
        .arg(&jar_path)
        .arg(entry)
        .arg("-C")
        .arg(&conf.bin)
        .arg(".")
        .status();
    if let Ok(status) = status
    {
        if status.success()
        {
            println!("Successfully created JAR `{}`", jar_path.display());
        }
        else
        {
            eprintln!("Error: Failed to create JAR `{}`", target);
        }
    }
    else
    {
        eprintln!("Error: Could not run `jar` command");
    }
}
