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
}

impl BlueprintRepository {
    /// Initializes a new Blueprint repository
    pub fn new() -> Self {
        BlueprintRepository {
            _kind: RepositoryType::LOCAL, // always hardcode this to local for now
        }
    }

    /// Generates a pathbuf from the blueprint repository
    pub fn to_pathbuf(&self) -> Result<PathBuf, VarError> {
        match self.to_str() {
            Ok(loc) => Ok(PathBuf::from(loc)),
            Err(e) => return Err(e),
        }
    }

    /// Generates a string representing the blueprint repository location
    pub fn to_str(&self) -> Result<String, VarError> {
        match env::var("HOME") {
            Ok(home) => Ok(format!("{}/{}", home, ".blueprint")),
            Err(e) => Err(e),
        }
    }
}
