use clap::{Arg, Command};

pub type StaticCommand = Command<'static>;
pub type StaticArg = Arg<'static>;
