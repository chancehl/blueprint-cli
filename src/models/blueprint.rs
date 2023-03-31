use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::{
    env,
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Blueprint {
    template: String,
    name: String,
    tokens: Vec<String>,
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

    pub fn execute(mut self, loc: Option<&PathBuf>) -> Result<()> {
        // generate template
        for token in &self.tokens {
            let mut line = String::new();

            println!("Enter value for {}:", token);

            std::io::stdin().read_line(&mut line).unwrap();

            let value = line
                .strip_suffix("\n")
                .unwrap_or(&line.to_string())
                .to_owned();

            self.template = self.template.replace(token, &value);
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
