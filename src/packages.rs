use std::{fs::{self, File}, io::{Write}, path::PathBuf};
use crate::config::*;

#[derive(PartialEq, Eq)]
pub enum PathType {
    SRC,
    TESTS,
    CLASS,
}

pub fn package_to_path(target: &str, t: PathType, conf: &CONFIG) -> PathBuf
{
    let mut pkg_path: PathBuf = match t
    {
        PathType::SRC => PathBuf::from(conf.src.clone()),
        PathType::CLASS => PathBuf::from(conf.bin.clone()),
        PathType::TESTS => PathBuf::from(conf.test.clone()),

    };
    for path in target.split('.')
    {
        pkg_path.push(path);
    }
    pkg_path
}

pub fn validate_package(name: &str) -> bool
{
    if name.ends_with('.')
        || name.starts_with('.')
        || name.contains(' ')
        || name.contains("..")
        || name.contains(". ")
    {
        return false;
    }
    true
}

pub fn init_pkg(package: &str, conf: &CONFIG) -> Result<(), String>
{
    if !validate_package(&package)
    {
        return Err(format!("Stopping, package name not valid\n"));
    }
    let mut pkg_path = package_to_path(&package, PathType::SRC, &conf);
    let test_path = package_to_path(&package, PathType::TESTS, &conf);
    fs::create_dir_all(&pkg_path)
        .map_err(|e| format!("Coundn't create package '{}' : {}", pkg_path.display(), e))?;
    fs::create_dir_all(&conf.lib)
        .map_err(|e| format!("Coundn't create package '{}' : {}", pkg_path.display(), e))?;
    fs::create_dir_all(&test_path)
        .map_err(|e| format!("Coundn't create package '{}' : {}", pkg_path.display(), e))?;
    pkg_path.push("Main.java");
    let mut file: File = File::create(&pkg_path)
        .map_err(|e| format!("Couldn't create file '{}' : {}", pkg_path.display(), e))?;

    let java_contents = match package
    {
        "" => format!("public class Main\n{{\n\tpublic static void main(String[] args)\n\t{{\n\t\tSystem.out.println(\"Hello from no-package\");\n\t}}\n}}"),
        _ => format!("package {};\npublic class Main\n{{\n\tpublic static void main(String[] args)\n\t{{\n\t\tSystem.out.println(\"Hello from no-package\");\n\t}}\n}}", &package),
    };
    file.write_all(java_contents.as_bytes())
        .map_err(|e| format!("Coulnd't create file '{}' : {}", pkg_path.display(), e))?;

    Ok(())
}
