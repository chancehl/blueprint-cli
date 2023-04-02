use std::{env, fs, path::PathBuf};

pub fn handler() -> Result<(), &'static str> {
    let home = env::var("HOME").expect("Could not locate $HOME value");
    let loc = format!("{}/.blueprint", home);

    match PathBuf::from(&loc).exists() {
        true => eprintln!(".blueprint directory already exists at {}", loc),
        false => fs::create_dir(&loc).expect("Could not create .blueprint directory"),
    };

    Ok(())
}
