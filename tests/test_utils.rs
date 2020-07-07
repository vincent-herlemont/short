#![allow(dead_code)]

use cli_integration_test::IntegrationTestEnvironment;

pub const HOME_DIR: &'static str = "home";
pub const HOME_CFG_FILE: &'static str = "home/.short/cfg.yaml";
pub const PROJECT_DIR: &'static str = "project";
pub const PROJECT_CFG_FILE: &'static str = "project/short.yaml";
pub const PROJECT_RUN_FILE: &'static str = "project/run.sh";
pub const PROJECT_ENV_EXAMPLE_1_FILE: &'static str = "project/.example1";
pub const PROJECT_ENV_EXAMPLE_2_FILE: &'static str = "project/.example2";
pub const PROJECT_ENV_DIR: &'static str = "project/env";
pub const PRIVATE_ENV_DIR: &'static str = "private_env";
pub const PRIVATE_ENV_DEV_FILE: &'static str = "private_env/.dev";
pub const TMP_DIR: &'static str = "tmp";

pub fn init<L: AsRef<str>>(label: L) -> IntegrationTestEnvironment {
    let mut e = IntegrationTestEnvironment::new(label);
    e.add_dir(HOME_DIR);
    e.add_dir(PROJECT_DIR);
    e.add_dir(PRIVATE_ENV_DIR);
    e.add_dir(TMP_DIR);
    e.set_cfg_command_callback(|root_path, mut command| {
        command.current_dir(root_path.join(PROJECT_DIR));
        command.env("HOME", root_path.join(HOME_DIR));

        #[cfg(unix)]
        command.env("TMPDIR", root_path.join(TMP_DIR));

        #[cfg(windows)]
        command.env("TMP", &root_path.join(TMP_DIR));

        command
    });
    e
}
