# Short 

:construction: *Disclaimer : Work in progress. It's an experimental project that I used for learn rust. Most of the code are dirty and need to be rewrite.*

Short it's command-line tool that allow to run sh script with multiple .env file.

### Initializing an project :black_square_button:
You need to go to your project's directory and type :
```
$> short init
```
This create an empty `short.yml` configuration file.

### Create new set up.

The following command, create new set up with empty runnable shell script. By default, filename is `run.sh`, but
you can customise the name with `-n <custom_name_script>` and you can provide your own script with `-f <path_script>`.
```
$> short new
```

Here the content of `run.sh`
```sh
#!/bin/bash
declare -A all && eval all=($ALL) # Associative array with all variables.
declare -p all                    # Print [debug]
```

### Create environment :black_square_button:
Now you have to create an environment to your current set up, in the following example we create an environment named `dev`
but you can choose the name as you want.
```
$> short env new dev
```

### Run :black_square_button:

Short run the set up with the configured environment.
```
$> short run
```

### Show current set up and environment :black_square_button:
It's useful to see which set up is configured for deployment so that avoid deployment mistake.
In the example below, the set up `setup_1` with the `dev` environment is configured.
```
$> short show
setup_1 dev
```

### Switch of current set up or/and environment :black_square_button:
The main functionality of short is to switch easily between environment and set up.
In the example below, we switch twice, first on the set up `setup_1` with `prod`
environment and second on the set up `setup_2` with `dev` environment.
```
$> short use setup_1 prod
$> short use setup_2 dev
```

### List set up and environments :black_square_button:
Here we have to display all set up available on your project. The stared row `*` is 
the current set up / environment.
```
$> short ls
   setup_1 dev
   setup_1 prod
 * setup_2 dev
   setup_2 prod
```

### Change your env directory :black_square_button:
For some case and project organisation, you need to have an specific directory
for store your environment files. The example below configure the current set up with
the directory `./envs/` as env directory.
```
$> short env dir ./envs/
```

### Add a private env directory :black_square_button:
In some case like security, you have to store your environment files in an other directory outside of your
project directory. With this method you avoid to mix critical data present in environment file and your code.
This method add private env directory to your current set up.

The example below configure the current set up with the directory `../penvs/` as private env directory.
```
$> short env pdir ../penvs/
```

### Rename setup :black_square_button:

You can simply rename setup with the following command.
```
$> short rename <last setup name> <new setup name>
```

----
# Configuration files

Short command line manage by himself configuration files, but you can edit these entirely by yourself.

### Project configuration file (short.yml)

This configuration is ranked at the root of your project. It defined all setup configuration of your project.

```yaml
setups:
  - name: setup_1
    file: ./run.sh
    array_vars:
      all: ".*"
```

With custom public env directory and custom vars.
```yaml
setups:  
  - name: setup_1
    public_env_dir: './env/'
    file: ./run.sh
    array_vars:
      all: ".*"
    vars: [ SETUP_NAME ]
```

### Global configuration file

This configuration is placed in your home directory.
It's store configuration that not allowed in local configuration the path of private env directory.

```yaml
projects:
  - dir: "/home/perichon/project/my_project"
    setups:
      - name: 'setup_1'
        private_env_dir: "/home/perichon/private_envs/"
```


----
[See deprecated v0-0-1](https://github.com/vincent-herlemont/short/tree/v0-0-1)