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
        let alias_regex = Regex::new(r"^alias [a-zA-Z0-9]+[ ]*=[ ]*'.*'$").unwrap();

        let mut lines: Vec<String> = file_to_vec(path);

        let mut alias_map: HashMap<String, String> = HashMap::new();
        for (index, line) in lines.iter_mut().enumerate() {
            if alias_regex.is_match(line.trim()) {
                line.drain(..5);
                let all: Vec<String> = line.split("=").map(String::from).collect();
                let alias_name = all[0].trim();
                let alias_expr = all[1].trim().replace("'", "");

                info!("{}:{}", alias_name, alias_expr);

                alias_map.insert(String::from(alias_name), String::from(alias_expr));
            } else
            // Comments
            if line.trim().starts_with("#") || line.trim() == "" {
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

    let v: Vec<String> = contents.split("\n").map(String::from).collect();
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_aliases() {
        AliasSystem::from_file("/Users/patrickhaller/.dotfiles/bash_aliases");
    }
}
