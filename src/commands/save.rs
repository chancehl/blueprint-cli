use clap::ArgMatches;
use std::path::PathBuf;

use crate::models::blueprint::Blueprint;

pub fn handler(arg_matches: &ArgMatches) -> Result<(), &'static str> {
    let blueprint_json_loc = arg_matches.get_one::<PathBuf>("BLUEPRINT");

    let no_validate = arg_matches.get_one::<bool>("no-validate");

    todo!();
}
