# sht / short / 🩳 
[![Crate](https://img.shields.io/crates/v/short.svg)](https://crates.io/crates/short) 
[![linux](https://github.com/vincent-herlemont/short/workflows/linux/badge.svg)](https://github.com/vincent-herlemont/short/actions?query=workflow%3Alinux)
[![osx](https://github.com/vincent-herlemont/short/workflows/osx/badge.svg)](https://github.com/vincent-herlemont/short/actions?query=workflow%3Aosx)
[![dicord](https://img.shields.io/static/v1?label=join&message=Discord&color=7289da&&logo=Discord)](https://discord.gg/AnVYgJM)
[![dicord](https://img.shields.io/static/v1?label=status&&message=WIP&color=orange&logo=data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADAAAAAwCAYAAABXAvmHAAAABmJLR0QA/wD/AP+gvaeTAAADuUlEQVRoge1ZS2gUWRQ99/VHNCOCikZGB5yFO4kfoiAoyuBoOm13dbDjb+PCz2ZWImKCuhGTncJsRBFmXESxI3aVnY6jcREFRdEYVPwQFdz5GRT8REnSXddFYlPVdHVev6omiHWgoOq+9+49p+t97q0GfPjw4WMiQU4NLW13+Pt9e2u9Y7+JhphoAm4RrHRANK39wcDfBNSAeG+XZpyvBjFZSE0h9B8s3A7EJmHol8Kwd6/ezprTt/vkSLUIjoeKp5Bplzyjdvabes/YKEBqCrV3/legHd0WPwvGZkvzagA3PeYljcoXsSmuWR+JaY1nbBTg+AaEyTFT0AkAc6x2Bq4VLZwVDd0Nky5FLg05+eLuhrmg0DGA142ZehDg/fRn1zN16mM8nRqOHFiWAQJ1BHRa7dlE+imANxbTFDEcXubkZ5R88D7AGwFMHbuakKdbnIn+6pJ/+SnU3rrk/7bW+mabkcBEuG5nSasdnVDoGIDpJVqmI0hHZYk6QekgY5Ns60CUXQeFaVMK5dqkoCYAsAlg4pVRPZ5w7l49KAnIJtKPALy1mILEdM5BxFVnT9SjEt8KtVyIwMy0z2piIFRShJlvAfCuhJf3ILQoxbdAOZnLNqVPA2i32kqJoMbuAeS4DkAKwMfRi86DaDmtv/hcNX7Bv6vRDGrUtQ4CthQ5HWHiTV2akXblXwKu8/xkKjn5a2ikF4DtLGBiFiZ6mcggpr6hmsH+K+uuDLqNVwxPCpVIKlkrQiO3AfxWplsewBMG7gqm50LQY0O74PoNeVZpRfT4QsF0A6MnrRQYSGU1fTNIfav1rCLr1oyHBGzC6C8tBQKaG3XtLzdxPa91I2mtTaCi7fErMy3NNqWfqMTzvCYWxE8rHDKZiDuSqWRYKZ7KoLJgqlMYtfhLePiQSrhqfJWQE8C4Z30kppaIEVtVabAJE8CCW0H82spFmOLfmBGT3sUAjwXE9fg8ADMluppThsM3GNgOe7Y638wHKqoRPBWQBxZJdn3R2dz5OasZl5nppK2FeEdUj2+UjempAGaSFfCgQCCU2wNgwO6IjkdSyVoZR54KIMn5T0wFAZkNmS8MbCXA+nFsZiA8/A94/HPKawGyb+C+9SGb0PtMYntqzrS+Udd2jufIMwExIzaVgd+lOgdzD4pNg9M+HCbgttVGwNHIhcSCcq48E8D5QB3kUpNPmQ2Zl8XG3jW9uRywDcBni7mGiDuWntgVcnLmOhdau3VHVYv27+g5c6ok1x/+/wFfgA8fPnz83PgG8Ekdaq6qdi0AAAAASUVORK5CYII=
)](https://discord.gg/AnVYgJM)
> A concise cli launcher / project manager using env files. 

The main goal it's readability and time saving with commands use in your project.
Short it's command-line tool that allow to run program (usually sh script) with environment variables mapping from .env files.
- It take care to **synchronize** and to **check** the format of all [**environment files**](#environment-file-environment_name) to each others.
- Allow multiple [**setups**](#setup) in the same [**project**](#project-shortyaml).
- Allow to store example of **no critical** environment file in your source code with an [**public env directory**](#public-directory).
- Allow to store prod/stage/etc.. and **critical** environment file in your source code with an [**private env directory 🔒**](#private-directory-).
- You can apply a mapping in order to **select**, **group** and add **custom formats / cases** on the fly on the environment variables.
- The result of mapping will be **inject** as environment variables in the output .sh script that will be executed.

![short global workflow](./docs/img/short_global_workflow.png)

---

It's include an index/registry that allow to share project templates: **[🌱 template-index](https://github.com/vincent-herlemont/short-template-index/blob/master/readme.md)**.

# Install

```
cargo install short
```

### Configure prompt

<details>
  <summary>✨ PS1 (BASH/ZSH)</summary>
  
Example with PS1 configure by `.bashrc`

```shell script
export PS1="$(sht show -f):\w\$ "
```

Example with PS1 configure by `.zshrc`

```shell script
⚠️ TODO ...
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

# Quick start with template

<details>
  <summary>🌱 Example with <b>Node && ExpressJs</b></summary>
  
  Generate an simply aws sam project base on this template [node-express](https://github.com/vincent-herlemont/node-express-short-template).
  
  Requirement : You have installed [node](https://nodejs.org/) and [npm](https://www.npmjs.com/).
  
  ```
  $> sht init
  $> sht generate node-express -d -t
  $> sht run
  ```
  `-t`: generate from template.
  `-d`: create a sub directory _(optional)_.
</details>


<details>
  <summary>🌱 Example with <b>AWS SAM</b></summary>
  
  Generate an simply aws sam project base on this template [aws-node-sam](https://github.com/vincent-herlemont/aws-node-sam-short-template).
  
  Requirement : You have installed [SAM](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/serverless-sam-cli-install.html)
  and [AWS_CLI](https://docs.aws.amazon.com/cli/latest/userguide/cli-chap-install.html).
  
  ```
  $> sht init
  $> sht generate aws-node-sam -d -t
  $> sht run
  ```
  `-t`: generate from template.
  `-d`: create a sub directory _(optional)_.
</details>

You can list all templates available with `sht generate -l` and add a new one [**here**](https://github.com/vincent-herlemont/short-template-index/blob/master/readme.md#add-template-and-share-with-the-community).

# Quick start blank ✍️

Generate a simply bash script who display variables. You can use this base
for do as you want. 

```
$> sht init
$> sht generate setup_1 test -d
$> sht run
```
`-d`: create a sub directory (optional).

---
WIP README
- ⚠️ tutorials step by step.
- ⚠️ command documentations.
---

# Commands
### `init` project.
Create an empty `short.yaml` configuration file. This one define the your project directory.
All `short` command inside of this folder and his child's folders take for references this configuration file.
```
$> sht init
```
_short.yaml (generated)_
```yaml 
setups: {}
```
### `generate` setup. 
Generate an **empty** setup or a setup from the **project templates [repository](https://github.com/vincent-herlemont/short-template-index/blob/master/readme.md)**, 
this command can be also list all available project templates.

<details>
  <summary><b>✍ Generate an empty setup ️</b></summary>

| Arguments | Required  | Description |
| ---------- | -------- | ----------- |
| <setup_name> | yes | Setup name |
| <env_name> | yes  | Env name |

| Options | [Allow empty*]() | Default | Description |
| ---------- | -------- | ------- | ----------- |
| -d , --directory | yes | <setup_name> | Target directory. |
| -p , --private| no | false | 🔒 Save to private directory. _[conflict with "-d"]_ |
| -s , --shebang| no | #!/bin/bash | Interpreter program when `run.sh` generation. |
| -f , --file| no | run.sh | Path script, create directory if they miss. _[conflict with "-d"]_ |
| -e , --env-directory| no | . | Public env directory. _[conflict with "-d"]_ | 

Example : create a setup named `my_setup` with `.test` environment file.
```
$> sht generate my_setup test
```
_short.yaml (generated)_ : Configuration file.
```
setups:
  my_setup:
    file: run.sh
    array_vars:
      all: ".*"
    vars: []
```
_.test (generated)_ : Environment file.
```
VAR1=VALUE1
VAR2=VALUE2
```
_run.sh (generated)_ : Runnable file.
```
#!/bin/bash
declare -A all && eval all=($ALL)

declare -p all
```
The seconds line `declare -A all && eval all=($ALL)` allow to use **[bash associative array](https://www.gnu.org/software/bash/manual/html_node/Arrays.html)**.

</details>

<details>
  <summary><b>🌱 List all <a href="https://github.com/vincent-herlemont/short-template-index/blob/master/readme.md">project templates</a></b></summary>
  
```
$> sht generate -l
```
</details>

<details>
  <summary><b>🌱 Generate setup from projects template</b></summary>

| Arguments | Required  | Description |
| ---------- | -------- | ----------- |
| <setup_name/template> | yes | Setup name or \<template> name with "-t" option left empty |
  
| Options | [Allow empty*](#option-allow-empty) | Default | Description |
| ---------- | -------- | ------- | ----------- |
| -t , --template | yes | <setup_name> | Template name, can be founded in list template `-l` |
| -d , --directory | yes | <setup_name> | Target directory. |
  
Example : create a setup `node-express` with its associated envs.
```
$> sht generate node-express -t
```
👉 _short.yaml (generated)_ and _run.sh (generated)_ with generate from the following project template : [**node-express**](https://github.com/vincent-herlemont/node-express-short-template).

</details>

### `rename` setup

Rename setup. e.g `my_setup` -> `another_setup`.
```
$> sht rename my_setup another_setup
```

### `new` env

Create new env. e.g `dev`
```
$> sht new dev
```
Or private env. e.g `prod`
```
$> sht new dev -p
```
🔒 `-p` save the file in the private directory. 

### `sync` env

Sync all environment and ask you for each diff what to do.
```
$> sht sync
```

### `edit` env

Edit an environment file with your default text editor.You can choose different editor with `--editor <editor>` or `EDITOR` env vars.
```
$> sht edit
```

### `dir` env directory

Set or unset env directory.
```
$> sht dir ./envs/
$> sht dir --unset
```

### `pdir` env private directory

Set or unset env directory.
```
$> sht pdir ../private_envs/
$> sht pdir --unset
```

### `use` select/switch your setup / environment

e.g. Select `my_setup` with `dev` environment.
```
$> sht use my_setup dev
```
e.g. Switch from `dev` to `prod` environment.
```
$> sht use prod
```
👉 If an setup and environment if already selected,you can avoid to provide the setup and just indicate the environment that you want.

### `show` your current setup / environment

```
$> sht show
💁 your current setup is `my_setup`:`dev`
```

### `ls` list all setups and environments

List all setups / environments and indicated the current one like `sht show`.
```
$> sht ls
  my_project (run.sh)
     prod (.prod)
     dev (.dev)
  my_sub_project_1 (my_sub_project_1/run.sh)
     prod (sub_env/.prod)
     staging (sub_env/.staging)
     test (sub_env/.test)
  my_sub_project_2 (my_sub_project_2/run.sh)
>    prod (sub_env/.prod)
     staging (sub_env/.staging)
     test (sub_env/.test)
```

### `vars` display/compare mapping environment variables

e.g. Display variables mapping of `test` current environment
```
$> sht vars
                           | test
 all         | ALL (.*)
             | VAR1        | VALUE1
             | VAR2        | VALUE2
 var1        | VAR1        | VALUE1
 var2        | VAR2        | VALUE2
 short_setup | SHORT_SETUP | my_sub_project_2
 short_env   | SHORT_ENV   | test
```

e.g Compare variables mapping of `test` and `prod` environment
```
$> sht vars -e prod test
                           | prod             | test
 all         | ALL (.*)
             | VAR1        | VALUE1           | VALUE1
             | VAR2        | VALUE2_OF_PROD   | VALUE2
 var1        | VAR1        | VALUE1           | VALUE1
 var2        | VAR2        | VALUE2_OF_PROD   | VALUE2
 short_setup | SHORT_SETUP | my_sub_project_2 | my_sub_project_2
 short_env   | SHORT_ENV   | prod             | test
```

### `envs` display/compare environment variables

e.g. Display variables of `test` current environment
```
$> sht vars
      | test
 VAR1 | VALUE1
 VAR2 | VALUE2
```

e.g. Compare variables of `test` and `prod` environment
```
$> sht vars -e prod test
      | prod           | test
 VAR1 | VALUE1         | VALUE1
 VAR2 | VALUE2_OF_PROD | VALUE2
```

# Configuration file `short.yaml`

### `setups`.`<setup_name>`.`file`: Runnable file

File that will be run with command `sht run`.

| Type | Required | Description |
| ---------- |-------- | ----------- |
| Path | yes  | Relative path to the run file |

### `setups`.`<setup_name>`.`array_vars`: Array Vars

Group and apply case on environment variables.

 ⚠️ TODO: array_vars specification.

### `setups`.`<setup_name>`.`vars[]`: Vars

List of selected variables. If it's **empty** all variables will be injected.

### `setups`.`<setup_name>`.`public_env_dir`: Public env directory

| Type | Required | Default | Description |
| ---------- | -------- | -------- | ----------- |
| Path | no | . | Relative path to the [public env directory](#public-directory) |

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
    use         Switch of current setup or/and environment.
    show        Show your current setup.
    ls          Display setups and environments available.
    vars        Display/Diff mapping environment variables.
    envs        Display/Diff environment variables.
    help        Prints this message or the help of the given subcommand(s)
```
---
# Concepts

### Project [`short.yaml`](#configuration-file-shortyaml)

It's a directory with the configuration file `short.yaml` inside it : that defined the **project root**.  
All `short` command inside of this folder and his child's folders take for references this configuration file.

- For **create/init** an project see [`init`](#init-project) command.

### Setup

Setup it's is main concept of short. It's an configuration of [`short.yaml`](#configuration-file-shortyaml) 
and take a **name**, a **runnable file**, [**public env directory**](#public-directory) and **mapping options**.
This is how short gets an easily way to simplify run command.

👉 Each setup one and only one **one runnable file**.

- For **create/generate** an setup see [`generate`](#generate-setup)
- For **list** all setups see [`ls`](#ls-list-all-setups-and-environments) command.
- For **display** mapping of the setups see [`vars`](#vars-displaycompare-mapping-environment-variables) command.

### Environment file `.<environment_name>`

Each environment file define one environment in order to the environment name come from the file name like `.<my_env>` _environment file_ => `my_env` _environment name_.

👉 **The prefix `.` is mandatory**. 

### Directory

These directories store `.<env>` files.
Env files presents in it's directories will be synchronised to each other 
if these belong to the same [**setup**](#setup).

#### Public directory

This directory must be inside of your project (The default value it's the [root folder of the project](#project-shortyaml)).
That can be a sub folder like `./env/` see [**setup configuration**](#setupssetup_namepublic_env_dir-public-env-directory) for more details.
So if you had configured git or another code versioning solution, public directory allow to save with your code
no critical configuration files like example configuration file.

```
.
└── project
    ├── envs # public env directory
    │   └── .dev
    ├── ...
    └── short.yaml
```

#### Private directory 🔒

This directory must be outside of your project. The path of the private directory 
will be not store in **project configuration ✅** ! So if you had configured git or
another code versioning solution that will be never commit with your code.

```
.
├── envs # private env directory
│   └── .dev
└── project
    ├── run.sh
    └── short.yaml
```

### Environment file .<environment_name>

The file formatting must be follow the [**RFC 2 – .env file**](https://smartmob-rfc.readthedocs.io/en/latest/2-dotenv.html) guide line.
```
# Comment

VAR1=VALUE1
VAR2=VALUE2
```

Each environment inside on the same setup (_public environment directory/private environment directory_), are **synchronised** to each other.

- For **create** an new environment file see [`new`](#new-env) command.
- For **list** all environment files see [`ls`](#ls-list-all-setups-and-environments) command.
- For **display** the content of environment file see [`envs`](#envs-displaycompare-environment-variables) command.
- For **edit** en environment file see [`edit`](#edit-env) command.

### Option allow empty

Option like `-d` who can found in `sht generate my_template my_env -d` can have three state.
1) **Deactivate** you not specified the option :  e.g. `sht generate my_template my_env` 
2) **Activate**; take the value by default : e.g. `sht generate my_template my_env -d` 
The value of `-d` is `my_template`.
3) **Activate with value** : e.g.  `sht generate my_template my_env -d foo`.
The value of `-d` is `foo`.  