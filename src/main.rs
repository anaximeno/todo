use todo::prelude::*;

//TODO: Update main.rs

use clap::{
    Command,
    ArgMatches,
    Arg,
};

/// Represents the todo application
struct App {
    name: String,
    version: String
}

fn parse_args(app_name: &str, app_version: &str) -> ArgMatches {
    Command::new(app_name)
            .version(app_version)
            .author("Anaxímeno Brito")
            .about("Command-Line Todo Application.")
            .subcommand(
                Command::new("add")
                    .arg(
                        Arg::new("name")
                            .help("The name of the todo")
                            .takes_value(true)
                            .required(true)
                    )
                    .arg(
                        Arg::new("tasks")
                            .short('t')
                            .long("task")
                            .help("The task to be added to the todo list")
                            .takes_value(true)
                            .multiple_occurrences(true)
                            .multiple_values(true)
                    )
                    .arg(
                        Arg::new("description")
                            .short('d')
                            .long("desc")
                            .help("The Description of the todo")
                            .takes_value(true)
                    )
            ).subcommand(
                Command::new("done")
            ).get_matches()
}

impl App {
    /// Used to create an app
    fn new(name: &str, version: &str) -> Self {
        Self {name: String::from(name), version: String::from(version)}
    }

    /// References the name of this app
    fn name(&self) -> &String {
        &self.name
    }

    /// References the version of this app
    fn version(&self) -> &String {
        &self.version
    }

    /// Run main routine
    fn run(&mut self) {
        let args = parse_args(self.name(), self.version());

        match args.subcommand() {
            Some(("add", add_matches)) => {
                let name = add_matches.get_one::<String>("name")
                                                       .map(|s| String::from(s)).unwrap();

                let description = add_matches.get_one::<String>("description")
                                                             .map(|s| String::from(s));

                let todo = Todo::add(name, description).unwrap();

                println!("Todo added: {:#?}", todo);
            },
            _ => ()
        }

    }
}

fn main() {
    // XXX
    Todo::init_table().unwrap();
    Task::init_table().unwrap();

    let mut app = App::new("TodoApp", "2.2.0");
    app.run();
}
