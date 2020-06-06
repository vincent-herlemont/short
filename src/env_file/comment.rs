use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Comment {
    value: String,
}

impl Display for Comment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "#{}", self.value)
    }
}

impl Comment {
    pub fn from_line(line: &String) -> Option<Self> {
        let parts: Vec<&str> = line.splitn(2, "#").collect();
        match parts.as_slice() {
            [empty, comment] if empty.is_empty() => Some(Self {
                value: String::from(comment.to_owned()),
            }),
            _ => None,
        }
    }
}
