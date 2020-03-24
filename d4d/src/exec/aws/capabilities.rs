use std::collections::HashSet;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::hash::Hash;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum Capability {
    CAPABILITY_IAM,
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
    pub fn new(list: &[Capability]) -> Self {
        Self {
            list: list.iter().cloned().collect(),
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
}

#[cfg(test)]
mod tests {
    use crate::exec::aws::capabilities::{Capabilities, Capability};

    #[test]
    fn to_string() {
        let capabilities = Capabilities::new(&[]);
        assert!(capabilities.to_strings().is_none());

        let capabilities = Capabilities::new(&[
            Capability::CAPABILITY_IAM,
            Capability::CAPABILITY_NAMED_IAM,
            Capability::CAPABILITY_IAM,
        ]);
        if let Some(capabilities) = capabilities.to_strings() {
            assert_eq!(capabilities, &["CAPABILITY_IAM", "CAPABILITY_NAMED_IAM"]);
        } else {
            assert!(false);
        }
    }
}
