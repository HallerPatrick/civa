use super::error::ConfigError;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use log::info;

pub struct AliasSystem {
    alias_map: HashMap<String, String>,
}

impl AliasSystem {
    pub fn get_alias(&self, token: String) -> Option<&String> {
        self.alias_map.get(&token)
    }

    #[allow(dead_code)]
    pub fn update_alias(&mut self, token: String, alias: String) -> bool {
        self.alias_map.insert(token, alias).is_some()
    }
}

impl AliasSystem {
    pub fn from_file(path: &str) -> Result<Self, ConfigError> {
        lazy_static! {
            static ref ALIAS_REGEX: Regex =
                Regex::new(r#"^alias (?P<alias>[a-zA-Z.]+) *= *["'](?P<command>.*)["']"#).unwrap();
        }

        let mut lines: Vec<String> = file_to_vec(path);

        let mut alias_map: HashMap<String, String> = HashMap::new();
        for (index, line) in lines.iter_mut().enumerate() {
            if ALIAS_REGEX.is_match(line.trim()) {
                let caps = ALIAS_REGEX.captures(line).unwrap();
                info!("Alias: {}, Command: {}", &caps["alias"], &caps["command"]);
                alias_map.insert(String::from(&caps["alias"]), String::from(&caps["command"]));
            } else
            // Comments
            if line.trim().starts_with('#') || line.trim() == "" {
                // Pass
            } else {
                return Err(ConfigError {
                    message: format!("Error on line {}", index),
                });
            }
        }

        Ok(AliasSystem { alias_map })
    }
}

fn file_to_vec(path: &str) -> Vec<String> {
    let file = File::open(path).unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();

    let v: Vec<String> = contents.split('\n').map(String::from).collect();
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn read_aliases() {
    //     AliasSystem::from_file("/Users/patrickhaller/.dotfiles/bash_aliases");
    // }
}
