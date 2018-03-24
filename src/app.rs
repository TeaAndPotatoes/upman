use clap::{App, Arg, SubCommand};

pub fn create_app() -> App<'static, 'static> {
    return App::new("Update Manager")
        .version("0.1.0-Beta")
        .author("Brendan Doney <bre.doney@gmail.com>")
        .about("Cross-platform, command-line update manager for easily checking and scheduling updates for different tools")
        .subcommand(SubCommand::with_name("list")
            .about("List all of the update commands added to this tool")
        )
        .subcommand(SubCommand::with_name("run")
            .about("Run each of the commands stored through add")
            .arg(Arg::with_name("show output")
                .short("o")
                .long("output")
                .help("Show the output of the command that is currently execusing")
            )
        )
        .subcommand(SubCommand::with_name("remove")
            .about("Remove a specific command from the configuration file")
            .arg(Arg::with_name("command")
                .required(true)
                .takes_value(true)
                .help("The command to remove from the config file")
            )
        )
        .subcommand(SubCommand::with_name("add")
            .about("Add a command to the list of tools to update")
            .arg(Arg::with_name("command")
                .required(true)
                .takes_value(true)
                .multiple(true)
                .help("The command to add to the update checker")
            )
        );
}
