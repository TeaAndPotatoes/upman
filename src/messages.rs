pub extern crate console;

use self::console::style;
use std::fmt::Display;

pub enum StyledMessages {
    Error,
}

impl StyledMessages {
    fn value(&self) -> console::StyledObject<&str> {
        match *self {
            StyledMessages::Error => style("error:").red().bold(),
        }
    }

    pub fn length_error(line_num: usize, length: usize) {
        if length == 0 {
            println!("        There are currently no command in the config file.");
            println!("        Use 'upman add <command>' to add a command");
        } else {
            let mut error = vec![
                format!(
                    "Command number '{}' is out of bounds of config file",
                    console::style(line_num).yellow()
                ),
            ];
            if line_num - length > 1 {
                error.push(format!(
                    "There are currently only {} command in the config file",
                    console::style(length).green()
                ));
            } else {
                error.push(format!(
                    "Command numbers start at '{}'",
                    console::style("1").green()
                ));
            }
            error.print_error();
        }
    }
}

pub trait PrintError {
    fn print_error(&self);
}

impl<T> PrintError for Vec<T>
where
    T: Display,
{
    fn print_error(&self) {
        if self.len() > 0 {
            println!("{} {}", StyledMessages::Error.value(), self[0]);

            if self.len() > 1 {
                for line in &self[1..] {
                    println!("        {}", line);
                }
            }
        }
    }
}

impl PrintError for str {
    fn print_error(&self) {
        println!("{} {}", StyledMessages::Error.value(), self);
    }
}

impl PrintError for String {
    fn print_error(&self) {
        println!("{} {}", StyledMessages::Error.value(), self);
    }
}
