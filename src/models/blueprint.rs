use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::{
    collections::HashMap,
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
    template: String,

    /// The tokens the CLI will prompt you for
    tokens: Vec<String>,

    /// Other blueprints to be included when this one is executed
    pub dependencies: Option<Vec<String>>,
}

impl From<&PathBuf> for Blueprint {
    fn from(value: &PathBuf) -> Self {
        let contents = fs::read_to_string(value).expect("Could not read Blueprint file");

        println!("contents={:?}", contents);

        todo!()
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
    pub fn execute(mut self, loc: Option<&PathBuf>) -> Result<()> {
        // generate template
        for token in &self.tokens {
            let value = prompt_for_value(format!("Enter value for token {}:", token));

            self.template = self.template.replace(token, &value);
        }

        // execute siblings if they exist
        if let Some(deps) = self.dependencies {
            for dep in deps {
                if let Some(blueprint) = Blueprint::seek(dep) {
                    blueprint.execute(loc).expect("Could not execute blueprint")
                }
            }
        }

        // write file to disk
        if let Some(path) = loc {
            let mut output =
                File::create(path).expect(&format!("Could not create output file at {:?}", path));

            write!(output, "{}", self.template)
                .expect(&format!("Could not write to output file at {:?}", path));
        } else {
            println!("{:?}", self.template);
        };

        Ok(())
    }
}
