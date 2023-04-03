use std::path::PathBuf;

use clap::{arg, Arg, ArgAction, Command};

pub mod create;
pub mod init;
pub mod make;
pub mod save;

pub fn make_commands() -> Command {
    // shared args
    let force_arg = Arg::new("FORCE")
        .short('f')
        .long("force")
        .action(ArgAction::SetTrue);

    let create_command = Command::new("create")
        .about("Creates a file from a blueprint")
        .arg(arg!(<BLUEPRINT> "Which blueprint file to use"))
        .arg(
            arg!([DESTINATION] "The desination to save the file")
                .value_parser(clap::value_parser!(PathBuf)),
        )
        .arg_required_else_help(true);

    let init_command = Command::new("init")
        .about("Initializes tool by creating the .blueprint directory for you")
        .arg(force_arg);

    let make_command = Command::new("make")
        .about("Creates a blueprint .json file from a given file")
        .arg(
            arg!(<TEMPLATE> "The file to use as the template")
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
        .subcommand(init_command)
        .subcommand(make_command)
        .subcommand(save_command);

    root_command
}
