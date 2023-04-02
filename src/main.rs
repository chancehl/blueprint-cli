mod commands;
mod models;

use commands::{
    create::handler as create_handler, init::handler as init_handler, make_commands,
    save::handler as save_handler,
};

fn main() {
    let commands = make_commands();
    let matches = commands.get_matches();

    match matches.subcommand() {
        Some(("create", sub_matches)) => {
            create_handler(sub_matches).expect("Error while running create command")
        }
        Some(("save", sub_matches)) => {
            save_handler(sub_matches).expect("Error while running save command")
        }
        Some(("init", _sub_matches)) => init_handler().expect("Error while running init command"),
        _ => unreachable!(),
    }
}
