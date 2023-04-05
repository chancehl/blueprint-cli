use clap::ArgMatches;
use colored::Colorize;
use std::{fs, path::PathBuf};

use crate::models::{
    blueprint::Blueprint,
    repository::{BlueprintRepository, RepositoryType},
};

pub fn handler(arg_matches: &ArgMatches) -> Result<(), &'static str> {
    let blueprint_file = arg_matches
        .get_one::<PathBuf>("FILE")
        .expect("Missing file");

    let file_contents = fs::read_to_string(blueprint_file).expect("Could not read file");

    let blueprint_dir = BlueprintRepository::new(RepositoryType::LOCAL)
        .to_pathbuf()
        .expect("Could not initialize blueprint repository");

    match serde_json::from_str::<Blueprint>(&file_contents) {
        Ok(blueprint) => {
            if !blueprint_dir.exists() {
                eprintln!("Looks like the {:?} directory does not exist. Tip: you can run `blueprint init` to create this directory.", blueprint_dir.to_str().unwrap());

                return Err(".blueprint dir does not exist");
            }

            let destination = blueprint_dir.join(format!("{}.json", blueprint.name));

            fs::copy(blueprint_file, &destination).unwrap_or_else(|_| panic!("Could not write file to {}",
                destination.display()));

            println!(
                "{} Saved blueprint {} to {}",
                "✔".green(),
                &blueprint.name,
                &destination.display()
            );
        }
        Err(err) => {
            eprintln!("Error while running save command: {:?}", err);

            return Err("SAVE_ERROR");
        }
    }

    Ok(())
}
