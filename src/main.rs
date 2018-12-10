#[macro_use]
extern crate clap;

mod cmd;

use crate::cmd::*;
use colored::*;

fn main() {
    let matches = build_app().get_matches();

    match matches.subcommand() {
        ("new", Some(matches)) => {
            cmd_new(&matches).unwrap_or_else(|e| println!("{}", e.red().bold()))
        }
        _ => build_app().print_help().expect("faild print help"),
    }
}
