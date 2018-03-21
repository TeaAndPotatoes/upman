extern crate clap;

use clap::{App, Arg, SubCommand};
use std::io::{BufRead, BufReader};
use std::io::prelude::*;
use std::fs::{File, OpenOptions};

fn main() {
    let app = create_app();
    let matches = app.get_matches();

    let config_filename = "foo.txt";

    let mut config_file = OpenOptions::new()
        .read(true)
        .write(true)
        .append(true)
        .create(true)
        .open(config_filename)
        .expect("Unable to open or create file at current directory");

    match matches.subcommand_name() {
        Some("list") => list_tools(&config_file),
        Some("add") => {
            // The value of input is required and add has been confirmed, so `input` can be parsed
            config_file
                .write_all(
                    format!("{}\n\n",
                        matches.subcommand_matches("add").unwrap()
                            .value_of("input").unwrap()
                    ).as_bytes(),
                )
                .expect("Unable to write to file");
        }
        None => println!("No subcommand was run"),
        _ => println!("An unchecked subcommand was run"),
    }
}

fn create_app() -> App<'static, 'static> {
    return App::new("Update Manager")
        .version("0.1.0-Beta")
        .author("Brendan Doney <bre.doney@gmail.com>")
        .about("Cross-platform, command-line update manager for easily checking and scheduling updates for different tools")
        .subcommand(SubCommand::with_name("list")
            .about("List all of the update commands added to this tool")
        )
        .subcommand(SubCommand::with_name("add")
            .about("Used for adding a command to check for updates")
            .arg(Arg::with_name("input")
                .required(true)
                .takes_value(true)
                .help("the command to add to the update checker")
            )
        );
}

fn list_tools(file: &File) {
    println!("Commands are not currently scheduled to check at any regular period\n");
    println!("Registered commands:");
    let buf_reader = BufReader::new(file);
    for line in buf_reader.lines() {
        let l = line.expect("Could not read line from file");
        if !l.is_empty() {
            println!("\t{}", l);
        }
    }
}
