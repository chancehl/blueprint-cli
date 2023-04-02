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
            let location = PathBuf::from(format!("{}/.blueprint/{}.json", home, blueprint.name));

            if !location.exists() {
                eprintln!("Looks like the {}/.blueprint directory does not exist. Tip: you can run `blueprint init` to create this directory.", home);

                return Err(".blueprint dir does not exist");
            }

            fs::copy(blueprint_json_loc, &location)
                .expect(&format!("Could not write file to {}", location.display()));

            println!(
                "Saved blueprint {} to {}",
                &blueprint.name,
                &location.display()
            );
        }
        Err(err) => {
            eprintln!("Error while running save command: {:?}", err)
        }
    }

    Ok(())
}
