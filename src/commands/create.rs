use clap::ArgMatches;
use std::path::PathBuf;

use crate::models::blueprint::Blueprint;

pub fn handler(arg_matches: &ArgMatches) -> Result<(), &'static str> {
    let blueprint_name = arg_matches
        .get_one::<String>("BLUEPRINT")
        .expect("Blueprint name required");

    let destination = arg_matches.get_one::<PathBuf>("DESTINATION");

    match Blueprint::seek(blueprint_name.to_string()) {
        Some(blueprint) => blueprint
            .execute(destination)
            .expect("Could not execute blueprint"),
        None => {
            eprintln!("Could not locate blueprint with name {0}. Did you mean to save instead?\n\n`blueprint save {0} --template=./{0}.json`", blueprint_name)
        }
    };

    Ok(())
}
