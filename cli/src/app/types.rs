use clap::{Arg, ArgMatches, Command};
use anyhow::Result;

pub type StaticCommand = Command<'static>;
pub type StaticArg = Arg<'static>;

pub trait Matcher<T, Y> {
    fn match_args(matches: ArgMatches) -> Result<T>;
    fn match_trait(&self) -> Result<Y>;
}
