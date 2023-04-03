use std::{error::Error, fs, path::PathBuf};

use clap::ArgMatches;

use crate::{
    models::{blueprint::Blueprint, repository::BlueprintRepository},
    utils::prompt::prompt_for_value,
};

pub fn handler(arg_matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let template = arg_matches
        .get_one::<PathBuf>("TEMPLATE")
        .expect("Template file location required");

    let contents = fs::read_to_string(template).expect("Could not read template file");

    let blueprint_name = prompt_for_value("Enter name for blueprint file:".to_owned());

    let blueprint_json = serde_json::to_string(&Blueprint {
        name: blueprint_name.clone(),
        dependencies: Some(Vec::new()),
        template: contents,
        tokens: Vec::new(),
    })
    .expect("Could not convert template to json");

    let blueprint_repo = BlueprintRepository::new()
        .to_pathbuf()
        .expect("Could not find blueprint repository");

    let output = blueprint_repo.join(format!("{}.json", blueprint_name.clone()));

    fs::write(&output, blueprint_json).expect(&format!("Could not write to {:?}", &output));

    println!(
        "Saved blueprint {} to {:?}",
        blueprint_name.clone(),
        &output
    );

    Ok(())
}
