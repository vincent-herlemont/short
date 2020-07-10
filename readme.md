# sht / short / 🩳 
[![Crate](https://img.shields.io/crates/v/short.svg)](https://crates.io/crates/short) ![linux](https://github.com/vincent-herlemont/short/workflows/linux/badge.svg) ![osx](https://github.com/vincent-herlemont/short/workflows/osx/badge.svg)
> A concise cli launcher / project manager using env files.

The main goal it's readability and time saving with commands use in your project.
Short it's command-line tool that allow to run program (usually sh script) with environment variables mapping from .env files.
It take care about to synchronize and to check the format of all env files to each others.
You can apply a mapping in order to select, group and add custom format / cases on the fly on the variables.
The result of mapping will be inject as environment variables in the output .sh script that will be executed.

![short global workflow](./docs/img/short_global_workflow.png)

---

It's include an index/registry that allow to share project templates: **[🎁 template-index](https://github.com/vincent-herlemont/short-template-index/blob/master/readme.md)**.

# Install

```
cargo install short
```

### Configure prompt

<details>
  <summary>✨ PS1</summary>
  
Example with PS1 configure by `.bashrc`

```shell script
export PS1="$(sht show -f):\w\$ "
```

</details>

<details>
  <summary>✨ starship</summary>
  
Example with [custom pre-prompt : **starship**](https://starship.rs/advanced-config/#custom-pre-prompt-and-pre-execution-commands-in-bash).

Here the custom script that starship run before display prompt.

```shell script
#!/bin/bash

function blastoff(){
    sht show -f
}
starship_precmd_user_func=blastoff
```
    
</details>

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
    ls          Display setups and environments available.
    vars        Display mapping environment variables.
    envs        Display environment variables.
    help        Prints this message or the help of the given subcommand(s)
```

