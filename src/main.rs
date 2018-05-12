extern crate clap;
extern crate dialoguer;
extern crate indicatif;
extern crate open;

use indicatif::{ProgressBar, ProgressStyle};
use messages::{console, PrintError};
use std::fs::{create_dir_all, File, OpenOptions};
use std::io::prelude::*;
use std::io::{BufRead, BufReader};

mod app;
mod command;
mod messages;

fn main() {
    let app = app::create_app();
    let matches = app.get_matches();

    let config_directory = format!("{}/.upman/", std::env::home_dir().unwrap().display());
    create_dir_all(&config_directory).expect("Unable to create missing directories");
    let config_filepath = format!("{}upman.conf", config_directory);

    match matches.subcommand() {
        ("list", _) => list_tools(&config_filepath),
        ("add", Some(add_matches)) => {
            for m in add_matches.values_of("command").unwrap() {
                let a = add_matches.value_of("number").unwrap_or("").parse::<usize>();
                if add_command(&config_filepath, m, a.ok()).is_err() {
                    format!("Could not add command '{}' to the end of the config file", m).print_error();
                }
            }
        }
        ("remove", Some(remove_matches)) => {
            let arg_value = remove_matches.value_of("command").unwrap();
            if remove_matches.is_present("number") {
                if let Ok(line_number) = arg_value.parse::<usize>() {
                    if remove_command_line(&config_filepath, line_number).is_err() {
                        format!("Could not remove line {} from config file", line_number)
                            .print_error();
                    }
                } else {
                    unimplemented!()
                }
            } else {
                match arg_value {
                    "all" | "." => clear_commands(&config_filepath),
                    val => if remove_command_name(&config_filepath, val).is_err() {
                        format!("Unable to remove {} from config file", val).print_error();
                    },
                }
            }
        }
        ("run", run_matches) | ("", run_matches) => {
            // If no subcommand was used it'll match the tuple ("", None)
            let show_output = match run_matches {
                Some(set) => !set.is_present("silent"),
                None => true,
            };
            run_updates(&config_filepath, show_output);
        }
        ("open-config", _) => open_file(config_filepath),
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable!()
    }
}

fn list_tools(file_path: &String) {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_path)
        .expect("Unable to open or create file at current directory");

    let mut printed_prompt = false;
    let buf_reader = BufReader::new(file);

    for line in buf_reader.lines() {
        let l = line.unwrap_or(String::new());
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

fn run_updates(file_path: &String, show_output: bool) {
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
            pb.set_style(
                ProgressStyle::default_spinner()
                    .tick_chars("/|\\- ")
                    .template(&format!("{command_str} {{spinner}}", command_str = l)),
            );

            if !show_output {
                pb.enable_steady_tick(200);
            } else {
                pb.finish(); // Finishing spinner prints out the formatted message
            }

            let mut command_child_process = match command::Command::from(&l) {
                Some(mut val) => match val.run_command(show_output) {
                    Ok(result) => result,
                    Err(_) => {
                        vec![
                            "The current command could not be run.",
                            "Is the command itself correct?\n",
                        ].print_error();
                        continue;
                    }
                },
                None => {
                    format!("Could not create command from {}\n", l).print_error();
                    continue;
                }
            };

            loop {
                // Loop until the command has finished executing
                match command_child_process.try_wait() {
                    Ok(Some(_)) => {
                        if !show_output {
                            pb.finish();
                        }

                        match command_child_process.stdout.as_mut() {
                            Some(val) => {
                                let mut output = String::new();
                                val.read_to_string(&mut output)
                                    .expect("Unable to read result of command");
                                if show_output && output.trim().is_empty() {
                                    if !output.trim().is_empty() {
                                        println!("No output\n");
                                    }
                                }
                            }
                            None => println!(),
                        }
                        break; // Either way, the command did finish, so break here
                    }
                    Ok(None) => {
                        if !show_output {
                            pb.tick();
                        }
                    }
                    Err(_) => format!(
                        "Could not execute command: '{}'\n",
                        console::style(&l).yellow()
                    ).print_error(),
                }
            }
        }
    }
}

fn add_command(file_path: &String, command: &str, line_number: Option<usize>) -> Result<(), std::io::Error> {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_path)
        .expect("Unable to open or create file at current directory");

    let prefixed_command = format!("$ {}", command);

    if let Some(line_num) = line_number {
        
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
        let mut contents: Vec<String> = BufReader::new(&src_file)
            .lines()
            .map(|line| line.unwrap())
            .collect();
        drop(src_file); // Drop for re-opening in write mode

        let length = contents.len();
        println!("{} => {} lines long", line_num, length);

        // Vec<T> panics if remove is out of bounds, so check validity of line_number
        if 1<= line_num && line_num < length {
            contents.insert(line_num - 1, String::from(prefixed_command));
        } else if line_num > length {
            contents.push(String::from(prefixed_command));
        } else {
            format!("Line numbers must be greater than '{}'", messages::console::style("1")).print_error();
        }
        // Write the contents of `contents` regardless of success of remove
        write_contents(File::create(&file_path)?, contents.iter());

    } else {
        // The line_number arg was not passed, so just append to the end of the file
        write!(file, "{}", prefixed_command)?;
    }

    Ok(())
}

fn remove_command_line(file_path: &String, line_number: usize) -> Result<(), std::io::Error> {
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
    let mut contents: Vec<String> = BufReader::new(&src_file)
        .lines()
        .map(|line| line.unwrap())
        .collect();
    drop(src_file); // Drop for re-opening in write mode

    // Vec<T> panics if remove is out of bounds, so check validity of line_number
    let length = contents.len();
    if 1 <= line_number && line_number <= length {
        contents.remove(line_number - 1);
    } else {
        messages::StyledMessages::length_error(line_number, length);
    }
    // Write the contents of `contents` regardless of success of remove
    write_contents(File::create(&file_path)?, contents.iter());

    Ok(())
}

fn remove_command_name(file_path: &String, command: &str) -> Result<(), std::io::Error> {
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
    if let Some(i) = BufReader::new(&src_file)
        .lines()
        .position(|line| line.unwrap() == format!("$ {}", command))
    {
        drop(src_file); // Drop for re-opening in write mode
        remove_command_line(file_path, i + 1)?; // Add 1 to i, because line numbers are used instead of indexes
    } else {
        // If not in Some(i), the command does not exist, so ignore and return Ok(()) as well
        format!(
            "The command '{}' was not found",
            console::style(command).yellow()
        ).print_error();
    }

    Ok(())
}

fn write_contents<'a, I>(mut file: File, vals: I)
where
    I: IntoIterator<Item = &'a String>,
{
    // Create and write to file using iterator
    for val in vals {
        writeln!(file, "{}", val).unwrap();
    }
}

fn clear_commands(file_path: &String) {
    // Create confirmation with dialoguer
    let mut confirm =
        dialoguer::Confirmation::new("Are you sure you would like to clear the config file?");
    confirm.default(false);

    if confirm.interact().unwrap() {
        // Try creating file, which truncates file if found
        File::create(&file_path).expect("Unable to clear the config file");
    // TODO: migrate to a match, with backup method for clearing
    } else {
        println!("Cancelling...");
    }
}

fn open_file(file_path: String) {
    if open::that(&file_path).is_err() {
        format!("The file config file at '{}' could not be opened, as it does not exist or has read-only restrictions", console::style(file_path).yellow()).print_error();
    }
}
