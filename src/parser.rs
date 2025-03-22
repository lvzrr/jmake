use std::{fs, path::Path};
use crate::config::*;

fn get_conf_contents() -> String
{
    fs::read_to_string("jmake.toml").unwrap_or_else(|_| "".to_string())
}

pub fn parse_file(mut defaults: CONFIG) -> CONFIG
{
    let config: String = get_conf_contents();

    if !Path::new("jmake.toml").exists()
    {
        return defaults;
    }

    let mut current_key: Option<String> = None;
    let mut collecting_array: Vec<String> = Vec::new();
    let mut inside_array = false;

    for line in config.lines()
    {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#')
        {
            continue;
        }

        if inside_array
        {
            if line.ends_with(']')
            {
                let clean = line.trim_end_matches(']').trim();
                if !clean.is_empty()
                {
                    collecting_array.extend(clean.split(',').map(|s| s.trim().trim_matches(&['"', '\''][..]).to_string()));
                }
                if let Some(ref key) = current_key
                {
                    match key.as_str()
                    {
                        "pre" => defaults.pre = collecting_array.clone(),
                        "post" => defaults.post = collecting_array.clone(),
                        "jvm_options" => defaults.jvm_options = collecting_array.clone(),
                        "run_args" => defaults.run_args = collecting_array.clone(),
                        "sandbox" => defaults.sandbox = collecting_array.clone(),
                        _ => eprintln!("Warning, unrecognised key '{}': using default config", key),
                    }
                }
                collecting_array.clear();
                inside_array = false;
                current_key = None;
            }
            else
            {
                collecting_array.extend(line.split(',').map(|s| s.trim().trim_matches(&['"', '\''][..]).to_string()));
            }
            continue;
        }

        if line.contains('=')
        {
            let parts: Vec<&str> = line.splitn(2, '=').collect();
            let key = parts[0].trim();
            let value = parts[1].trim().trim_matches(&['"', '\''][..]);

            if value.starts_with('[') && !value.ends_with(']')
            {
                inside_array = true;
                current_key = Some(key.to_string());
                let first = value.trim_start_matches('[').trim();
                if !first.is_empty()
                {
                    collecting_array.extend(first.split(',').map(|s| s.trim().trim_matches(&['"', '\''][..]).to_string()));
                }
                continue;
            }

            if value.starts_with('[') && value.ends_with(']')
            {
                let contents = &value[1..value.len() - 1];
                let arr = contents
                    .split(',')
                    .map(|s| s.trim().trim_matches(&['"', '\''][..]).to_string())
                    .collect::<Vec<String>>();
                match key
                {
                    "pre" => defaults.pre = arr,
                    "post" => defaults.post = arr,
                    "jvm_options" => defaults.jvm_options = arr,
                    "run_args" => defaults.run_args = arr,
                    "sandbox" => defaults.sandbox = arr,
                    _ => eprintln!("Warning, unrecognised key '{}': using default config", key),
                }
                continue;
            }

            match key
            {
                "src" => defaults.src = value.to_string(),
                "bin" => defaults.bin = value.to_string(),
                "lib" => defaults.lib = value.to_string(),
                "test" => defaults.test = value.to_string(),
                "cache" => defaults.cache = value.to_string(),
                "classpath" => defaults.classpath = value.to_string(),
                "jvm_version" => defaults.jvm_version = match value.to_string().as_str()
                {
                    "1" => jni::JNIVersion::V1,
                    "2" => jni::JNIVersion::V2,
                    "4" => jni::JNIVersion::V4,
                    "6" => jni::JNIVersion::V6,
                    "8" => jni::JNIVersion::V8,
                    _ => jni::JNIVersion::Invalid(0),
                },
                "comp_flags" => defaults.comp_flags = value.to_string(),
                "threads" =>
                {
                    if let Ok(n) = value.parse::<usize>()
                    {
                        defaults.threads = n;
                    }
                },
                _ =>
                {
                    eprintln!("Warning, unrecognised key '{}': using default config", key);
                }
            }
        }
    }
    defaults
}
