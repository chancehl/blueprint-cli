use std::{fs, path::PathBuf};

pub struct Blueprint {
    contents: String,
    tokens: Vec<String>,
}

impl From<&PathBuf> for Blueprint {
    fn from(value: &PathBuf) -> Self {
        let contents = fs::read_to_string(value).expect("Could not read Blueprint file");

        println!("contents={:?}", contents);

        todo!()
    }
}
