use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Var(String);

impl Var {
    pub fn to_var(&self) -> String {
        self.to_string().to_lowercase()
    }

    pub fn to_env_var(&self) -> String {
        self.to_string().to_uppercase()
    }
}

impl ToString for Var {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl From<String> for Var {
    fn from(string: String) -> Self {
        Var(string)
    }
}

impl From<&str> for Var {
    fn from(string: &str) -> Self {
        Var(string.to_string())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Vars(Vec<Var>);

impl Vars {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn add(&mut self, name: Var) {
        if self.0.iter().find(|var| *var == &name).is_none() {
            self.0.append(&mut vec![name])
        }
    }
}

impl AsRef<Vec<Var>> for Vars {
    fn as_ref(&self) -> &Vec<Var> {
        &self.0
    }
}

impl Default for Vars {
    fn default() -> Self {
        Self::new()
    }
}
