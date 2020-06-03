use crate::env_file::entry::Entry;
use crate::env_file::{Env, Var};

#[derive(Debug)]
pub struct EnvIterator<'a> {
    env: &'a Env,
    index: usize,
}

impl<'a> EnvIterator<'a> {
    pub fn new(env: &'a Env) -> Self {
        Self { env, index: 0 }
    }
}

impl<'a> Iterator for EnvIterator<'a> {
    type Item = &'a Var;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(var) = self.env.entries.get(self.index) {
            self.index += 1;
            if let Entry::Var(var) = var {
                return Some(&var);
            } else {
                return self.next();
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::env_file::Env;

    #[test]
    fn env_iterator() {
        let mut env = Env::new("".into());
        env.add("name1", "value1");
        env.add_empty_line();
        env.add("name2", "value2");

        let mut iter = env.iter();

        if let Some(var) = iter.next() {
            assert_eq!(var.name(), "name1");
            assert_eq!(var.value(), "value1");
        } else {
            assert!(false);
        }

        if let Some(var) = iter.next() {
            assert_eq!(var.name(), "name2");
            assert_eq!(var.value(), "value2");
        } else {
            assert!(false);
        }

        assert!(iter.next().is_none());
    }
}
