use crate::cfg::{ArrayVar, ArrayVars, SetupCfg, Var, Vars};
use crate::env_file::Env;
use anyhow::Result;
use regex::Regex;
use std::ops::Deref;

type EnvValue = String;

#[derive(Debug)]
pub struct EnvVar(Var, EnvValue);

impl EnvVar {
    pub fn var_name(&self) -> &Var {
        &self.0
    }
    pub fn env_value(&self) -> &EnvValue {
        &self.1
    }
}

impl From<(Var, EnvValue)> for EnvVar {
    fn from(t: (Var, EnvValue)) -> Self {
        Self(t.0, t.1)
    }
}

pub fn generate_array_env_var(env: &Env, array_var: &ArrayVar) -> Result<EnvVar> {
    let mut env_value_buf = " ".to_string();
    let re = Regex::new(array_var.pattern().as_str())?;
    for (env_name, env_value) in env.iter() {
        if re.is_match(&env_name) {
            env_value_buf = format!("{}[{}]='{}' ", env_value_buf.clone(), &env_name, &env_value);
        }
    }
    Ok((array_var.var_name().clone(), env_value_buf).into())
}

pub fn generate_env_var(env: &Env, var: &Var) -> EnvVar {
    env.iter()
        .find_map(|(env_name, env_value)| {
            if env_name == var.to_env_var() {
                Some((var.clone(), env_value).into())
            } else {
                None
            }
        })
        .map_or((var.clone(), "".to_string()).into(), |e| e)
}

pub fn generate_env_vars<AV, V>(env: &Env, array_vars: AV, vars: V) -> Result<Vec<EnvVar>>
where
    AV: Deref<Target = ArrayVars>,
    V: Deref<Target = Vars>,
{
    let mut env_vars: Vec<EnvVar> = vec![];

    for array_var in array_vars.inner().iter() {
        let env_var = generate_array_env_var(env, array_var)?;
        env_vars.append(&mut vec![env_var]);
    }

    for var in vars.inner().iter() {
        let env_var = generate_env_var(env, var);
        env_vars.append(&mut vec![env_var]);
    }

    Ok(env_vars)
}

#[derive(Debug)]
#[deprecated]
pub struct VarLast {
    pub array: bool,
    pub name: String,
    pub env_name: String,
    pub env_value: String,
}

#[deprecated]
pub fn generate_var_last(env: &Env, array_var: &ArrayVar) -> Result<VarLast> {
    let var_name = array_var.var_name();
    let pattern = array_var.pattern();
    let re = Regex::new(pattern.as_str())?;
    let mut var = VarLast {
        array: true,
        name: var_name.to_string(),
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

#[deprecated]
pub fn generate_vars_last(env: &Env, array_vars: &ArrayVars) -> Result<Vec<VarLast>> {
    let mut vec = vec![];
    for array_var in array_vars.inner().iter() {
        vec.append(&mut vec![generate_var_last(env, array_var)?])
    }
    Ok(vec)
}

#[cfg(test)]
mod tests {
    use crate::cfg::{ArrayVar, ArrayVars, Var};
    use crate::env_file::Env;
    use crate::run_file::var::{generate_array_env_var, generate_env_var};
    use crate::run_file::{generate_var_last, generate_vars_last};

    #[test]
    fn generate_array_var_test() {
        let array_var: ArrayVar = ("all".into(), ".*".into()).into();
        let mut env_file = Env::new();
        env_file.add("VAR1", "VALUE1");
        env_file.add("VAR2", "VALUE2");

        let env_var = generate_array_env_var(&env_file, &array_var).unwrap();
        assert_eq!(env_var.var_name().to_string(), "all");
        assert_eq!(env_var.env_value(), " [VAR1]='VALUE1' [VAR2]='VALUE2' ");
    }

    #[test]
    fn generate_env_var_test() {
        let var: Var = "VAR1".into();

        let mut env_file = Env::new();
        env_file.add("VAR1", "VALUE1");
        env_file.add("VAR2", "VALUE2");

        let env_var = generate_env_var(&env_file, &var);
        assert_eq!(env_var.var_name().to_string(), "VAR1");
        assert_eq!(env_var.env_value(), "VALUE1");
    }

    #[test]
    #[ignore]
    #[deprecated]
    fn generate_var_test_last() {
        let array_var: ArrayVar = ("all".into(), ".*".into()).into();
        let mut env_file = Env::new();
        env_file.add("VAR1", "VALUE1");
        env_file.add("VAR2", "VALUE2");

        let var = generate_var_last(&env_file, &array_var).unwrap();
        assert_eq!(var.name, "all");
        assert_eq!(var.env_value, " [VAR1]='VALUE1' [VAR2]='VALUE2' ");
        assert_eq!(var.array, true);
    }

    #[test]
    #[ignore]
    #[deprecated]
    fn generate_vars_test_last() {
        let mut array_vars = ArrayVars::new();
        array_vars.add("all".into(), ".*".into());
        array_vars.add("var1".into(), "VAR1".into());
        let mut env_file = Env::new();
        env_file.add("VAR1", "VALUE1");
        env_file.add("VAR2", "VALUE2");

        let vars = generate_vars_last(&env_file, &array_vars).unwrap();
        assert_eq!(vars.len(), 2);
    }
}
