use std::collections::HashSet;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::hash::Hash;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum Capability {
    #[allow(non_camel_case_types)]
    CAPABILITY_IAM,
    #[allow(non_camel_case_types)]
    CAPABILITY_NAMED_IAM,
}

impl Display for Capability {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Capability::CAPABILITY_IAM => write!(f, "{}", "CAPABILITY_IAM"),
            Capability::CAPABILITY_NAMED_IAM => write!(f, "{}", "CAPABILITY_NAMED_IAM"),
        }
    }
}

#[derive(Debug)]
pub struct Capabilities {
    list: HashSet<Capability>,
}

impl<'a> Capabilities {
    pub fn new() -> Self {
        Self {
            list: HashSet::new(),
        }
    }
    pub fn to_strings(&self) -> Option<Vec<String>> {
        if self.list.is_empty() {
            None
        } else {
            let mut list: Vec<String> = self.list.iter().map(|c| c.to_string()).collect();
            list.sort();
            Some(list)
        }
    }

    pub fn add(&mut self, capability: Capability) {
        self.list.insert(capability);
    }
}

#[cfg(test)]
mod tests {
    use crate::exec::aws::capabilities::{Capabilities, Capability};

    #[test]
    fn to_string() {
        let capabilities = Capabilities::new();
        assert!(capabilities.to_strings().is_none());

        let mut capabilities = Capabilities::new();
        capabilities.add(Capability::CAPABILITY_IAM);
        capabilities.add(Capability::CAPABILITY_NAMED_IAM);
        capabilities.add(Capability::CAPABILITY_IAM);

        if let Some(capabilities) = capabilities.to_strings() {
            assert_eq!(capabilities, &["CAPABILITY_IAM", "CAPABILITY_NAMED_IAM"]);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn add() {
        let mut capabilities = Capabilities::new();
        capabilities.add(Capability::CAPABILITY_NAMED_IAM);
        if let Some(capabilities) = capabilities.to_strings() {
            assert_eq!(capabilities, &["CAPABILITY_NAMED_IAM"]);
        } else {
            assert!(false);
        }
    }
}
