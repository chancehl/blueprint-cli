use std::path::PathBuf;

use clap::{Arg, ArgAction, Command};

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
        .about("Creates a file from a blueprint template")
        .arg(Arg::new("BLUEPRINT").required(true))
        .arg(
            Arg::new("DESTINATION")
                .short('d')
                .long("destination")
                .value_parser(clap::value_parser!(PathBuf)),
        )
        .arg_required_else_help(true);

    let init_command = Command::new("init")
        .about("Initializes the local repository for the user by creating a `$HOME/.blueprint` directory.")
        .arg(force_arg);

    let make_command = Command::new("make")
        .about("Creates a blueprint template from a given file")
        .arg(
            Arg::new("FILE")
                .required(true)
                .value_parser(clap::value_parser!(PathBuf)),
        )
        .arg(Arg::new("NAME").short('n').long("name").required(true))
        .arg_required_else_help(true);

    let save_command = Command::new("save")
        .about("Saves a blueprint template file to the local blueprint repository")
        .arg(
            Arg::new("FILE")
                .required(true)
                .value_parser(clap::value_parser!(PathBuf)),
        )
        .arg_required_else_help(true);

    

    Command::new("git")
        .about("A simple cli for creating files from templates")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(create_command)
        .subcommand(init_command)
        .subcommand(make_command)
        .subcommand(save_command)
}
