use super::command_bar::{command_bar_config_reader, CommandBarConfig};
use std::fs::File;
use std::path::Path;
use xdg;

static PREFIX: &str = "civa";
static COMMAND_BAR_CONFIG_FILE: &str = "bar.yaml";
static HISTORY_FILE: &str = "civa.history.txt";

pub struct ContextManager {
    // config_dir: Option<xdg::BaseDirectories>,
    pub command_bar_config: CommandBarConfig,
    base_dir: xdg::BaseDirectories,
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

                return Self {
                    base_dir: dir,
                    command_bar_config,
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
}
