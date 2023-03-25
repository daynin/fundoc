extern crate clap;
use clap::{arg, ArgMatches, Command};

pub fn create_cli() -> ArgMatches {
    Command::new("Fundoc")
        .version(env!("CARGO_PKG_VERSION"))
        .about("\nFundoc extracts documentation from source files and merge it into readable .md files with references to the sources")
        .arg(arg!(-i --init "Creates the config file"))
        .arg(arg!(-e --extension "This flag is only for running Fundoc as an extension for mdBook. It requires by mdBook preprocessors API"))
        .arg(arg!([supports] ... "Check if fundoc has a plugin for passed file type from mdBook"))
        .get_matches()
}
