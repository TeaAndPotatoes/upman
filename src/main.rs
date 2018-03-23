extern crate clap;

use std::io::{BufRead, BufReader};
use std::io::prelude::*;
use std::fs::{create_dir_all, File, OpenOptions};
use std::process::Command;

mod app;

fn main() {
    let app = app::create_app();
    let matches = app.get_matches();

    let config_directory = format!("{}/.upman/", std::env::home_dir().unwrap().display());
    create_dir_all(&config_directory).expect("Unable to create missing directories");
    let config_filepath = format!("{}upman.conf", config_directory);

    match matches.subcommand() {
        ("list", _) => list_tools(config_filepath),
        ("run", _) => run_updates(config_filepath),
        ("add", Some(add_matches)) => {
            add_command(config_filepath, add_matches.value_of("input").unwrap())
        }
        ("remove", Some(remove_matches)) => {
            remove_command(config_filepath, remove_matches.value_of("command").unwrap())
                .expect("Unable to remove command from config file")
        }
        ("", None) => println!("No subcommand was used"), // If no subcommand was used it'll match the tuple ("", None)
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable!()
    }
}

fn list_tools(file_path: String) {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_path)
        .expect("Unable to open or create file at current directory");

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

fn run_updates(file_path: String) {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(file_path)
        .expect("Unable to open or create file at current directory");

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
                Ok(output) => println!("Output: {}", String::from_utf8_lossy(&output.stdout)),
                Err(_) => println!("<WARNING> Could not execute command: {}\n", l),
            };
        }
    }
}

fn add_command(file_path: String, command: &str) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_path)
        .expect("Unable to open or create file at current directory");

    write!(file, "{}\n", command).expect("Could not write to config file");
}

fn remove_command(file_path: String, command: &str) -> Result<(), std::io::Error> {
    match command {
        "all" | "." => {
            if confirm_selection() {
                // Try creating file, which truncates file if found
                File::create(&file_path)?;
            }
        }
        _ => {
            // Try to open file at file_path, and create file if not found
            let src_file = match File::open(&file_path) {
                Ok(file) => file,
                Err(_) => OpenOptions::new()
                    .write(true)
                    .read(true)
                    .create(true)
                    .open(&file_path)?,
            };
            // Existence of file has been verified already, so try reading and removing command
            let contents: Vec<String> = BufReader::new(&src_file)
                .lines()
                .map(|line| line.unwrap())
                .collect();
            drop(src_file); // Drop for re-opening in write mode

            let mut write_file = File::create(&file_path)?;
            for line in contents {
                // Filter through lines
                if !line.contains(command) {
                    writeln!(write_file, "{}", line)?;
                }
            }
        }
    }
    return Ok(());
}

fn confirm_selection() -> bool {
    return true;
}
