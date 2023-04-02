use std::{env::VarError, fs};

use crate::utils::blueprint_dir;

pub fn handler() -> Result<(), VarError> {
    match blueprint_dir::as_pathbuf() {
        Ok(path) => {
            if path.exists() {
                eprintln!(
                    "{:?} directory already exists, skipping initialization steps. If you want to force a fresh install, pass the `--force` flag to this command.",
                    path.as_os_str()
                )
            } else {
                fs::create_dir(&path).expect("Could not create .blueprint directory");

                println!(
                    "Successfully initialized CLI. The following were created:\n* {:?}",
                    path.as_os_str()
                )
            }

            Ok(())
        }
        Err(e) => Err(e),
    }
}
