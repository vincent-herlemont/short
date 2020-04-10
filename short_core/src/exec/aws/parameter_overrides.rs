
use crate::exec::aws::workflow::{
    ENV_AWS_CAPABILITY_IAM, ENV_AWS_CAPABILITY_NAMED_IAM, ENV_AWS_REGION, ENV_AWS_S3_BUCKET_DEPLOY,
};
use env::Env;

use voca_rs::case::pascal_case;

pub struct ParameterOverrides<'e, 'sn> {
    env: &'e Env,
    stack_name: &'sn String,
}

impl<'e, 'sn> ParameterOverrides<'e, 'sn> {
    pub fn new(env: &'e Env, stack_name: &'sn String) -> Self {
        Self { env, stack_name }
    }

    pub fn args(&self) -> Vec<String> {
        let mut out = vec!["--parameter-overrides".to_string()];
        let mut vars: Vec<_> = self
            .env
            .iter()
            .filter(|(name, _value)| {
                if name == ENV_AWS_REGION
                    || name == ENV_AWS_S3_BUCKET_DEPLOY
                    || name == ENV_AWS_CAPABILITY_IAM
                    || name == ENV_AWS_CAPABILITY_NAMED_IAM
                {
                    false
                } else {
                    true
                }
            })
            .map(|(name, value)| format!("{}={}", pascal_case(name.as_str()), value))
            .collect();
        vars.append(&mut vec![format!("StackName={}", self.stack_name)]);
        vars.sort();
        out.append(&mut vars);
        out
    }
}

#[cfg(test)]
mod tests {
    use crate::exec::aws::parameter_overrides::ParameterOverrides;
    use crate::exec::aws::workflow::ENV_AWS_REGION;
    use env::Env;

    #[test]
    fn display() {
        let mut env = Env::new();
        env.add("VAR1", "1");
        env.add("VAR2", "2");
        env.add(ENV_AWS_REGION, "aws-region");
        let stack_name = String::from("the-stack-name");
        let parameter_overrides = ParameterOverrides::new(&env, &stack_name);
        let args = parameter_overrides.args();
        assert_eq!(
            args,
            vec![
                "--parameter-overrides",
                "StackName=the-stack-name",
                "Var1=1",
                "Var2=2"
            ]
        );
    }
}
