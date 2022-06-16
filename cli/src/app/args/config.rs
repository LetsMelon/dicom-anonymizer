use clap::Arg;

use crate::app::types::StaticArg;

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
    Arg::new("read").takes_value(false).short('r').long("read")
}
