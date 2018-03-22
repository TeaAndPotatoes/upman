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

    match matches.subcommand() {
        ("list", _) => list_tools(&config_file),
        ("run", _) => run_updates(&config_file),
        ("add", Some(add_matches)) => config_file
            .write_all(add_matches.value_of("input").unwrap().as_bytes())
            .expect("Could not write to config file"),
        ("remove", Some(remove_matches)) => {
            // Unwrap command, as it is required 
            match remove_matches.value_of("command").unwrap() {
                "*" => config_file.set_len(0).expect("Unable to clear file"),
                _ => println!("Other option")
            };
        },
        ("", None) => println!("No subcommand was used"), // If no subcommand was used it'll match the tuple ("", None)
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable!()
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
        .subcommand(SubCommand::with_name("remove")
            .about("Remove a specific command from the configuration file")
            .arg(Arg::with_name("command")
                .required(true)
                .takes_value(true)
                .help("the command to remove from the config file")
            )
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
