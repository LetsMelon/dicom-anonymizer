use anyhow::{bail, Result};
use clap::ArgMatches;
use std::path::PathBuf;
use strum::EnumCount;

use crate::app::types::IMatcher;

#[derive(Debug, Copy, Clone, Eq, PartialEq, EnumCount)]
pub enum ConfigAction {
    Create,
    Modify,
    Show,
}

#[derive(Debug)]
pub struct ConfigManager {
    pub input: PathBuf,
    pub action: ConfigAction,
}

impl ConfigManager {
    fn new(path: PathBuf, action: ConfigAction) -> Self {
        Self {
            input: path,
            action,
        }
    }
}

impl IMatcher<ConfigManager> for ConfigManager {
    fn match_args(matches: ArgMatches) -> Result<Box<Self>> {
        let action_create = matches.contains_id("create");
        let action_modify = matches.contains_id("modify");
        let action_show = matches.contains_id("show");

        let sum = (action_create as i32) + (action_modify as i32) + (action_show as i32);

        // Checks
        if sum != 1 {
            bail!("Must specify on of the options! (create, modify or show)");
        }

        let input = PathBuf::from(matches.value_of("input").unwrap());
        debug_assert!(
            ConfigAction::COUNT == 3,
            "TODO: implement new 'ConfigAction' member in if"
        );
        let action = if action_create {
            ConfigAction::Create
        } else if action_modify {
            ConfigAction::Modify
        } else {
            // else if not necessary, because it's only when 'action_show' has been set
            ConfigAction::Show
        };

        Ok(Box::from(ConfigManager::new(input, action)))
    }

    fn match_trait(&self) -> Result<ConfigManager> {
        // TODO rethink about the IMatcher trait
        todo!()
    }
}
