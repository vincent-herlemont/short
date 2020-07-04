use std::ops::Deref;

use anyhow::Result;
use regex::Regex;

use crate::cfg::{ArrayVar, ArrayVars, Setup, VarCase, VarName, Vars};
use crate::env_file;
use crate::env_file::Env;
use heck::*;

#[derive(Debug)]
pub enum EnvValue {
    Var(env_file::Var),
    ArrayVar((ArrayVar, Vec<env_file::Var>)),
}

impl ToString for EnvValue {
    fn to_string(&self) -> String {
        match self {
            EnvValue::Var(value) => value.value().clone(),
            EnvValue::ArrayVar((array_var, array_var_value)) => {
                let mut env_value_buf = " ".to_string();
                for var in array_var_value.iter() {
                    env_value_buf = format!(
                        "{}[{}]='{}' ",
                        env_value_buf.clone(),
                        match array_var.case() {
                            VarCase::CamelCase => var.name().to_camel_case(),
                            VarCase::KebabCase => var.name().to_kebab_case(),
                            VarCase::SnakeCase => var.name().to_snake_case(),
                            VarCase::ShoutySnakeCase => var.name().to_shouty_snake_case(),
                            VarCase::MixedCase => var.name().to_mixed_case(),
                            VarCase::TitleCase => var.name().to_title_case(),
                            VarCase::None => var.name().to_owned(),
                        },
                        var.value()
                    );
                }
                env_value_buf
            }
        }
    }
}

#[derive(Debug)]
pub struct EnvVar(VarName, EnvValue);

const ENV_ENVIRONMENT_VAR: &'static str = "short_env";
const ENV_SETUP_VAR: &'static str = "short_setup";

impl EnvVar {
    pub fn var(&self) -> &VarName {
        &self.0
    }
    pub fn env_value(&self) -> &EnvValue {
        &self.1
    }

    pub fn from_env(env: &Env) -> Result<Self> {
        let var = VarName::new(ENV_ENVIRONMENT_VAR.to_string());
        let env_var = env_file::Var::new(ENV_SETUP_VAR, env.name()?);
        Ok(EnvVar(var, EnvValue::Var(env_var)))
    }

    pub fn from_setup(setup: &Setup) -> Result<Self> {
        let var = VarName::new(ENV_SETUP_VAR.to_string());
        let env_var = env_file::Var::new(ENV_SETUP_VAR, setup.name()?);
        Ok(EnvVar(var, EnvValue::Var(env_var)))
    }
}

impl From<(VarName, EnvValue)> for EnvVar {
    fn from(t: (VarName, EnvValue)) -> Self {
        Self(t.0, t.1)
    }
}

pub fn generate_array_env_var(env: &Env, array_var: &ArrayVar) -> Result<EnvVar> {
    let re = Regex::new(array_var.pattern().as_str())?;
    let mut array_var_value: Vec<env_file::Var> = vec![];
    for var in env.iter() {
        if re.is_match(var.name()) {
            array_var_value.push(var.clone());
        }
    }
    Ok((
        array_var.var().clone(),
        EnvValue::ArrayVar((array_var.clone(), array_var_value)),
    )
        .into())
}

pub fn generate_env_var(env: &Env, var: &VarName) -> EnvVar {
    env.iter()
        .find_map(|env_var| {
            if env_var.name() == &var.to_env_var() {
                Some((var.clone(), EnvValue::Var(env_var.clone())).into())
            } else {
                None
            }
        })
        .map_or(
            (
                var.clone(),
                EnvValue::Var(env_file::Var::new(var.to_string(), "")),
            )
                .into(),
            |e| e,
        )
}

pub fn generate_env_vars<AV, V>(env: &Env, array_vars: AV, vars: V) -> Result<Vec<EnvVar>>
where
    AV: Deref<Target = ArrayVars>,
    V: Deref<Target = Vars>,
{
    let mut env_vars: Vec<EnvVar> = vec![];

    for array_var in array_vars.as_ref().iter() {
        let env_var = generate_array_env_var(env, array_var)?;
        env_vars.append(&mut vec![env_var]);
    }

    for var in vars.as_ref().iter() {
        let env_var = generate_env_var(env, var);
        env_vars.append(&mut vec![env_var]);
    }

    Ok(env_vars)
}

#[cfg(test)]
mod tests {
    use crate::cfg::{ArrayVar, VarCase, VarName};
    use crate::env_file::Env;
    use crate::run_file::var::{generate_array_env_var, generate_env_var};

    #[test]
    fn generate_array_var_test() {
        let array_var: ArrayVar = ArrayVar::new("all".into(), ".*".into(), VarCase::None).into();
        let mut env_file = Env::new("".into());
        env_file.add("VAR1", "VALUE1");
        env_file.add("VAR2", "VALUE2");

        let env_var = generate_array_env_var(&env_file, &array_var).unwrap();
        assert_eq!(env_var.var().to_string(), "all");
        assert_eq!(
            env_var.env_value().to_string(),
            " [VAR1]='VALUE1' [VAR2]='VALUE2' "
        );
    }
    #[test]
    fn generate_array_var_with_format_test() {
        let array_var: ArrayVar =
            ArrayVar::new("all".into(), ".*".into(), VarCase::CamelCase).into();
        let mut env_file = Env::new("".into());
        env_file.add("VAR1", "VALUE1");
        env_file.add("VAR2", "VALUE2");

        let env_var = generate_array_env_var(&env_file, &array_var).unwrap();
        assert_eq!(env_var.var().to_string(), "all");
        assert_eq!(
            env_var.env_value().to_string(),
            " [Var1]='VALUE1' [Var2]='VALUE2' "
        );
    }

    #[test]
    fn generate_env_var_test() {
        let var: VarName = "VAR1".into();

        let mut env_file = Env::new("".into());
        env_file.add("VAR1", "VALUE1");
        env_file.add("VAR2", "VALUE2");

        let env_var = generate_env_var(&env_file, &var);
        assert_eq!(env_var.var().to_string(), "VAR1");
        assert_eq!(env_var.env_value().to_string(), "VALUE1");
    }
}
