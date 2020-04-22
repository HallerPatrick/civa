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

use log::{debug, info};
use std::collections::HashMap;
use std::env::var;
use std::fs::read_dir;
use std::fs::read_link;
use std::fs::symlink_metadata;
use std::fs::DirEntry;
use std::path::PathBuf;

// Probably to expensive
fn collect_all_binaries_of_path() -> HashMap<String, String> {
    let mut path_bins: HashMap<String, String> = HashMap::new();
    let paths = retrieve_path_vars();

    for path in paths {
        let path_paths = match read_dir(&path) {
            Ok(p) => p,
            Err(_) => continue,
        };


        for p in path_paths {
            let dir: DirEntry = p.unwrap();

            // Collect all files in path, if not symlink
            if dir.file_type().unwrap().is_file() {
                path_bins.insert(
                    String::from(dir.file_name().to_str().unwrap()),
                    dir.path().to_str().unwrap().to_owned(),
                );
            } else {
                // Follow all symlinks
                if let Some(abs_path) = follow_symlink(dir.path()) {
                    path_bins.insert(
                        String::from(dir.file_name().to_str().unwrap()),
                        abs_path,
                    );
                }
            }
        }
    }

    info!("Found {} binaries in PATH", path_bins.len());
    path_bins
}


// TODO: Split up function
fn follow_symlink(dir: PathBuf) -> Option<String> {
    match symlink_metadata(dir.to_str().unwrap().clone()) {
        Ok(metadata) => {
            let file_type = metadata.file_type();

            if file_type.is_symlink() {
                match read_link(dir.to_str().unwrap().clone()) {
                    Ok(path_buf) => {
                        if path_buf.is_absolute() {
                            return Some(String::from(path_buf.to_str().unwrap()));
                        }

                        let mut abs_path: PathBuf = PathBuf::new();

                        abs_path.push(dir.parent().unwrap().to_str().unwrap());
                        abs_path.push(path_buf);

                        match abs_path.canonicalize() {
                            // Solve recursive symlinks
                            Ok(new_path) => follow_symlink(new_path),
                            Err(err) => {
                                debug!("Could not follow link {:?}, reason: {:?}", abs_path, err);
                                None
                            }
                        }

                        // return Some(String::from(new_path.to_str().unwrap()));
                    }
                    Err(_) => None,
                }
            } else if file_type.is_file() {
                Some(String::from(dir.to_str().unwrap()))
            } else {
                None
            }
        }
        Err(_) => None,
    }
}

fn retrieve_path_vars() -> Vec<String> {
    match var("PATH") {
        Ok(val) => split_var_string(val),
        Err(err) => panic!(format!("Could not retrieve PATH env, Reason: {}", err)),
    }
}

fn split_var_string(val: String) -> Vec<String> {
    val.split(':').map(String::from).collect()
}

pub struct EnvManager {
    env_vars: HashMap<String, String>,
}

impl EnvManager {
    pub fn new() -> Self {
        let env_vars: HashMap<String, String> = collect_all_binaries_of_path();
        Self { env_vars }
    }

    pub fn get_expanded(&self, command_name: String) -> Option<&String> {
        self.env_vars.get(&command_name)
    }

    pub fn has_command(&self, command_name: &str) -> bool {
        self.env_vars.contains_key(command_name)
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
