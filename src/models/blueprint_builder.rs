use super::{blueprint::Blueprint, error::Error};

#[derive(Default)]
pub struct BlueprintBuilder {
    name: Option<String>,
    template: Option<String>,
    tokens: Option<Vec<String>>,
    dependencies: Option<Vec<String>>,
    file_name: Option<String>,
}

impl BlueprintBuilder {
    pub fn new() -> Self {
        BlueprintBuilder::default()
    }

    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.name = Some(name);

        self
    }

    pub fn set_template(&mut self, template: String) -> &mut Self {
        self.template = Some(template);

        self
    }

    pub fn set_tokens(&mut self, tokens: Vec<String>) -> &mut Self {
        self.tokens = Some(tokens);

        self
    }

    // pub fn set_dependencies(&mut self, dependencies: Option<Vec<String>>) -> &mut Self {
    //     self.dependencies = dependencies;

    //     self
    // }

    pub fn set_file_name(&mut self, file_name: String) -> &mut Self {
        self.file_name = Some(file_name);

        self
    }

    pub fn build(&mut self) -> Result<Blueprint, Error> {
        let Some(name) = &self.name else {
            return Err(Error::Static("Missing questions"));
        };

        let Some(template) = &self.template else {
            return Err(Error::Static("Missing template"));
        };

        let Some(tokens) = &self.tokens else {
            return Err(Error::Static("Missing tokens"));
        };

        let Some(file_name) = &self.file_name else {
            return Err(Error::Static("Missing file_name"));
        };

        Ok(Blueprint {
            name: name.to_string(),
            template: template.to_string(),
            tokens: tokens.to_vec(),
            dependencies: self.dependencies.to_owned(),
            file_name: file_name.to_owned(),
        })
    }
}
