//
// The command bar describes the bar which is constanly
// shown to the user.
//
// In general does is consists of following components:
//
// {CWD}{SVN Status}{PROMPT_SYMBOL}{INPUT_FIELD}|{OPTIONALS}
//
//
// CWD = Current working directory, fold if to long or referencing user directories
// SVN = Quite common now, shows current branch and current changes
// PROMPT_SYMBOL = Usually something like ">" or "$"
// INPUT_FIELD = Where the use input text is displayed
// OPTIONALS = Zsh shows the current times or amount of inputed commands
//
//
// Civa provides customization
//
//

use std::fs;

use log::info;
use yaml_rust::{Yaml, YamlLoader};

use super::error::ConfigError;
use super::{Color, Style};

#[derive(Debug)]
enum CommandBarComponents {
    CWD,
    SVN,
    PROMPT,
    INPUTFIELD,
    USER,
}

#[derive(Debug)]
struct Component {
    color: Color,
    style: Style,
    component_type: CommandBarComponents,
}

impl Component {
    fn default(component: CommandBarComponents) -> Self {
        Self {
            color: Color::default(),
            style: Style::NORMAL,
            component_type: component,
        }
    }
}

#[derive(Debug)]
struct CommandBarConfig {
    component: Vec<Component>,
}

impl CommandBarConfig {
    fn default() -> Self {
        Self {
            component: vec![
                Component::default(CommandBarComponents::CWD),
                Component::default(CommandBarComponents::SVN),
                Component::default(CommandBarComponents::PROMPT),
            ],
        }
    }
}

fn command_bar_config_reader(config_file: &str) -> Result<CommandBarConfig, ConfigError> {
    let content = match fs::read_to_string(config_file) {
        Ok(c) => c,
        Err(_) => return Ok(CommandBarConfig::default()),
    };

    let mut config = match YamlLoader::load_from_str(content.as_str()) {
        Err(err) => {
            return Err(ConfigError {
                message: format!("{:?}", err),
            })
        }
        Ok(c) => c,
    };

    config_builder(&mut config);

    Ok(CommandBarConfig::default())
}

fn config_builder(config: &mut Vec<Yaml>) {
    info!("{:?}", config);
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_config_yaml() {
        let c = command_bar_config_reader("examples/.civa.bar.yaml");

        assert_eq!(format!("{:?}", c), "")
    }

    #[test]
    fn test_config_yal() {
        let c = command_bar_config_reader("examples/.civa.bar.yaml");

        assert_eq!(format!("{:?}", c), "")
    }
}
