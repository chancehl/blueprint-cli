use std::{env, fs, path::PathBuf};

pub fn handler() -> Result<(), &'static str> {
    let home = env::var("HOME").expect("Could not locate $HOME value");
    let loc = format!("{}/.blueprint", home);

    if PathBuf::from(&loc).exists() {
        eprintln!(
            "{} directory already exists, skipping initialization steps. If you want to force a fresh install, pass the `--force` flag to this command.",
            loc
        )
    } else {
        fs::create_dir(&loc).expect("Could not create .blueprint directory");

        println!(
            "Successfully initialized CLI. The following were created:\n* {:?}",
            loc.to_string()
        )
    }

    Ok(())
}
