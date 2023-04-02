use std::{
    env::{self, VarError},
    path::PathBuf,
};

const BLUEPRINT_DIR: &str = ".blueprint";

pub fn as_pathbuf() -> Result<PathBuf, VarError> {
    match as_str() {
        Ok(loc) => Ok(PathBuf::from(loc)),
        Err(e) => return Err(e),
    }
}

pub fn as_str() -> Result<String, VarError> {
    match env::var("HOME") {
        Ok(home) => Ok(format!("{}/{}", home, BLUEPRINT_DIR)),
        Err(e) => return Err(e),
    }
}
