use std::{fs, path::{PathBuf, Path}, time::SystemTime};
use crate::packages::*;
use crate::config::*;

pub fn  get_target_files(target: &str, conf: &CONFIG, check: bool, t: PathType) -> Result<Vec<PathBuf>, String>
{
    let ext: &str = match t
    {
        PathType::SRC => ".java",
        PathType::TESTS => ".java",
        PathType::CLASS => ".class",
    };
    let mut files: Vec<PathBuf> = Vec::new();
    let target_dir: PathBuf = package_to_path(&target, t, &conf);
    if !target_dir.is_dir()
    {
        return Err(format!("'{}' is not a directory", target_dir.display()));
    }
    for entry in fs::read_dir(&target_dir)
        .map_err(|e| format!("Couldn't open target {} : {}", target_dir.display(), e))?
    {
        let entry = entry.map_err(|e| format!("Couldn't read entry: {}", e))?;
        if let Some(filename) = entry.file_name().to_str()
        {
                if filename.ends_with(ext) && check_incremental(&entry.path(), conf, check)
                {
                    files.push(entry.path());
                }
        }
    }
    Ok(files)
}

pub fn check_incremental(file: &Path, conf: &CONFIG, check: bool) -> bool
{
    if !check
    {
        return true;
    }
    let class_file = file.strip_prefix(&conf.src)
        .map(|rel_path| PathBuf::from(&conf.bin).join(rel_path))
        .unwrap_or_else(|_| file.to_path_buf())
        .with_extension("class");
    if !class_file.exists()
    {
        return true;
    }
    let src_mod_time: SystemTime = match fs::metadata(file).and_then(|meta|meta.modified())
    {
        Ok(time) => time,
        Err(_) => return true,
    };
    let class_mod_time: SystemTime = match fs::metadata(&class_file).and_then(|meta|meta.modified())
    {
        Ok(time) => time,
        Err(_) => return true,
    };
    src_mod_time > class_mod_time
}

pub fn expand_classpath(cp: &str) -> String
{
    let mut parts = Vec::new();
    let sep = if cfg!(windows) { ";" } else { ":" };

    for entry in cp.split(sep)
    {
        let is_wildcard = entry.ends_with("/*") || entry.ends_with("\\*");

        if is_wildcard
        {
            let dir = &entry[..entry.len() - 2];
            if let Ok(read_dir) = fs::read_dir(dir)
            {
                for file in read_dir.flatten()
                {
                    let path = file.path();
                    if path.extension().map_or(false, |ext| ext == "jar")
                    {
                        parts.push(path.to_string_lossy().to_string());
                    }
                }
            }
        }
        else
        {
            parts.push(entry.to_string());
        }
    }
    parts.join(sep)
}
