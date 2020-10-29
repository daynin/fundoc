extern crate clap;
use clap::{App, Arg, ArgMatches};

pub fn create_cli() -> ArgMatches<'static> {
    App::new("Fundoc")
        .version(env!("CARGO_PKG_VERSION"))
        .about("\nFundoc extracts documentation from source files and merge it into readable .md files with references to the sources")
        .arg(Arg::with_name("init")
           .short("i")
           .long("init")
           .help("Creates the config file"))
        .get_matches()
}
