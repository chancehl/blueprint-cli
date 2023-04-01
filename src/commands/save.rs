use clap::ArgMatches;
use std::{env, fs, path::PathBuf};

use crate::models::blueprint::Blueprint;

pub fn handler(arg_matches: &ArgMatches) -> Result<(), &'static str> {
    let blueprint_json_loc = arg_matches
        .get_one::<PathBuf>("BLUEPRINT")
        .expect("You must provide the blueprint .json file location");

    let home = env::var("HOME").expect("Could not locate $HOME value");

    let file_contents = fs::read_to_string(blueprint_json_loc).expect("Could not read file");

    match serde_json::from_str::<Blueprint>(&file_contents) {
        Ok(blueprint) => {
            fs::copy(
                blueprint_json_loc,
                PathBuf::from(format!("{}/.blueprint/{}.json", home, blueprint.name)),
            )
            .expect(&format!("Could not write file to {}/.blueprint/", home));
        }
        Err(err) => {
            eprintln!("Error while running save command: {:?}", err)
        }
    }

    Ok(())
}
