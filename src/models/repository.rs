use std::{
    env::{self, VarError},
    path::PathBuf,
};

pub enum RepositoryType {
    LOCAL,
    _REMOTE,
}

pub struct BlueprintRepository {
    _kind: RepositoryType,
    _root: String,
}

impl BlueprintRepository {
    pub fn new() -> Self {
        BlueprintRepository {
            _kind: RepositoryType::LOCAL, // always hardcode this to local for now
            _root: ".blueprint".to_string(),
        }
    }

    pub fn to_pathbuf(&self) -> Result<PathBuf, VarError> {
        match self.to_str() {
            Ok(loc) => Ok(PathBuf::from(loc)),
            Err(e) => return Err(e),
        }
    }

    pub fn to_str(&self) -> Result<String, VarError> {
        match env::var("HOME") {
            Ok(home) => Ok(format!("{}/{}", home, ".blueprint")),
            Err(e) => Err(e),
        }
    }
}
