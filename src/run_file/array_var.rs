use crate::cfg::{ArrayVar, ArrayVars};
use crate::env_file::Env;
use anyhow::Result;
use regex::Regex;

#[derive(Debug)]
pub struct Var {
    pub array: bool,
    pub name: String,
    pub env_name: String,
    pub env_value: String,
}

pub fn generate_var(env: &Env, array_var: &ArrayVar) -> Result<Var> {
    let (var_name, pattern) = array_var;
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

pub fn generate_vars(env: &Env, array_vars: &ArrayVars) -> Result<Vec<Var>> {
    let mut vec = vec![];
    for array_var in array_vars.inner().borrow().iter() {
        vec.append(&mut vec![generate_var(env, array_var)?])
    }
    Ok(vec)
}

#[cfg(test)]
mod tests {
    use crate::cfg::{ArrayVar, ArrayVars};
    use crate::env_file::Env;
    use crate::run_file::{generate_var, generate_vars};

    #[test]
    fn generate_var_test() {
        let array_var: ArrayVar = ("all".into(), ".*".into());
        let mut env_file = Env::new();
        env_file.add("VAR1", "VALUE1");
        env_file.add("VAR2", "VALUE2");

        let var = generate_var(&env_file, &array_var).unwrap();
        assert_eq!(var.name, "all");
        assert_eq!(var.env_value, " [VAR1]='VALUE1' [VAR2]='VALUE2' ");
        assert_eq!(var.array, true);
    }

    #[test]
    fn generate_vars_test() {
        let mut array_vars = ArrayVars::new();
        array_vars.add("all".into(), ".*".into());
        array_vars.add("var1".into(), "VAR1".into());
        let mut env_file = Env::new();
        env_file.add("VAR1", "VALUE1");
        env_file.add("VAR2", "VALUE2");

        let vars = generate_vars(&env_file, &array_vars).unwrap();
        assert_eq!(vars.len(), 2);
    }
}
