use predicates::prelude::Predicate;
use predicates::str::contains;
use short_utils::integration_test;
use short_utils::integration_test::environment::IntegrationTestEnvironment;

#[test]
fn cmd_help() {
    let e = IntegrationTestEnvironment::new("cmd_help");
    let mut command = e.command(env!("CARGO_PKG_NAME"));
    let r = command.assert();
    r.stderr(
        r#" short 0.0.2
Vincent Herlemont <vincentherl@leszeros.com>

USAGE:
    short [FLAGS] [OPTIONS] [SUBCOMMAND]

FLAGS:
        --dry-run    Disable all executions
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -s, --setup <setup>        Set up name
    -e, --env <environment>    Environment name

SUBCOMMANDS:
    init      Create an empty "short.yml" configuration file
    add       Add set up provider (cloudformation,...)
    env       Manage environment
    deploy    Deploy your set up
    show      Show your current set up
    use       Switch of current setup or/and environment
    ls        List set up and environments
    help      Prints this message or the help of the given subcommand(s)
"#,
    );
}

#[test]
fn cmd_help_add() {
    let e = IntegrationTestEnvironment::new("cmd_help");
    let mut command = e.command(env!("CARGO_PKG_NAME"));
    let r = command.arg("add").assert();
    r.stderr(
        r#"short-add 
Add set up provider (cloudformation,...)

USAGE:
    short add [FLAGS] [OPTIONS] <SUBCOMMAND>

FLAGS:
        --dry-run    Disable all executions
    -h, --help       Prints help information

OPTIONS:
    -s, --setup <setup>        Set up name
    -e, --env <environment>    Environment name

SUBCOMMANDS:
    cloudformation    Add cloudformation setup
    help              Prints this message or the help of the given subcommand(s)
"#,
    );
}

#[test]
fn cmd_help_env() {
    let e = IntegrationTestEnvironment::new("cmd_help");
    let mut command = e.command(env!("CARGO_PKG_NAME"));
    let r = command.arg("env").assert();
    r.stderr(
        r#"short-env 
Manage environment

USAGE:
    short env [FLAGS] [OPTIONS] <SUBCOMMAND>

FLAGS:
        --dry-run    Disable all executions
    -h, --help       Prints help information

OPTIONS:
    -s, --setup <setup>        Set up name
    -e, --env <environment>    Environment name

SUBCOMMANDS:
    new     Add new environment
    dir     Change env directory
    pdir    Add or change private env directory
    help    Prints this message or the help of the given subcommand(s)
"#,
    );
}