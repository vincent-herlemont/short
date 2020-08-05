extern crate anyhow;
#[macro_use]
extern crate log;

use std::env;

use anyhow::Result;
use clap::{App, AppSettings, Arg, ArgGroup, SubCommand};

use short::cfg::global_cfg_directory;
use short::cli::cfg::reach_directories;
use short::cli::commands;
use short::cli::commands::DEFAULT_SHOW_FORMAT;
use short::cli::terminal::emoji;
use short::run_file::kind::Kind;

use short::utils::check_update::{check_update, CrateInfo};
use short::BIN_NAME;
use strum::IntoEnumIterator;

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const TTL_CHECK_VERSION_SECONDS: &'static i64 = &43200;

fn main() -> Result<()> {
    env_logger::init();
    info!("BIN_NAME {}", BIN_NAME);
    info!("VERSION v{}", VERSION);

    Ok(run()?)
}

fn run() -> Result<()> {
    let files_kind = Kind::iter()
        .map(|e| e.as_ref().to_string())
        .collect::<Vec<_>>();
    let files_kind = files_kind.iter().map(|e| e.as_str()).collect::<Vec<_>>();
    let files_kind = files_kind.as_slice();

    let setup_arg = Arg::with_name("setup")
        .long("setup")
        .short("s")
        .takes_value(true)
        .help("Set up name");
    let environment_arg = Arg::with_name("environment")
        .long("env")
        .short("e")
        .takes_value(true)
        .help("Environment name");
    let environments_arg = Arg::with_name("environments")
        .help("Environments, you can provides several for compare to each other.")
        .long("envs")
        .short("e")
        .min_values(0)
        .multiple(true)
        .takes_value(true);

    let env_vars = vec![
        Arg::with_name("empty")
            .long("empty")
            .help("Set new vars with an empty value silently"),
        Arg::with_name("copy")
            .long("copy")
            .help("Set new vars and copy the current value silently"),
        Arg::with_name("delete")
            .long("delete")
            .help("Delete vars silently"),
        Arg::with_name("no_delete")
            .long("no_delete")
            .help("Take care to not delete vars, command fail otherwise."),
        Arg::with_name("file")
            .long("file")
            .short("f")
            .takes_value(true)
            .help("Source env file, take as model for envs synchronisation."),
    ];
    let env_group_vars = vec![
        ArgGroup::with_name("update_action")
            .args(&["empty", "copy"])
            .multiple(false),
        ArgGroup::with_name("delete_action")
            .args(&["delete", "no_delete"])
            .multiple(false),
    ];

    let app = App::new(format!("{}sht", emoji::SHORT))
        .version(VERSION)
        .author("Vincent Herlemont <vincentherl@leszeros.com>")
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::DeriveDisplayOrder)
        .setting(AppSettings::VersionlessSubcommands)
        .subcommand(
            SubCommand::with_name("init")
                .about("Init project, create an empty \"short.yaml\" configuration file."),
        )
        .subcommand(
            SubCommand::with_name("generate")
                .about("Generate empty setup or from project template repository.")
                .setting(AppSettings::ArgRequiredElseHelp)
                .arg(
                    Arg::with_name("setup_name")
                        .index(1)
                        .help("Setup name [or <template> name with \"-t\" option]."),
                )
                .arg(Arg::with_name("env_name").requires("kind").index(2).help("Env Name"))
                .arg(Arg::with_name("kind").index(3).help("Kind of file").possible_values(files_kind))
                .arg(
                    Arg::with_name("file")
                        .long("file")
                        .short("f")
                        .takes_value(true)
                        .help(r#"Path script, create directory if they miss. _[conflict with "-d"]_."#),
                )
                .arg(
                    Arg::with_name("public_env_directory")
                        .long("env-directory")
                        .short("e")
                        .takes_value(true)
                        .help(r#"Public env directory _[conflict with "-d"]_."#),
                )
                .arg(
                    Arg::with_name("private")
                        .long("private")
                        .short("p")
                        .help(r#"Save to private directory _[conflict with "-d"]_."#),
                )
                .arg(
                    Arg::with_name("list")
                        .long("list")
                        .short("l")
                        .help("Display template list."),
                )
                .arg(
                    Arg::with_name("template")
                        .long("template")
                        .short("t")
                        .takes_value(true)
                        .min_values(0)
                        .help("Template name."),
                )
                .arg(
                    Arg::with_name("target_directory")
                        .long("directory")
                        .short("d")
                        .takes_value(true)
                        .min_values(0)
                        .help("Template env directory."),
                )
                .group(
                    ArgGroup::with_name("action_type")
                        .args(&["setup_name", "list"])
                        .required(true),
                )
                .group(
                    ArgGroup::with_name("generate_type")
                        .args(&["env_name", "template", "list"])
                        .required(true),
                )
                .group(
                    ArgGroup::with_name("exclude_for_target_directory")
                        .args(&["file","private","public_env_directory"])
                        .multiple(true)
                        .required(false)
                        .conflicts_with("target_directory"),
                ).group(
                    ArgGroup::with_name("exclude_for_template")
                        .args(&["file","private","public_env_directory"])
                        .multiple(true)
                        .required(false)
                        .conflicts_with("template")
            ),

        )
        .subcommand(
            SubCommand::with_name("run")
                .about("Run setup [ARGS...].")
                .arg(
                    Arg::with_name("args")
                        .help("All arguments will be pass to the runnable script as argument.")
                        .index(1)
                        .multiple(true)
                        .takes_value(true),
                )
                .arg(setup_arg.clone())
                .arg(environment_arg.clone()),
        )
        .subcommand(
            SubCommand::with_name("rename")
                .about("Rename setup.")
                .arg(
                    Arg::with_name("last_setup_name")
                        .index(1)
                        .required(true)
                        .help("Last setup name."),
                )
                .arg(
                    Arg::with_name("new_setup_name")
                        .index(2)
                        .required(true)
                        .help("New setup name."),
                ),
        )
        .subcommand(
            SubCommand::with_name("new")
                .about("Create env file \".<env>\", in public directory by default.")
                .setting(AppSettings::ArgRequiredElseHelp)
                .arg(
                    Arg::with_name("name")
                        .help("Environment name.")
                        .index(1)
                        .required(true),
                )
                .arg(setup_arg.clone())
                .arg(
                    Arg::with_name("private")
                        .long("private")
                        .short("p")
                        .help("Save to private directory."),
                )
                .args(&env_vars)
                .groups(&env_group_vars),
        )
        .subcommand(
            SubCommand::with_name("sync")
                .about("Sync env files.")
                .arg(setup_arg.clone())
                .args(&env_vars)
                .groups(&env_group_vars),
        )
        .subcommand(
            SubCommand::with_name("edit")
                .about("Edit env file.")
                .arg(
                    Arg::with_name("environment")
                        .help("Environment name.")
                        .index(1),
                )
                .arg(setup_arg.clone())
                .arg(
                    Arg::with_name("editor")
                        .long("editor")
                        .takes_value(true)
                        .help("Editor binary path."),
                )
                .args(&env_vars)
                .groups(&env_group_vars),
        )
        .subcommand(
            SubCommand::with_name("dir")
                .about("Public env directory, [.] by default.")
                .setting(AppSettings::ArgRequiredElseHelp)
                .arg(
                    Arg::with_name("env_dir")
                        .help("Env directory path, must be inside of your project directory.")
                        .index(1),
                )
                .arg(
                    Arg::with_name("unset")
                        .long("unset")
                        .help("Unset directory path, use [.] as default value."),
                )
                .group(
                    ArgGroup::with_name("action")
                        .args(&["env_dir", "unset"])
                        .required(true),
                )
                .arg(setup_arg.clone()),
        )
        .subcommand(
            SubCommand::with_name("pdir")
                .about("Private env directory, unset by default.")
                .setting(AppSettings::ArgRequiredElseHelp)
                .arg(
                    Arg::with_name("env_dir")
                        .help("Private env directory path, have to be outside of your project directory.")
                        .index(1),
                )
                .arg(
                    Arg::with_name("unset")
                        .long("unset")
                        .help("Unset private directory path."),
                )
                .group(
                    ArgGroup::with_name("action")
                        .args(&["env_dir", "unset"])
                        .required(true),
                )
                .arg(setup_arg.clone()),
        )
        .subcommand(
            SubCommand::with_name("use")
                .about("Switch of current setup or/and environment.")
                .setting(AppSettings::ArgRequiredElseHelp)
                .arg(
                    Arg::with_name("setup_or_environment")
                        .help("The setup name or environment name if another one is already specified.")
                        .index(1),
                )
                .arg(
                    Arg::with_name("environment")
                        .help("The environment name.")
                        .index(2),
                )
                .arg(
                    Arg::with_name("unset")
                        .help("Unset current setup and environment.")
                        .long("unset")
                        .short("u")
                        .takes_value(false),
                ),
        )
        .subcommand(
            SubCommand::with_name("show")
                .about("Show your current setup.")
                .arg(
                    Arg::with_name("format")
                        .long("format")
                        .short("f")
                        .takes_value(true)
                        .min_values(0)
                        .help(format!("Display format \"{}\" by default.", DEFAULT_SHOW_FORMAT).as_str())
                )
                .arg(
                    Arg::with_name("display_setup")
                        .long("setup")
                        .short("s")
                        .takes_value(false)
                        .help("Display setup name."),
                )
                .arg(
                    Arg::with_name("display_env")
                        .long("env")
                        .short("e")
                        .takes_value(false)
                        .help("Display setup env."),
                )
                .group(
                    ArgGroup::with_name("display")
                        .args(&["display_setup", "display_env"])
                        .required(false)
                        .multiple(false),
                )
        )
        .subcommand(
            SubCommand::with_name("ls")
                .about("Display setups and environments available.")
                .arg(setup_arg.clone())
                .arg(environment_arg.clone()),
        )
        .subcommand(SubCommand::with_name("vars")
            .about("Display/Diff mapping environment variables.")
            .arg(setup_arg.clone())
            .arg(environments_arg.clone())
        )
        .subcommand(SubCommand::with_name("envs")
            .about("Display/Diff environment variables.")
            .arg(setup_arg.clone())
            .arg(environments_arg.clone())
        ).get_matches();

    if let None = app.subcommand_matches("show") {
        if let Ok((_, global_dir)) = reach_directories() {
            let global_cfg_directory = global_cfg_directory(&global_dir);
            let current_crate = CrateInfo::current();
            if let Some(message) = check_update(
                &global_cfg_directory,
                &current_crate,
                TTL_CHECK_VERSION_SECONDS,
            ) {
                println!("{}", message);
            }
        }
    }

    if let Some(_) = app.subcommand_matches("init") {
        commands::init(&app)?;
    } else if let Some(args) = app.subcommand_matches("generate") {
        commands::generate(&args)?;
    } else if let Some(args) = app.subcommand_matches("run") {
        commands::run(&args)?;
    } else if let Some(args) = app.subcommand_matches("ls") {
        commands::ls(&args)?;
    } else if let Some(args) = app.subcommand_matches("rename") {
        commands::rename(&args)?;
    } else if let Some(args) = app.subcommand_matches("use") {
        commands::r#use(&args)?;
    } else if let Some(args) = app.subcommand_matches("show") {
        commands::show(args)?;
    } else if let Some(args) = app.subcommand_matches("dir") {
        commands::env_dir(args)?;
    } else if let Some(args) = app.subcommand_matches("pdir") {
        commands::env_pdir(args)?;
    } else if let Some(args) = app.subcommand_matches("new") {
        commands::env_new(args)?;
    } else if let Some(args) = app.subcommand_matches("edit") {
        commands::env_edit(args)?;
    } else if let Some(args) = app.subcommand_matches("sync") {
        commands::env_sync(args)?;
    } else if let Some(args) = app.subcommand_matches("vars") {
        commands::vars(args)?;
    } else if let Some(args) = app.subcommand_matches("envs") {
        commands::envs(args)?;
    }

    Ok(())
}
