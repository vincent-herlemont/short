use crate::env_file::{Comment, Var};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Entry {
    Var(Var),
    Comment(Comment),
    Empty,
}

impl Display for Entry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Entry::Var(var) => write!(f, "{}", var),
            Entry::Comment(comment) => write!(f, "{}", comment),
            Entry::Empty => writeln!(f, ""),
        }
    }
}
