use anyhow::{Context, Result};
use clap::ArgMatches;
use colored::*;
use prettytable::format;
use prettytable::Attr;
use prettytable::{Cell, Row, Table};

use crate::cli::cfg::get_cfg;

use crate::cli::selected_envs::selected_envs;
use crate::cli::settings::get_settings;

use crate::env_file::Env;
use crate::run_file::{generate_env_vars, EnvValue, EnvVar, ENV_ENVIRONMENT_VAR, ENV_SETUP_VAR};
use crate::utils::colorize::is_cli_colorized;
use prettytable::color::BLUE;

pub fn vars(app: &ArgMatches) -> Result<()> {
    let mut cfg = get_cfg()?;
    cfg.sync_local_to_global()?;
    let cfg = cfg;

    let settings = get_settings(app, &cfg);

    let setup_name = settings.setup()?;
    let setup = cfg.current_setup(setup_name)?;

    let envs = selected_envs(app, &setup, &settings)?;

    let is_current_env = |env: &Env| {
        if let Ok(current_env) = settings.env() {
            if let Ok(env_name) = env.name() {
                if *current_env == env_name {
                    return true;
                }
            }
        }
        false
    };

    let env_ref = envs.get(0).map(|env| env.clone()).unwrap();

    // Retrieve vars / array_vars
    let local_setup = setup.local_setup().unwrap();
    let local_setup = local_setup.borrow();
    let array_vars = local_setup.array_vars().unwrap_or_default();
    let vars = local_setup.vars().unwrap_or_default();
    drop(local_setup);
    let mut env_vars = generate_env_vars(&env_ref, array_vars.borrow(), vars.borrow())?;
    env_vars.push(
        EnvVar::from_setup(&setup)
            .context(format!("fail to generate var from setup `{:?}`", setup))?,
    );
    env_vars.push(EnvVar::from_env(&Env::new(".default".into())).unwrap());

    let mut render_table = Table::new();
    render_table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);

    let mut title: Vec<Cell> = vec![];
    let mut current_env_column_index = 0;
    for (i, env) in envs.iter().enumerate() {
        let mut cell = Cell::new(env.name()?.as_str()).with_style(Attr::Bold);
        if is_current_env(env) {
            current_env_column_index = i;
            cell = cell.with_style(Attr::ForegroundColor(BLUE))
        }

        title.push(cell);
    }

    title.splice(0..0, vec![Cell::new("").with_hspan(2)].into_iter());

    render_table.add_row(Row::new(title));

    let nb_envs = envs.len() + 1;
    for env_var in env_vars {
        let env_value = env_var.env_value();
        match env_value {
            EnvValue::Var(_value) => {
                let mut line = vec![
                    Cell::new(env_var.var().to_var().as_str()),
                    Cell::new(env_var.var().to_env_var().as_str()).with_style(Attr::Bold),
                ];
                for (i, env) in envs.iter().enumerate() {
                    let default_env_var_env = EnvVar::from_env(&env)?;
                    let default_env_var_setup = EnvVar::from_setup(&setup)?;
                    let var_name = env_var.var().to_string();

                    let env_var = if let Ok(var) = env.get(&var_name) {
                        Some(var)
                    } else if ENV_ENVIRONMENT_VAR == &var_name {
                        if let EnvValue::Var(var) = &default_env_var_env.env_value() {
                            Some(var)
                        } else {
                            None
                        }
                    } else if ENV_SETUP_VAR == &var_name {
                        if let EnvValue::Var(var) = &default_env_var_setup.env_value() {
                            Some(var)
                        } else {
                            None
                        }
                    } else {
                        None
                    };

                    if let Some(env_var) = env_var {
                        let mut cell = Cell::new(env_var.value());
                        if &i == &current_env_column_index {
                            cell = cell.with_style(Attr::ForegroundColor(BLUE));
                        }
                        line.push(cell);
                    }
                }
                render_table.add_row(Row::new(line));
            }
            EnvValue::ArrayVar((array_var, array_var_values)) => {
                let line = vec![
                    Cell::new(env_var.var().to_var().as_str()),
                    Cell::new(
                        format!(
                            "{} ({})",
                            env_var.var().to_env_var().bold(),
                            array_var.pattern()
                        )
                        .as_str(),
                    )
                    .with_hspan(nb_envs),
                ];
                render_table.add_row(Row::new(line));

                for var in array_var_values {
                    let mut line = vec![Cell::new("".to_string().as_str())];
                    line.push(Cell::new(var.name().clone().as_str()));
                    for (i, env) in envs.iter().enumerate() {
                        if let Ok(env_var) = env.get(var.name()) {
                            let mut cell = Cell::new(env_var.value().clone().as_str());
                            if &i == &current_env_column_index {
                                cell = cell.with_style(Attr::ForegroundColor(BLUE));
                            }
                            line.push(cell);
                        }
                    }
                    render_table.add_row(Row::new(line));
                }
            }
        }
    }

    if is_cli_colorized() {
        render_table.print_tty(true);
    } else {
        render_table.printstd();
    }

    Ok(())
}
