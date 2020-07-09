# sht / short / 🩳 
[![Crate](https://img.shields.io/crates/v/short.svg)](https://crates.io/crates/short) ![linux](https://github.com/vincent-herlemont/short/workflows/linux/badge.svg) ![osx](https://github.com/vincent-herlemont/short/workflows/osx/badge.svg)
> A concise cli project manager using env files.

Short it's command-line tool that allow to run programme (usually sh script) 
with mapping environment variables from .env file.

It's include an index/registry that allow to share project templates: **[🎁 template-index](https://github.com/vincent-herlemont/short-template-index/blob/master/readme.md)**.

# Install

```
cargo install short
```

### Configure prompt

- Example with PS1 configure by `.bashrc`
```
export PS1="$(sht show -f):\w\$ "
```
- Example with [custom pre-prompt : starship✨](https://starship.rs/advanced-config/#custom-pre-prompt-and-pre-execution-commands-in-bash).

Here the custom script that starship run before display prompt.
```
#!/bin/bash

function blastoff(){
    sht show -f
}
starship_precmd_user_func=blastoff
```

# Quick start

That commands allow to generate an simply aws sam project base on this template [aws-sam-short-template](https://github.com/vincent-herlemont/aws-sam-short-template).

```
$> sht init
$> sht generate aws-sam -d -t
$> sht run
```

---

⚠️ wip ...

- TODO : tutorials step by step.
- TODO : command documentations.

# Help 
```
USAGE:
    sht [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    init        Init project, create an empty "short.yaml" configuration file.
    generate    Generate empty setup or from template setup repository.
    run         Run setup [ARGS...].
    rename      Rename setup.
    new         Create env file ".<env>", in public directory by default.
    sync        Sync env files.
    edit        Edit env file.
    dir         Public env directory, [.] by default.
    pdir        Private env directory, unset by default.
    show        Show your current setup.
    use         Switch of current setup or/and environment.
    ls          Display setups and environments.
    vars        Display mapping environment variables.
    envs        Display environment variables.
    help        Prints this message or the help of the given subcommand(s)
```

