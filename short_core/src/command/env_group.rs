use crate::cfg::{EnvGroup, EnvGroups};
use anyhow::{Context, Result};
use regex::Regex;
use short_env::Env;
use std::rc::Rc;

#[derive(Debug)]
pub struct Var {
    pub array: bool,
    pub name: String,
    pub env_name: String,
    pub env_value: String,
}

pub fn generate_var(env: &Env, env_group: &EnvGroup) -> Result<Var> {
    let (var_name, pattern) = env_group;
    let re = Regex::new(pattern.as_str())?;
    let mut var = Var {
        array: true,
        name: var_name.clone(),
        env_name: String::new(),
        env_value: String::from(" "),
    };
    for (env_name, env_value) in env.iter() {
        if env_name == *pattern {
            var.env_value = env_value;
            var.env_name = env_name;
            var.array = false;
            break;
        }
        if re.is_match(&env_name) {
            var.env_value = format!("{}[{}]='{}' ", var.env_value.clone(), &env_name, &env_value);
        }
    }
    if var.array {
        var.env_name = var.name.to_uppercase()
    }
    Ok(var)
}

pub fn generate_vars(env: &Env, env_groups: &EnvGroups) -> Result<Vec<Var>> {
    let mut vec = vec![];
    for env_group in env_groups.inner().borrow().iter() {
        vec.append(&mut vec![generate_var(env, env_group)?])
    }
    Ok(vec)
}

#[cfg(test)]
mod tests {
    use crate::cfg::{EnvGroup, EnvGroups};
    use crate::command::{generate_var, generate_vars};
    use short_env::Env;

    #[test]
    fn generate_var_test() {
        let env_group: EnvGroup = ("all".into(), ".*".into());
        let mut env_file = Env::new();
        env_file.add("VAR1", "VALUE1");
        env_file.add("VAR2", "VALUE2");

        let var = generate_var(&env_file, &env_group).unwrap();
        assert_eq!(var.name, "all");
        assert_eq!(var.env_value, " [VAR1]='VALUE1' [VAR2]='VALUE2' ");
        assert_eq!(var.array, true);
    }

    #[test]
    fn generate_vars_test() {
        let mut env_groups = EnvGroups::new();
        env_groups.add("all".into(), ".*".into());
        env_groups.add("var1".into(), "VAR1".into());
        let mut env_file = Env::new();
        env_file.add("VAR1", "VALUE1");
        env_file.add("VAR2", "VALUE2");

        let vars = generate_vars(&env_file, &env_groups).unwrap();
        assert_eq!(vars.len(), 2);
    }
}
