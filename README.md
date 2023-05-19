# Blueprint 📃

## Overview

Blueprint (binary `bp` or `blueprint`) is a simple file scaffolding tool that lets you create files quickly based on user-defined templates.

## Installation

### (Easiest) Are you a Rustacean? 🦀

Run `cargo install blueprint-cli`.

### Semi-automated installation 🚙

Open a terminal and run the following:

```
# Downloads the file
$ curl https://github.com/chancehl/blueprint-cli/releases/download/v1.0.0/blueprint-x86_64-apple-darwin.tar.gz --output SOME_FOLDER -L

# Unzips the file
$ tar -xf SOME_FOLDER\blueprint-x86_64-apple-darwin.tar.gz

# Moves the binary to the appropriate bin folder so you can reference it by name
$ mv SOME_FOLDER/blueprint-x86_64-apple-darwin/blueprint /usr/bin

# Gives the binary permissions to run
$ chmod +755 /usr/bin/blueprint

# Refresh terminal
$ source ~/.bashrc
```

† Replace the version with the one that you are trying to download (latest = 1.0.0).
†† Replace the file with the one appropriate for your operating system (see releases page for all options).

### Manual installation 🔨

1. Navigate to the [blueprint-cli Github repository](https://github.com/chancehl/blueprint-cli)
2. On the right-hand side look for a panel that says "releases"
3. Click the link to the release that says "latest"
4. Download the appropriate file for your operating system
5. Extract the binaries
6. (MacOS/Linux users) copy the file to your `/usr/bin` or `/usr/local/bin` directory
7. (MacOS/Linux users) run `chmod +755` on the binary you moved to the `bin` folders

## Usage

```
A simple cli for creating files from templates

Usage: bp <COMMAND>

Commands:
  create  Creates a file from a blueprint
  init    Initializes tool by creating the .blueprint directory for you
  make    Creates a blueprint .json file from a given file
  save    Saves a blueprint .json file to the .blueprint folder on disk
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

## Commands

### Init

```
Initializes the local repository for the user by creating a `$HOME/.blueprint` directory.

Usage: bp init [OPTIONS]

Options:
  -f, --force
  -h, --help   Print help
```

### Make

```
Creates a blueprint template from a given file

Usage: bp make [OPTIONS] <TEMPLATE>

Arguments:
  <TEMPLATE>  The file to use as the template

Options:
  -n, --name <NAME>
  -h, --help         Print help
```

### Create

```
Creates a file from a blueprint template

Usage: bp create <BLUEPRINT> [DESTINATION]

Arguments:
  <BLUEPRINT>    Which blueprint file to use
  [DESTINATION]  The destination to save the file

Options:
  -h, --help  Print help
```

### Save

```
Saves a blueprint template file to the local blueprint repository

Usage: bp save <BLUEPRINT>

Arguments:
  <BLUEPRINT>  The blueprint .json file

Options:
  -h, --help  Print help
```
