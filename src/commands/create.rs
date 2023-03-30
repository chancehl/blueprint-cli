use clap::ArgMatches;
use std::path::PathBuf;

pub fn handler(arg_matches: &ArgMatches) -> Result<(), &'static str> {
    let blueprint = arg_matches
        .get_one::<String>("BLUEPRINT")
        .expect("Blueprint required");

    let destination = arg_matches.get_one::<PathBuf>("DESTINATION");

    println!("blueprint={:?}, destination={:?}", blueprint, destination);

    // validate blueprint file

    Ok(())
}
