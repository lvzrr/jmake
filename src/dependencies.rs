use std::{fs, path::PathBuf, collections::{HashMap, VecDeque}};
use regex::Regex;

pub fn  get_classes(file: PathBuf) -> Result<Vec<String>, std::io::Error>
{
    let contents: String = fs::read_to_string(&file)?;
    let re = Regex::new(r"\b[A-Z][a-zA-Z0-9_]*\b").unwrap();
    let mut dependencies: Vec<String> = Vec::new();
    for capture in re.find_iter(&contents)
    {
        dependencies.push(capture.as_str().to_string());
    }
    Ok(dependencies)
}

pub fn topological_sort(files: Vec<PathBuf>) -> Vec<PathBuf>
{
    let graph = create_hashmap(files.clone());
    let mut class_to_path: HashMap<String, PathBuf> = HashMap::new();
    let valid_classes: std::collections::HashSet<String> = files.iter()
        .filter_map(|f| f.file_stem())
        .filter_map(|s| s.to_str())
        .map(|s| s.to_string())
        .collect();

    for file in &files
    {
        if let Some(class_name) = file.file_stem().and_then(|s| s.to_str())
        {
            class_to_path.insert(class_name.to_string(), file.clone());
        }
    }

    let mut in_degree: HashMap<String, usize> = HashMap::new();
    for (class, dependencies) in &graph
    {
        for dep in dependencies
        {
            if valid_classes.contains(dep) && dep != class
            {
                *in_degree.entry(dep.clone()).or_insert(0) += 1;
            }
        }
        in_degree.entry(class.clone()).or_insert(0);
    }

    let mut queue = VecDeque::new();
    for (class, &count) in &in_degree
    {
        if count == 0 {
            queue.push_back(class.clone());
        }
    }

    let mut sorted_paths: Vec<PathBuf> = Vec::new();
    while let Some(class) = queue.pop_front()
    {
        if let Some(path) = class_to_path.get(&class)
        {
            sorted_paths.push(path.clone());
        }
        if let Some(dependents) = graph.get(&class)
        {
            for dependent in dependents
            {
                if valid_classes.contains(dependent)
                {
                    if let Some(count) = in_degree.get_mut(dependent)
                    {
                        *count -= 1;
                        if *count == 0
                        {
                            queue.push_back(dependent.clone());
                        }
                    }
                }
            }
        }
    }
    sorted_paths.reverse();
    sorted_paths
}

pub fn create_hashmap(files: Vec<PathBuf>) -> HashMap<String, Vec<String>>
{
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    let valid_classes: std::collections::HashSet<String> = files.iter()
        .filter_map(|f| f.file_stem())
        .filter_map(|s| s.to_str())
        .map(|s| s.to_string())
        .collect();

    for file in files
    {
        let class_name = match file.file_stem().and_then(|s| s.to_str())
        {
            Some(name) => name.to_string(),
            None => continue,
        };
        let deps = match get_classes(file.clone())
        {
            Ok(d) => d.into_iter().filter(|dep| valid_classes.contains(dep) && dep != &class_name).collect(),
            Err(_) => Vec::new(),
        };
        graph.insert(class_name, deps);
    }
    graph
}
