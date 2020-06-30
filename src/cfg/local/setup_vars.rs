use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct VarName(String);

impl VarName {
    pub fn new(var: String) -> Self {
        Self(var)
    }

    pub fn to_var(&self) -> String {
        self.to_string().to_lowercase()
    }

    pub fn to_env_var(&self) -> String {
        self.to_string().to_uppercase()
    }
}

impl ToString for VarName {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl From<String> for VarName {
    fn from(string: String) -> Self {
        VarName(string)
    }
}

impl From<&str> for VarName {
    fn from(string: &str) -> Self {
        VarName(string.to_string())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Vars(Vec<VarName>);

impl Vars {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn add(&mut self, name: VarName) {
        if self.0.iter().find(|var| *var == &name).is_none() {
            self.0.append(&mut vec![name])
        }
    }
}

impl AsRef<Vec<VarName>> for Vars {
    fn as_ref(&self) -> &Vec<VarName> {
        &self.0
    }
}

impl Default for Vars {
    fn default() -> Self {
        Self::new()
    }
}
