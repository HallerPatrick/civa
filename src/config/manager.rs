use super::alias::AliasSystem;
use super::command_bar::{command_bar_config_reader, CommandBarConfig};
use crate::env::environment::EnvManager;

use rcalc::Calculator;
use std::cell::RefCell;
use std::fs::File;
use std::path::Path;
use xdg;

static PREFIX: &str = "civa";
static COMMAND_BAR_CONFIG_FILE: &str = "bar.yaml";
static HISTORY_FILE: &str = "civa.history.txt";
static ALIAS_FILE: &str = "civa.alias.txt";

pub struct ContextManager {
    // config_dir: Option<xdg::BaseDirectories>,
    pub command_bar_config: CommandBarConfig,
    base_dir: xdg::BaseDirectories,
    pub alias_system: AliasSystem,
    pub env_manager: EnvManager,
    pub calculator: RefCell<Calculator>,
}

impl ContextManager {
    pub fn init() -> Self {
        let base_dir = xdg::BaseDirectories::with_prefix(PREFIX);
        let config_dir = base_dir;

        match config_dir {
            Ok(dir) => {
                let command_bar_config = command_bar_config_reader(
                    dir.find_config_file(COMMAND_BAR_CONFIG_FILE)
                        .unwrap()
                        .to_str()
                        .unwrap(),
                )
                .unwrap();

                let alias_system = AliasSystem::from_file(
                    dir.find_config_file(ALIAS_FILE).unwrap().to_str().unwrap(),
                )
                .unwrap();

                return Self {
                    calculator: RefCell::new(Calculator::new()),
                    base_dir: dir,
                    command_bar_config,
                    alias_system,
                    env_manager: EnvManager::new(),
                };
            }
            Err(_) => panic!("Could not find home"), //CommandBarConfig::default(),
        };
    }

    pub fn retrieve_history_cache(&self) -> String {
        match self.base_dir.find_cache_file(HISTORY_FILE) {
            Some(buf) => buf.to_str().unwrap().to_string(),
            None => {
                let cache_file = self
                    .base_dir
                    .place_cache_file(Path::new(HISTORY_FILE))
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string();

                match File::create(Path::new(&cache_file)) {
                    Ok(_) => cache_file,
                    Err(_) => String::new(),
                }
            }
        }
    }

    // pub fn retrieve_alias_config(&self) -> String {
    //     match self.base_dir.find_config_file(ALIAS_FILE) {
    //         Some(buf) => buf.to_str().unwrap().to_string(),
    //         None => {
    //             let alias_file = self
    //                 .base_dir
    //                 .place_config_file(Path::new(ALIAS_FILE))
    //                 .unwrap()
    //                 .to_str()
    //                 .unwrap()
    //                 .to_string();

    //             match File::create(Path::new(&alias_file)) {
    //                 Ok(_) => alias_file,
    //                 Err(_) => String::new(),
    //             }
    //         }
    //     }
    // }
}
