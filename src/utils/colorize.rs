pub fn is_cli_colorized() -> bool {
    if let Some(_) = option_env!("NO_COLOR") {
        return false;
    }
    if let Some(clicolor_force) = option_env!("CLICOLOR_FORCE") {
        if clicolor_force != "0" {
            return true;
        }
    }
    false
}
