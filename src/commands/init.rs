use std::{error::Error, fs};

use clap::ArgMatches;

use crate::{models::repository::BlueprintRepository, utils::prompt::prompt_for_value};

pub fn handler(arg_matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let force = arg_matches.get_flag("FORCE");

    match BlueprintRepository::new().to_pathbuf() {
        Ok(path) => {
            if path.exists() && force {
                let response = prompt_for_value(format!("This will remove the directory located at {:?} and all files within. Proceed? y/n:", path));

                if response.to_lowercase() == "y" {
                    fs::remove_dir_all(&path).expect("Could not remove .blueprint directory");

                    fs::create_dir(&path).expect("Could not create .blueprint directory");
                } else {
                    eprintln!("Input was not \"y\", aborting");

                    return Ok(());
                }
            } else if path.exists() {
                eprintln!(
                    "{:?} directory already exists, skipping initialization steps. If you want to force a fresh install, pass the `--force` flag to this command.",
                    path.as_os_str()
                )
            } else {
                fs::create_dir(&path).expect("Could not create .blueprint directory");
            }

            println!(
                "Successfully initialized CLI. The following were created:\n* {:?}",
                path
            );

            Ok(())
        }
        Err(e) => Err(Box::new(e)),
    }
}
