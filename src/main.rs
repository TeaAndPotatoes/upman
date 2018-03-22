extern crate clap;

use clap::{App, Arg, SubCommand};
use std::io::{BufRead, BufReader};
use std::io::prelude::*;
use std::fs::{File, OpenOptions};
use std::process::Command;

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
        Some("run") => run_updates(&config_file),
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
        .subcommand(SubCommand::with_name("run")
            .about("Run each of the commands stored through add")
        )
        .subcommand(SubCommand::with_name("add")
            .about("Add a command to the list of tools to update")
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

fn run_updates(file: &File) {
    let buf_reader = BufReader::new(file);
    for line in buf_reader.lines() {
        let l = line.expect("Could not read line from file");
        if !l.is_empty() {
            let mut collection: Vec<&str> = l.split(" ").collect();
            let mut command = Command::new(collection.remove(0)); // First part is always assured when splitting
            for part in collection {
                command.arg(part); // Adding additional args from Vec, if any
            }
            let output = command.output().expect("Failed to execute process");
            println!("status: {}", output.status);
            println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
            println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        }
    }
}
