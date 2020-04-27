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
pub enum CommandBarComponents {
    CWD,
    SVN,
    PROMPT,
    USER,
    UNDEFINED,
}

#[derive(Debug)]
pub struct Component {
    pub color: Color,
    pub style: Style,
    pub component_type: CommandBarComponents,
}

impl Component {
    fn from_string(component_name: &str, color: Color, style: Style) -> Self {
        let comp = match component_name.to_lowercase().as_str() {
            "cwd" => CommandBarComponents::CWD,
            "svn" => CommandBarComponents::SVN,
            "prompt" => CommandBarComponents::PROMPT,
            "user" => CommandBarComponents::USER,
            _ => CommandBarComponents::UNDEFINED,
        };

        Self {
            component_type: comp,
            color,
            style,
        }
    }
    fn default(component: CommandBarComponents) -> Self {
        Self {
            color: Color::default(),
            style: Style::default(),
            component_type: component,
        }
    }
}

#[derive(Debug)]
pub struct CommandBarConfig {
    pub components: Vec<Component>,
}

impl CommandBarConfig {
    fn default() -> Self {
        Self {
            components: vec![
                Component::default(CommandBarComponents::CWD),
                Component::default(CommandBarComponents::SVN),
                Component::default(CommandBarComponents::PROMPT),
            ],
        }
    }
}

pub fn command_bar_config_reader(config_file: &str) -> Result<CommandBarConfig, ConfigError> {
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

    Ok(config_builder(&mut config))
}

fn config_builder(config: &mut Vec<Yaml>) -> CommandBarConfig {
    let config = &config[0];
    let component_order: Vec<&str> = config["component_order"]
        .as_vec()
        .unwrap()
        .iter()
        .map(|c| c.as_str().unwrap())
        .collect();

    let mut components: Vec<Component> = Vec::new();

    for component_name in component_order {
        info!("Component: {}", component_name);
        let component_config = &config[component_name];
        info!("With config: {:?}", component_config);

        components.push(Component::from_string(
            component_name,
            Color::from_string(component_config["color"].as_str().unwrap()),
            Style::from_string(component_config["style"].as_str().unwrap()),
        ))
    }

    CommandBarConfig { components }
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
