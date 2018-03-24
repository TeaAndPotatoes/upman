extern crate clap;
extern crate dialoguer;
extern crate indicatif;

use std::io::{BufRead, BufReader};
use std::fs::{create_dir_all, File, OpenOptions};
use std::io::prelude::*;
use std::process;
use indicatif::{ProgressBar, ProgressStyle};

mod app;

fn main() {
    let app = app::create_app();
    let matches = app.get_matches();

    let config_directory = format!("{}/.upman/", std::env::home_dir().unwrap().display());
    create_dir_all(&config_directory).expect("Unable to create missing directories");
    let config_filepath = format!("{}upman.conf", config_directory);

    match matches.subcommand() {
        ("list", _) => list_tools(config_filepath),
        ("run", run_matches) => {
            let show_output = match run_matches {
                Some(set) => set.is_present("show"),
                None => false
            };
            run_updates(config_filepath, show_output);
        },
        ("add", Some(add_matches)) => {
            add_command(config_filepath, add_matches.value_of("command").unwrap())
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

    let mut printed_prompt = false;
    let buf_reader = BufReader::new(file);
    for line in buf_reader.lines() {
        let l = line.expect("Could not read line from file");
        if !l.trim().is_empty() && &l[..1] == "$" {
            if !printed_prompt {
                // Print the title for this command, if this is the first command
                println!("Registered commands:");
                println!("  Use \"upman add <command>\" to add a command to the list of command");
                println!("  Use \"upman remove <command>\" to remove a command from the list of commands\n");
                printed_prompt = true;
            }
            println!("      {}", &l);
        }
    }
    if !printed_prompt {
        println!("No commands are currently added to this tool\nAdd commands by using \"upman add <command>\"")
    } else {
        println!("\nUse \"upman run\" to run the listed commands");
    }
}

fn run_updates(file_path: String, show_output: bool) {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(file_path)
        .expect("Unable to open or create file at current directory");

    let buf_reader = BufReader::new(file);
    for line in buf_reader.lines() {
        let l = line.expect("Could not read line from file");
        if !l.trim().is_empty() && &l[..1] == "$" {
            let pb = ProgressBar::new_spinner();
            pb.enable_steady_tick(200);
            pb.set_style(
                ProgressStyle::default_spinner()
                    .tick_chars("/|\\- ")
                    .template(&format!("{command_str} {{spinner}}", command_str = l)),
            );
            let mut collection: Vec<&str> = l.split(" ").collect();
            collection.remove(0);

            // Take substring to cut off the command prompt from within the conf file
            let mut command = process::Command::new(&collection.remove(0));
            for part in collection {
                // Adding additional args from Vec, if any
                command.arg(part);
            }
            let mut spawned_command = command.stdout(process::Stdio::piped()).spawn().unwrap();

            loop {
                // Loop until the command has finished executing
                match spawned_command.try_wait() {
                    Ok(Some(_)) => {
                        pb.finish(); // Stop progress spinner, now that command is done
                        match spawned_command.stdout.as_mut() {
                            Some(val) => {
                                let mut output = String::new();
                                val.read_to_string(&mut output)
                                    .expect("Unable to read result of command");
                                if show_output {
                                    if !output.trim().is_empty() {
                                        println!("Output: {}", output);
                                    } else {
                                        println!("No output\n");
                                    }
                                }
                            }
                            None => println!(),
                        }
                        break; // Either way, the command did finish, so break here
                    }
                    Ok(None) => pb.tick(),
                    Err(_) => println!("<WARNING> Could not execute command: {}\n", l),
                }
            }
        }
    }
}

fn add_command(file_path: String, command: &str) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_path)
        .expect("Unable to open or create file at current directory");

    // The prompt for a command is a '$' mark at the beginning of the line
    write!(file, "$ {}\n", command).expect("Could not write to config file");
}

fn remove_command(file_path: String, command: &str) -> Result<(), std::io::Error> {
    match command {
        "all" | "." => {
            if confirm_selection("Are you sure you would like to clear the config file?") {
                // Try creating file, which truncates file if found
                File::create(&file_path)?;
            } else {
                println!("Cancelling...");
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

    Ok(())
}

fn confirm_selection(message: &str) -> bool {
    let mut confirm = dialoguer::Confirmation::new(message);
    confirm.default(false);
    confirm.interact().unwrap()
}
