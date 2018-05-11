#![allow(dead_code)]

use std;

#[derive(Debug)]
pub struct Command {
    full_command: String,
    runnable_command: std::process::Command,
}

impl Command {
    #[allow(dead_code)]
    const PREFIX: &'static str = "$ ";

    pub fn prefix_matches(command_str: &str) -> Vec<&str> {
        // Command without prefix
        let results: Vec<&str> = command_str
            .subset_left(Command::PREFIX)
            .split(Command::PREFIX)
            .collect();
        if results[0] == command_str {
            return Vec::new();
        } else {
            return results;
        }
    }

    pub fn first_prefix_match(command_str: &str) -> &str {
        let commands = Command::prefix_matches(command_str);
        if commands.len() < 1 {
            return "";
        } else {
            return commands[0];
        }
    }

    pub fn from(command: &str) -> Option<Command> {
        let single_command = Command::first_prefix_match(command);

        if single_command.is_empty() {
            return None;
        }

        let runnable_command = std::process::Command::from_str(single_command);

        Some(Command {
            full_command: String::from(single_command),
            runnable_command,
        })
    }

    pub fn run_command(
        &mut self,
        show_output: bool,
    ) -> Result<std::process::Child, std::io::Error> {
        if show_output {
            self.runnable_command.stdout(std::process::Stdio::inherit());
        } else {
            self.runnable_command.stdout(std::process::Stdio::null());
        }

        self.runnable_command.spawn()
    }
}

trait Subset {
    fn subset_left(&self, pattern: &str) -> &str;
    fn subset_right(&self, pattern: &str) -> &str;
    fn subset(&self, pattern: &str) -> &str;
}

impl Subset for str {
    fn subset_right(&self, pattern: &str) -> &str {
        if let Some(index) = self.rfind(pattern) {
            return &self[..index];
        } else {
            return self;
        }
    }

    fn subset_left(&self, pattern: &str) -> &str {
        if let Some(index) = self.find(pattern) {
            return &self[(index + pattern.len())..];
        } else {
            return self;
        }
    }

    fn subset(&self, pattern: &str) -> &str {
        return self.subset_left(pattern).subset_right(pattern);
    }
}

pub trait FromStr<T> {
    fn from_str(string: &str) -> T;
}

impl FromStr<std::process::Command> for std::process::Command {
    fn from_str(string: &str) -> std::process::Command {
        let mut str_iter = string.split(" ");

        let mut command: std::process::Command;

        if let Some(base_command) = str_iter.next() {
            command = std::process::Command::new(base_command);
        } else {
            panic!("The command passed did not contain a valid command");
        }

        for opt_arg in str_iter {
            if !opt_arg.trim().is_empty() {
                command.arg(opt_arg);
            }
        }

        return command;
    }
}
