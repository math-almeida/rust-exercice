extern crate core;

use clap::{command, value_parser, Arg};
use std::{path::PathBuf, process};

use rust_exercice::parser::process_csv;

fn main() {
    let args = command!()
        .arg(
            Arg::new("PATH")
                .required(true)
                .value_parser(value_parser!(PathBuf)),
        )
        .get_matches();
    let path = args.get_one::<PathBuf>("PATH").unwrap();

    if let Err(err) = process_csv(path) {
        eprintln!("Error while processing csv: {}", err);
        process::exit(1);
    }
}
