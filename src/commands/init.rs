use std::{error::Error, fs};

use clap::ArgMatches;
use colored::Colorize;

use crate::{
    models::error::Error as CliError,
    models::repository::{BlueprintRepository, RepositoryType},
    utils::prompt::prompt_for_value,
};

type InitializationError = CliError;

pub fn handler(arg_matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let force = arg_matches.get_flag("FORCE");

    match BlueprintRepository::new(RepositoryType::Local).to_pathbuf() {
        Ok(path) => {
            if path.exists() && !force {
                eprintln!(
                    "{:?} directory already exists, skipping initialization steps. If you want to force a fresh install, pass the `--force` flag to this command.",
                    path.as_os_str()
                );

                Err(Box::new(InitializationError::Static(
                    "Directory already exists",
                )))
            } else if path.exists() && force {
                let response = prompt_for_value(format!("{} This will remove local blueprint repository ({}) & all files within. Proceed? y/n:", "Warning!".yellow(), path.to_str().unwrap()));

                if response.to_lowercase() != "y" {
                    eprintln!("Input was not \"y\", aborting");

                    return Ok(());
                }

                fs::remove_dir_all(&path).expect("Could not remove .blueprint directory");

                fs::create_dir(&path).expect("Could not create .blueprint directory");

                println!(
                    "{} Successfully reinitialized {} folder",
                    "✔".green(),
                    path.to_str().unwrap()
                );

                Ok(())
            } else {
                fs::create_dir(&path).expect("Could not create .blueprint directory");

                println!(
                    "{} Successfully created {} folder",
                    "✔".green(),
                    path.to_str().unwrap()
                );

                Ok(())
            }
        }
        Err(e) => Err(Box::new(e)),
    }
}
