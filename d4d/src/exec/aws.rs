use crate::exec::{Runner, Software};

struct Aws {
    software: Software,
}

impl Aws {
    pub fn version(mut self) -> Runner {
        self.software.arg("--version");
        self.software.runner()
    }
}

#[cfg(test)]
mod tests {
    use crate::exec::aws::Aws;
    use crate::exec::Software;

    fn new_fake_aws() -> Aws {
        Aws {
            software: Software::fake("aws"),
        }
    }

    #[test]
    fn version() {
        let aws = new_fake_aws();
        let runner = aws.version();
        let args = runner.args();
        assert_eq!(args, &vec!["--version"])
    }
}
