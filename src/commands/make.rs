use std::{error::Error, fs, path::PathBuf};

use clap::ArgMatches;
use colored::Colorize;

use crate::models::{
    blueprint::Blueprint,
    repository::{BlueprintRepository, RepositoryType},
};

pub fn handler(arg_matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let file = arg_matches
        .get_one::<PathBuf>("FILE")
        .expect("Template file location required");

    let name = arg_matches
        .get_one::<String>("NAME")
        .expect("Missing --name flag");

    let contents = fs::read_to_string(file).expect("Could not read template file");

    let json = serde_json::to_string(&Blueprint {
        name: name.to_string(),
        template: contents,
        tokens: Vec::new(),
        dependencies: Some(Vec::new()),
        file_name: file
            .file_name()
            .expect("Could not locate file name")
            .to_os_string()
            .into_string()
            .unwrap(),
    })
    .expect("Could not convert template to json");

    let blueprint_repo = BlueprintRepository::new(RepositoryType::LOCAL)
        .to_pathbuf()
        .expect("Could not find blueprint repository");

    let destination = blueprint_repo.join(format!("{}.json", name));

    fs::write(&destination, json).expect(&format!("Could not write to {:?}", destination));

    println!(
        "{} Successfully made blueprint from file {:?}",
        "✔".green(),
        file
    );

    Ok(())
}
