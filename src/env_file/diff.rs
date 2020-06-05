use crate::env_file::entry::Entry;
use crate::env_file::{Env, Var};
use std::borrow::Cow;
use std::env::var;
use std::ops::Index;

pub struct EnvDiffController {
    update_var_fn: Box<dyn Fn(&mut Var) -> Cow<Var>>,
    delete_var_fn: Box<dyn Fn(&Var) -> bool>,
}

impl EnvDiffController {
    pub fn new<UVF: 'static, DVF: 'static>(update_var: UVF, delete_var: DVF) -> Self
    where
        UVF: Fn(&mut Var) -> Cow<Var>,
        DVF: Fn(&Var) -> bool,
    {
        Self {
            update_var_fn: Box::new(update_var),
            delete_var_fn: Box::new(delete_var),
        }
    }

    fn update_var<'a>(&self, var: &'a mut Var) -> Cow<'a, Var> {
        (&self.update_var_fn)(var)
    }

    fn delete_var(&self, var: &Var) -> bool {
        (&self.delete_var_fn)(var)
    }
}

type IndexEntry = usize;

impl Env {
    pub fn update_by_diff(&mut self, source_env: &Env, env_diff: &EnvDiffController) {
        let mut source_entries = source_env.entries.clone();
        // Prevent delete vars.
        // Append target entry to source entry if delete control return false.
        // In this way we prevent the delete of the the variable.
        for (index, target_entry) in self.entries.iter().enumerate() {
            if let None = source_env
                .entries
                .iter()
                .find(|entry| *entry == target_entry)
            {
                if let Entry::Var(var) = target_entry {
                    if !env_diff.delete_var(var) {
                        source_entries.insert(index, target_entry.clone());
                    }
                }
            }
        }

        let mut new_entries = vec![];
        // Delete vars : Don't append in new_entries var that not present in source_entries.
        // Update vars : Vars can be update via the update control.
        for source_entry in source_entries.iter() {
            if let Some(target_entry) = self.entries.iter().find(|entry| *entry == source_entry) {
                new_entries.push(target_entry.clone());
            } else {
                let source_entry = source_entry.clone();
                if let Entry::Var(var) = source_entry {
                    let mut var = var;
                    let update_var = env_diff.update_var(&mut var).into_owned();
                    new_entries.push(Entry::Var(update_var));
                } else {
                    new_entries.push(source_entry);
                }
            }
        }
        self.entries = new_entries;
    }
}

#[cfg(test)]
mod tests {
    use crate::env_file::diff::EnvDiffController;
    use crate::env_file::Env;
    use predicates::prelude::*;
    use std::borrow::Cow;

    #[test]
    fn update_by_diff_add_var() {
        let mut env_source = Env::new("".into());
        env_source.add("name1", "value1");

        let mut env_target = Env::new("".into());
        let controller = EnvDiffController::new(|var| Cow::Borrowed(var), |var| true);
        env_target.update_by_diff(&env_source, &controller);

        let mut env_expected = Env::new("".into());
        env_expected.add("name1", "value1");
        assert_eq!(env_expected.to_string(), env_target.to_string());
    }

    #[test]
    fn update_by_diff_add_altered_var() {
        let mut env_source = Env::new("".into());
        env_source.add("name1", "value1");

        let mut env_target = Env::new("".into());
        let controller = EnvDiffController::new(
            |var| {
                var.set_value("value1.1");
                Cow::Borrowed(var)
            },
            |var| true,
        );
        env_target.update_by_diff(&env_source, &controller);

        let mut env_expected = Env::new("".into());
        env_expected.add("name1", "value1.1");
        assert_eq!(env_expected.to_string(), env_target.to_string());
    }

    #[test]
    fn update_by_diff_replace_var() {
        let mut env_source = Env::new("".into());
        env_source.add("name1", "value1");

        let mut env_target = Env::new("".into());
        env_target.add("name1", "value1.1");
        let controller = EnvDiffController::new(|v| Cow::Borrowed(v), |_| true);
        env_target.update_by_diff(&env_source, &controller);

        let mut env_expected = Env::new("".into());
        env_expected.add("name1", "value1.1");
        assert_eq!(env_expected.to_string(), env_target.to_string());
    }

    #[test]
    fn update_by_diff_delete_var_true() {
        let mut env_source = Env::new("".into());

        let mut env_target = Env::new("".into());
        env_target.add("name1", "value1.1");
        let controller = EnvDiffController::new(|v| Cow::Borrowed(v), |_| true);
        env_target.update_by_diff(&env_source, &controller);

        let mut env_expected = Env::new("".into());
        assert_eq!(env_expected.to_string(), env_target.to_string());
    }

    #[test]
    fn update_by_diff_delete_var_false() {
        let mut env_source = Env::new("".into());

        let mut env_target = Env::new("".into());
        env_target.add("name1", "value1.1");
        let controller = EnvDiffController::new(|v| Cow::Borrowed(v), |_| false);
        env_target.update_by_diff(&env_source, &controller);

        let mut env_expected = Env::new("".into());
        env_expected.add("name1", "value1.1");
        assert_eq!(env_expected.to_string(), env_target.to_string());
    }
}
