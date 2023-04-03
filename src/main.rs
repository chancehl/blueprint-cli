mod commands;
mod models;
mod utils;

use commands::{
    create::handler as create_handler, init::handler as init_handler, make::handler as make_hander,
    make_commands, save::handler as save_handler,
};

fn main() {
    let matches = make_commands().get_matches();

    match matches.subcommand() {
        Some(("create", sub_matches)) => {
            create_handler(sub_matches).expect("Error while running create command")
        }
        Some(("save", sub_matches)) => {
            save_handler(sub_matches).expect("Error while running save command")
        }
        Some(("init", sub_matches)) => {
            init_handler(sub_matches).expect("Error while running init command")
        }
        Some(("make", sub_matches)) => {
            make_hander(sub_matches).expect("Error while running make command")
        }
        _ => unreachable!(),
    }
}
