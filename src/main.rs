#[macro_use]
extern crate clap;

mod cmd;

use crate::cmd::*;
use colored::*;
use failure::Error;

fn main() {
    let matches = build_app().get_matches();

    match matches.subcommand() {
        ("new", Some(matches)) => cmd_new(&matches).unwrap_or_else(|e| print_error(e)),
        ("build", Some(_)) => cmd_build().unwrap_or_else(|e| print_error(e)),
        ("serve", Some(_)) => cmd_serve().unwrap_or_else(|e| print_error(e)),
        _ => build_app().print_help().expect("failed to print help"),
    }
}

fn print_error(error: Error) {
    println!("{}", error.to_string().red().bold())
}
