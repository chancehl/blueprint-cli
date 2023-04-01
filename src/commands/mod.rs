use std::path::PathBuf;

use clap::{arg, Command};

pub mod create;
pub mod save;

pub fn make_commands() -> Command {
    let create_command = Command::new("create")
        .about("Creates a file from a blueprint")
        .arg(arg!(<BLUEPRINT> "Which blueprint file to use"))
        .arg(
            arg!([DESTINATION] "The desination to save the file")
                .value_parser(clap::value_parser!(PathBuf)),
        )
        .arg_required_else_help(true);

    let save_command = Command::new("save")
        .about("Saves a blueprint .json file to the .blueprint folder on disk")
        .arg(
            arg!(<BLUEPRINT> "The blueprint .json file").value_parser(clap::value_parser!(PathBuf)),
        )
        .arg_required_else_help(true);

    let root_command = Command::new("git")
        .about("A simple cli for creating files from templates")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(create_command)
        .subcommand(save_command);

    root_command
}
