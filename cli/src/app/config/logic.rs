use crate::app::config::ConfigFileVersions;
use anyhow::Result;
use clap::ArgMatches;

use crate::app::config::values::{ConfigAction, ConfigManager};
use crate::app::types::IMatcher;

pub fn logic(matches: ArgMatches) -> Result<()> {
    let matches = matches.subcommand_matches("config").unwrap().clone();

    let matches = *ConfigManager::match_args(matches)?;

    let file = ConfigFileVersions::parse(matches.input.clone())
        .expect("Error while reading the config file");

    match matches.action {
        ConfigAction::Create => {
            todo!();
        }
        ConfigAction::Modify => {
            todo!();
        }
        ConfigAction::Show => {
            println!("{}", file);
        }
    };
    Ok(())
}
