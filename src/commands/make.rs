use std::{error::Error, fs, path::PathBuf};

use clap::ArgMatches;
use colored::Colorize;

use crate::models::{
    blueprint_builder::BlueprintBuilder,
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

    let file_name = file
        .file_name()
        .expect("Could not locate file name")
        .to_os_string()
        .into_string()
        .unwrap();

    let blueprint = BlueprintBuilder::new()
        .set_name(name.to_string())
        .set_template(contents)
        .set_tokens(Vec::new())
        .set_file_name(file_name)
        .build()
        .expect("Could not create blueprint object");

    let json = serde_json::to_string(&blueprint).expect("Could not convert template to json");

    let blueprint_repo = BlueprintRepository::new(RepositoryType::Local)
        .to_pathbuf()
        .expect("Could not find blueprint repository");

    let destination = blueprint_repo.join(format!("{}.json", name));

    fs::write(&destination, json)
        .unwrap_or_else(|_| panic!("Could not write to {:?}", destination));

    println!(
        "{} Successfully made blueprint from file {:?}",
        "✔".green(),
        file
    );

    Ok(())
}
