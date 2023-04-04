# Blueprint

## Overview

Blueprint (binary `bp` or `blueprint`) is a simple file scaffolding tool that lets you create files quickly based on user-defined templates.

## Installation

Coming soon...

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
