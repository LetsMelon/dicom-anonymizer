use anyhow::Result;
use clap::{Arg, ArgMatches, Command};
use yaml_rust::Yaml;

pub type StaticCommand = Command<'static>;
pub type StaticArg = Arg<'static>;

pub trait IMatcher<T> {
    fn match_args(matches: ArgMatches) -> Result<Box<Self>>;
    fn match_trait(&self) -> Result<T>;
}

pub trait IConfigFile {
    fn parse(content: Vec<Yaml>) -> Result<Box<Self>>;
    fn get_version() -> String;
}
