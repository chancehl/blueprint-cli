use clap::ArgMatches;
use std::{fs, path::PathBuf};

use crate::models::blueprint::Blueprint;
use crate::utils::blueprint_dir;

pub fn handler(arg_matches: &ArgMatches) -> Result<(), &'static str> {
    let blueprint_from = arg_matches
        .get_one::<PathBuf>("BLUEPRINT")
        .expect("You must provide the blueprint .json file location");

    let file_contents = fs::read_to_string(blueprint_from).expect("Could not read file");
    let blueprint_dir =
        blueprint_dir::as_pathbuf().expect("Could not crate .blueprint directory name");

    match serde_json::from_str::<Blueprint>(&file_contents) {
        Ok(blueprint) => {
            if !blueprint_dir.exists() {
                eprintln!("Looks like the {:?} directory does not exist. Tip: you can run `blueprint init` to create this directory.", blueprint_dir.as_os_str());

                return Err(".blueprint dir does not exist");
            }

            let blueprint_to = blueprint_dir.join(format!("{}.json", blueprint.name));

            fs::copy(blueprint_from, &blueprint_to).expect(&format!(
                "Could not write file to {}",
                blueprint_to.display()
            ));

            println!(
                "Saved blueprint {} to {}",
                &blueprint.name,
                &blueprint_to.display()
            );
        }
        Err(err) => {
            eprintln!("Error while running save command: {:?}", err)
        }
    }

    Ok(())
}
