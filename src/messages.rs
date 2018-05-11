pub extern crate console;

use self::console::style;
use std::fmt::Display;

enum StyledMessages {
    Error,
}

impl StyledMessages {
    fn value(&self) -> console::StyledObject<&str> {
        match *self {
            StyledMessages::Error => style("error:").red().bold(),
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
