use clap::Arg;

use crate::app::types::StaticArg;
use crate::app::validator::validator_is_file_path;

#[inline(always)]
pub fn input() -> StaticArg {
    Arg::new("input")
        .takes_value(true)
        .value_name("FILE")
        .required(true)
        .help("Path to yaml config-file")
        .validator(validator_is_file_path)
}

#[inline(always)]
pub fn create() -> StaticArg {
    Arg::new("create")
        .takes_value(false)
        .short('c')
        .long("create")
}

#[inline(always)]
pub fn modify() -> StaticArg {
    Arg::new("modify")
        .takes_value(false)
        .short('m')
        .long("modify")
}

#[inline(always)]
pub fn read() -> StaticArg {
    Arg::new("show").takes_value(false).short('s').long("show")
}
