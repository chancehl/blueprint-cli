use std::{error::Error, fs, path::PathBuf};

use clap::ArgMatches;

use crate::models::blueprint::Blueprint;

pub fn handler(arg_matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let template = arg_matches
        .get_one::<PathBuf>("TEMPLATE")
        .expect("Template file location required");

    let contents = fs::read_to_string(template).expect("Could not read template file");

    let blueprint_json = serde_json::to_string(&Blueprint {
        name: "".to_owned(),
        dependencies: Some(Vec::new()),
        template: contents,
        tokens: Vec::new(),
    })
    .expect("Could not convert template to JSON");

    todo!();
}
