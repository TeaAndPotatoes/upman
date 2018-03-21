extern crate clap;

#[allow(unused_imports)]
use clap::{App, Arg, SubCommand};
use std::io::{BufRead, BufReader};
// use std::io::prelude::*;
use std::fs::{OpenOptions, File};

fn main() {
    let app = create_app();
    let matches = app.get_matches();

    let config_filename = "foo.txt";

    let config_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(config_filename)
        .expect("Unable to open or create file at current directory");

    match matches.subcommand_name() {
        Some("list") => {
            println!("List was used");
            list_tools(config_file);
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
        );
}

fn list_tools(file: File) {
    println!("Commands are not currently scheduled to check at any regular period\n");
    let buf_reader = BufReader::new(file);
    for line in buf_reader.lines() {
        let l = line.expect("Could not read line from file");
        println!("\t{}", l);
    }
}
