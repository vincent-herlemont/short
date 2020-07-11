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
It take care to synchronize and to check the format of all env files to each others.
Allow to store critical environment files in private paths that not shared in the project source code.
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

# Quick start with template

Generate an simply aws sam project base on this template [aws-node-sam](https://github.com/vincent-herlemont/aws-node-sam-short-template).

Requirement : You have installed [SAM](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/serverless-sam-cli-install.html)
and [AWS_CLI](https://docs.aws.amazon.com/cli/latest/userguide/cli-chap-install.html).

```
$> sht init
$> sht generate aws-node-sam -d -t
$> sht run
```

# Quick start blank

Generate a simply bash script who display variables. You can use this base
for do as you want.

```
$> sht init
$> sht generate setup_1 test
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

