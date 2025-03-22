use std::{fs, path::Path};
use crate::config::*;

fn get_conf_contents() -> String {
    fs::read_to_string("jmakefile").unwrap_or_else(|_| "".to_string())
}

pub fn parse_file(mut defaults: CONFIG) -> CONFIG {
    let config: String = get_conf_contents();

    if !Path::new("jmakefile").exists()
    {
        return defaults;
    }
    let mut inside_pre = false;
    let mut inside_post = false;

    for line in config.lines() {
        let stripped_line = line.trim();
        if stripped_line.is_empty() || stripped_line.starts_with("//")
        {
            continue;
        }
        if stripped_line.starts_with("pre={")
        {
            inside_pre = true;
            continue;
        }
        if stripped_line.starts_with("post={")
        {
            inside_post = true;
            continue;
        }
        if stripped_line == "};"
        {
            inside_pre = false;
            inside_post = false;
            continue;
        }
        if inside_pre
        {
            defaults.pre.push(
                stripped_line
                    .trim_end_matches(',')
                    .trim_matches('\'')
                    .to_string()
            );
            continue;
        }
        if inside_post
        {
            defaults.post.push(
                stripped_line
                    .trim_end_matches(',')
                    .trim_matches('\'')
                    .to_string()
            );
            continue;
        }
        let mut keyword = String::new();
        let mut value = String::new();
        let mut assignment = false;

        for c in stripped_line.chars() {
            if c == '='
            {
                assignment = true;
                continue;
            }
            if assignment
            {
                value.push(c);
            }
            else
            {
                keyword.push(c);
            }
        }

        match keyword.as_str() {
            "src" => defaults.src = value.trim_matches('\'').to_string(),
            "lib" => defaults.lib = value.trim_matches('\'').to_string(),
            "bin" => defaults.bin = value.trim_matches('\'').to_string(),
            "test" => defaults.test = value.trim_matches('\'').to_string(),
            "classpath" => defaults.classpath = value.trim_matches('\'').to_string(),
            "comp_flags" => defaults.comp_flags = value.trim_matches('\'').to_string(),
            "cache" => defaults.cache = value.trim_matches('\'').to_string(),
            "jvm_options" =>
            {
                defaults.jvm_options = value
                    .trim_matches('\'')
                    .split_whitespace()
                    .map(|s| s.to_string())
                    .collect();
            },
            "threads" => {
                match value.trim_matches('\'').parse::<usize>()
                {
                    Ok(0) =>
                    {
                        eprintln!("THREADS set to '0'. Aborting.");
                        std::process::exit(1);
                    },
                    Ok(n) => defaults.threads = n,
                    Err(_) => eprintln!("Warning: Invalid THREADS value '{}', using default.", value),
                }
            },
            _ => { 
                eprintln!("Warning: Unknown key '{}' in JMakefile, using default value...", keyword);
            }
        }
    }
    defaults
}
