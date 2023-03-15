extern crate clap;
use clap::{Arg, ArgMatches, Command};

pub fn create_cli() -> ArgMatches {
    Command::new("Fundoc")
        .version(env!("CARGO_PKG_VERSION"))
        .about("\nFundoc extracts documentation from source files and merge it into readable .md files with references to the sources") .arg(Arg::new("init")
             .short('i')
             .long("init")
             .help("Creates the config file"))
        .get_matches()
}
