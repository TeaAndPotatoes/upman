use std;

#[derive(Debug)]
struct Command {
    full_command: String,
    runnable_command: std::process::Command,
}

impl Command {
    #[allow(dead_code)]
    const PREFIX: &'static str = "$ ";

    pub fn parse_commands(command_str: &str) -> Vec<&str> {
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

    pub fn parse_first_command(command_str: &str) -> &str {
        let commands = Command::parse_commands(command_str);
        // let commands = parse_commands(command_str);
        if commands.len() < 1 {
            return "";
        } else {
            return commands[0];
        }
    }

    pub fn from(command: &str) -> Option<Command> {
        let full_command: String;
        let parsed = Command::parse_commands(command);
        if parsed.len() > 0 {
            full_command = String::from(parsed[0]);
        } else {
            return None;
        }

        let mut command_parts: Vec<&str> = full_command.split(" ").collect();
        let mut runnable_command = std::process::Command::new(&command_parts.remove(0));

        for part in command_parts {
            // Adding additional args from Vec, if any
            runnable_command.arg(part);
        }

        Some(Command {
            full_command: full_command.to_owned(),
            runnable_command,
        })
    }
}

pub trait Subset {
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
