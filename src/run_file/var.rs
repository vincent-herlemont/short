use crate::cfg::{ArrayVar, ArrayVars, Var, Vars};
use crate::env_file::Env;
use anyhow::Result;
use regex::Regex;
use std::ops::Deref;

type EnvValue = String;

#[derive(Debug)]
pub struct EnvVar(Var, EnvValue);

impl EnvVar {
    pub fn var(&self) -> &Var {
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
    Ok((array_var.var().clone(), env_value_buf).into())
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

#[cfg(test)]
mod tests {
    use crate::cfg::{ArrayVar, Var};
    use crate::env_file::Env;
    use crate::run_file::var::{generate_array_env_var, generate_env_var};

    #[test]
    fn generate_array_var_test() {
        let array_var: ArrayVar = ("all".into(), ".*".into()).into();
        let mut env_file = Env::new();
        env_file.add("VAR1", "VALUE1");
        env_file.add("VAR2", "VALUE2");

        let env_var = generate_array_env_var(&env_file, &array_var).unwrap();
        assert_eq!(env_var.var().to_string(), "all");
        assert_eq!(env_var.env_value(), " [VAR1]='VALUE1' [VAR2]='VALUE2' ");
    }

    #[test]
    fn generate_env_var_test() {
        let var: Var = "VAR1".into();

        let mut env_file = Env::new();
        env_file.add("VAR1", "VALUE1");
        env_file.add("VAR2", "VALUE2");

        let env_var = generate_env_var(&env_file, &var);
        assert_eq!(env_var.var().to_string(), "VAR1");
        assert_eq!(env_var.env_value(), "VALUE1");
    }
}
