use std::{error::Error, fs, path::PathBuf};

use clap::ArgMatches;

use crate::models::{blueprint::Blueprint, repository::BlueprintRepository};

pub fn handler(arg_matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let template = arg_matches
        .get_one::<PathBuf>("TEMPLATE")
        .expect("Template file location required");

    let name = arg_matches
        .get_one::<String>("NAME")
        .expect("Name required");

    let contents = fs::read_to_string(template).expect("Could not read template file");

    let blueprint_json = serde_json::to_string(&Blueprint {
        name: name.to_string(),
        template: contents,
        tokens: Vec::new(),
        dependencies: Some(Vec::new()),
        file_name: template
            .file_name()
            .expect("Could not locate file name")
            .to_os_string()
            .into_string()
            .unwrap(),
    })
    .expect("Could not convert template to json");

    let blueprint_repo = BlueprintRepository::new()
        .to_pathbuf()
        .expect("Could not find blueprint repository");

    let output = blueprint_repo.join(format!("{}.json", name.clone()));

    fs::write(&output, blueprint_json).expect(&format!("Could not write to {:?}", &output));

    println!("Saved blueprint {} to {:?}", name.clone(), &output);

    Ok(())
}
