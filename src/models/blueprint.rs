use serde::{Deserialize, Serialize};
use std::{
    env,
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use crate::utils::prompt::prompt_for_value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Blueprint {
    /// The name of the blueprint
    pub name: String,

    /// The template itself
    pub template: String,

    /// The tokens the CLI will prompt you for
    pub tokens: Vec<String>,

    /// Other blueprints to be included when this one is executed
    pub dependencies: Option<Vec<String>>,

    /// The name of the file to create when .execute() is called
    pub file_name: String,
}

impl From<&PathBuf> for Blueprint {
    fn from(value: &PathBuf) -> Self {
        let contents = fs::read_to_string(value).expect("Could not read Blueprint file");

        serde_json::from_str::<Blueprint>(&contents)
            .expect("Could not convert .json file to Blueprint")
    }
}

impl Blueprint {
    /// Searches for a blueprint with a given name in the $HOME/.blueprint directory
    pub fn seek(name: String) -> Option<Blueprint> {
        let blueprint_directory = PathBuf::from(format!(
            "{}/.blueprint",
            env::var("HOME").expect("Could not locate $HOME value")
        ));

        let local_templates =
            fs::read_dir(blueprint_directory).expect("Could not read $HOME/.blueprint dir");

        for template in local_templates {
            let contents = &fs::read_to_string(template.unwrap().path())
                .expect("Could not read template file");

            let blueprint_json = serde_json::from_str::<Blueprint>(&contents);

            if let Ok(blueprint) = blueprint_json {
                if blueprint.name == name {
                    return Some(blueprint);
                }
            };
        }

        None
    }

    /// Executes the blueprint, asking for template variables and writing to disk or console
    pub fn execute(&mut self, destination: Option<&PathBuf>) -> Result<(), &'static str> {
        // generate template
        for token in &self.tokens {
            let value = prompt_for_value(format!("Enter value for token {}:", token));

            self.template = self.template.replace(token, &value);
            self.file_name = self.file_name.replace(token, &value);
        }

        // execute siblings if they exist
        if let Some(deps) = &self.dependencies {
            for dep in deps {
                if let Some(mut blueprint) = Blueprint::seek(dep.to_string()) {
                    blueprint
                        .execute(destination)
                        .expect("Could not execute blueprint")
                }
            }
        }

        // write file to disk
        self.write_to_disk(destination)
    }

    /// Writes the blueprint's template to disk
    fn write_to_disk(&self, destination: Option<&PathBuf>) -> Result<(), &'static str> {
        let path = if let Some(destination) = destination {
            destination.to_owned()
        } else {
            PathBuf::from("./").join(&self.file_name)
        };

        let mut output =
            File::create(&path).expect(&format!("Could not create output file at {:?}", &path));

        write!(output, "{}", self.template)
            .expect(&format!("Could not write to output file at {:?}", path));

        Ok(())
    }
}
