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

pub fn create_compile_command(target: &str, conf: &CONFIG) -> String
{
    if let Err(e) = force_build_dir(target, conf)
    {
        eprintln!("Error creating build dir: {}", e);
        return "".to_string();
    }
    let mut files: Vec<PathBuf> = match get_target_files(target, conf, true)
    {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error: {}", e);
            return "".to_string();
        }
    };
    if files.is_empty()
    {
        return "".to_string();
    }
    files = topological_sort(files);
    let mut command = format!(
        "javac -cp \"{}\" -d {} {}",
        conf.classpath, conf.bin, conf.comp_flags
    );
    for file in files
    {
        command.push(' ');
        command.push_str(&file.display().to_string());
    }
    command
}

pub fn  launch_commands(commands: Vec<String>) -> Result<(), std::io::Error>
{
        for cmd in commands
        {
            if !cmd.is_empty()
            {
                println!("[COMPILER] {}", &cmd);
            }
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
    Ok(())
}

pub fn create_release(target: &str, conf: &CONFIG, entry: &str)
{
    let files = match get_target_files(target, conf, false)
    {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };
    let hash = match create_hash(&files)
    {
        Ok(h) => h,
        Err(e) => {
            eprintln!("Error generating hash: {}", e);
            return;
        }
    };
    let time = match time::SystemTime::now().duration_since(time::SystemTime::UNIX_EPOCH)
    {
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
