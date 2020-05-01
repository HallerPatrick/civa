use super::command_bar::{command_bar_config_reader, CommandBarConfig};
use xdg;

pub struct ContextManager {
    // config_dir: Option<xdg::BaseDirectories>,
    pub command_bar_config: CommandBarConfig,
}

impl ContextManager {
    pub fn init() -> Self {
        let base_dir = xdg::BaseDirectories::with_prefix("civa");

        let config_dir: Option<xdg::BaseDirectories> = match base_dir {
            Ok(dir) => Some(dir),
            Err(_) => None,
        };

        let command_bar_config = match config_dir.clone() {
            Some(dir) => command_bar_config_reader(
                dir.find_config_file("bar.yaml").unwrap().to_str().unwrap(),
            )
            .unwrap(),
            None => CommandBarConfig::default(),
        };

        Self {
            // config_dir,
            command_bar_config,
        }
    }
}
