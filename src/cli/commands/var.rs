use anyhow::Result;
use clap::ArgMatches;
use term_table::row::Row;
use term_table::table_cell::TableCell;

use crate::cli::cfg::get_cfg;
use crate::cli::commands::sync::{sync_workflow, SyncSettings};
use crate::cli::settings::get_settings;
use crate::env_file::Env;
use crate::run_file::{EnvValue, generate_env_vars};

pub fn var(app: &ArgMatches) -> Result<()> {
    let mut cfg = get_cfg()?;
    cfg.sync_local_to_global()?;
    let cfg = cfg;

    let settings = get_settings(app, &cfg);

    let setup_name = settings.setup()?;
    let setup = cfg.current_setup(setup_name)?;

    let envs: Vec<_> = setup.envs().into_iter().filter_map(|r| r.ok()).collect();
    let recent_env = Env::recent(&envs)?;
    let sync_settings = SyncSettings::new(app);
    let mut envs = sync_workflow(recent_env, envs, sync_settings)?;
    envs.sort();
    let envs = envs;

    if envs.is_empty() {
        println!("there is no env");
        return Ok(());
    }

    let env_ref = envs.get(0).map(|env| env.clone()).unwrap();

    // Retrive vars / array_vars
    let local_setup = setup.local_setup().unwrap();
    let local_setup = local_setup.borrow();
    let array_vars = local_setup.array_vars().unwrap_or_default();
    let vars = local_setup.vars().unwrap_or_default();
    drop(local_setup);
    let env_vars = generate_env_vars(&env_ref, array_vars.borrow(), vars.borrow())?;

    let mut render_table = term_table::Table::new();
    render_table.separate_rows = false;
    render_table.style = term_table::TableStyle::thin();

    let mut title: Vec<_> = envs
        .iter()
        .map(|env| TableCell::new(env.name().unwrap().clone()))
        .collect();
    title.splice(0..0, vec![TableCell::new_with_col_span("", 2)].into_iter());

    render_table.add_row(Row::new(title));

    let nb_envs = envs.len() + 1;
    for env_var in env_vars {
        let env_value = env_var.env_value();
        match env_value {
            EnvValue::Var(_value) => {
                let mut line = vec![
                    TableCell::new(env_var.var().to_var()),
                    TableCell::new(env_var.var().to_env_var()),
                ];
                for env in &envs {
                    line.push(TableCell::new(
                        env.get(env_var.var().to_string()).unwrap().value().clone(),
                    ));
                }
                render_table.add_row(Row::new(line));
            }
            EnvValue::ArrayVar((array_var, array_var_values)) => {
                let line = vec![
                    TableCell::new(env_var.var().to_var()),
                    TableCell::new_with_col_span(
                        format!("{} ({})", env_var.var().to_env_var(), array_var.pattern()),
                        nb_envs,
                    ),
                ];
                render_table.add_row(Row::new(line));

                for var in array_var_values {
                    let mut line = vec![TableCell::new("".to_string())];
                    line.push(TableCell::new(var.name().clone()));
                    for env in &envs {
                        let env_var = env.get(var.name()).unwrap();
                        line.push(TableCell::new(env_var.value().clone()));
                    }
                    render_table.add_row(Row::new(line));
                }
            }
        }
    }

    println!("{}", render_table.render());

    Ok(())
}
