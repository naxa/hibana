#[macro_use]
extern crate clap;

mod cmd;

use crate::cmd::*;
use colored::*;
use failure::Error;

fn main() {
    let matches = build_app().get_matches();

    let result = match matches.subcommand() {
        ("new", Some(matches)) => cmd_new(&matches),
        ("build", Some(_)) => cmd_build(),
        ("serve", Some(_)) => cmd_serve(),
        _ => {
            build_app().print_help().expect("failed to print help");
            return ();
        }
    };

    if let Err(e) = result {
        print_error(e);
    }
}

fn print_error(error: Error) {
    println!("{}", error.to_string().red().bold())
}
