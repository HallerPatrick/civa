// Environment Manager
//
// Collects all paths of PATH
//
//
// For now we go for the naive way and collect all possible binaries in every path
//
//
// All binaries are saved in a HashMap with the command name being the key
//
// e.g.:
//
//  "chsh" => /bin/chsh,
//  "ls" => /bin/ls
//  "cd" => /usr/bin/cd
//
//
// If a other command with the same name is found we can overwrite the old one
//

use log::info;
use std::collections::HashMap;
use std::env::var;
use std::fs::read_dir;

// Probably to expensive
fn collect_all_binaries_of_path() -> HashMap<String, String> {
    let mut path_bins: HashMap<String, String> = HashMap::new();
    let paths = retrive_path_vars();

    for path in paths {
        let path_paths = match read_dir(path.clone()) {
            Ok(p) => p,
            Err(_) => continue,
        };

        for p in path_paths {
            let dir = p.unwrap();

            if dir.file_type().unwrap().is_file() {
                path_bins.insert(path.clone(), String::from(dir.path().to_str().unwrap()));
            }
        }
    }

    info!("Found {} binaries in PATH", path_bins.len());
    info!("Found: {:?}", path_bins);
    path_bins
}

fn retrive_path_vars() -> Vec<String> {
    match var("PATH") {
        Ok(val) => split_var_string(val),
        Err(_) => panic!("Could not retrieve PATH env"),
    }
}

fn split_var_string(val: String) -> Vec<String> {
    val.split(":").map(|s| String::from(s)).collect()
}

pub struct EnvManager {
    env_vars: HashMap<String, String>,
}

impl EnvManager {
    pub fn new() -> Self {
        let mut env_vars: HashMap<String, String> = collect_all_binaries_of_path();
        Self { env_vars }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_split_var_string() {
        let result = split_var_string(String::from("/hello/world:other/one"));
        assert_eq!(result, vec!["/hello/world", "other/one"])
    }
}
