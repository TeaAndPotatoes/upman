extern crate clap;

use std::io::{BufRead, BufReader};
use std::io::prelude::*;
use std::fs::{File, OpenOptions, create_dir_all};
use std::process::Command;

mod app;

fn main() {
    let app = app::create_app();
    let matches = app.get_matches();

    let config_directory = format!("{}/.upman/", std::env::home_dir().unwrap().display());
    create_dir_all(&config_directory).expect("Unable to create missing directories");
    let config_filepath = format!("{}upman.conf", config_directory);

    let mut config_file = OpenOptions::new()
        .read(true)
        .write(true)
        .append(true)
        .create(true)
        .open(config_filepath)
        .expect("Unable to open or create file at current directory");

    match matches.subcommand() {
        ("list", _) => list_tools(&config_file),
        ("run", _) => run_updates(&config_file),
        ("add", Some(add_matches)) => config_file
            .write_all(format!("{}\n", add_matches.value_of("input").unwrap()).as_bytes())
            .expect("Could not write to config file"),
        ("remove", Some(remove_matches)) => {
            // Unwrap command, as it is required 
            match remove_matches.value_of("command").unwrap() {
                "*" | "all" | "." => config_file.set_len(0).expect("Unable to clear file"),
                _ => println!("Other option")
            };
        },
        ("", None) => println!("No subcommand was used"), // If no subcommand was used it'll match the tuple ("", None)
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable!()
    }
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
            // Execute the command using output(), and check for errors - notify if any are found
            match command.output() {
                Ok(output) => println!("stdout: {}", String::from_utf8_lossy(&output.stdout)),
                Err(_) => println!("\n<WARNING> Could not execute command: {}\n", l)
            };
        }
    }
}

// fn remove_command(command: &str, file: &File) {
    
// }
