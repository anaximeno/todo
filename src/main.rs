use todo::back::*;

use sqlite;

use clap::{
    Command,
    ArgMatches,
    Arg,
};

macro_rules! printerr {
    ($msg:tt) => {
        println!("Todo: Err: {}", $msg)
    };

    ($msg:tt, $exit_code:expr) => {
        println!("ToDo: Err: {}", $msg);
        // TODO: Check if $exit_code is an
        // integer type.
        std::process::exit($exit_code);
    };
}

/// Represents the todo application
struct App {
    name: String,
    version: String,
    dao: TodoDatabaseDAO,
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
                    .arg(
                        Arg::new("ref")
                        .takes_value(true)
                        .required(true)
                        .help("References the todo or tasks. Usage \"todoname:tasknumber\" e.g. \"test:1\"")
                    )
            ).get_matches()
}

impl App {
    /// Used to create an app
    fn new(name: &str, version: &str) -> Self {
        Self {
            dao: TodoDatabaseDAO::new(":memory:"),
            name: String::from(name),
            version: String::from(version)
        }
    }

    /// References the name of this app
    fn name(&self) -> &String {
        &self.name
    }

    /// References the version of this app
    fn version(&self) -> &String {
        &self.version
    }

    fn add_todo(&mut self, name: &str, desc: Option<&str>, tasks: Option<Vec<&str>>) -> Result<(), sqlite::Error> {
        self.dao.add_todo(name, desc) ? ;
        if let Some(todo) = self.dao.get_todo_by_name(name) {
            if let Some(tasks) = tasks {
                for task in tasks {
                    self.dao.add_task(task, todo.name()) ? ;
                }
            }
        }
        Ok(())
    }

    /// Run main routine
    pub fn run(&mut self) {
        let args = parse_args(self.name(), self.version());

        match args.subcommand() {
            Some(("add", add_matches)) => {
                let name = add_matches.get_one::<String>("name").unwrap();

                let tasks = match add_matches.get_many::<String>("tasks") {
                    Some(tasks) => Some(tasks.map(|s| s.as_str()).collect::<Vec<_>>()),
                    None => None
                };

                let description = match add_matches.get_one::<String>("description") {
                    Some(desc) => Some(desc.as_str()),
                    None => None
                };

                if let Err(e) = self.add_todo(name, description, tasks) {
                    let msg = match e.message {
                        Some(msg) => msg,
                        None => format!("Could not add todo '{}' to the database!", name)
                    };
                    printerr!(msg, 1);
                }

                if let Some(todo) = self.dao.get_todo_with_all_tasks(1) {
                    println!("{:#?}", todo);
                }
            },
            Some(("done", done_matches)) => {
                let reference = done_matches.get_one::<String>("ref").unwrap();
                let splitted_ref: Vec<_> = reference.split(':').collect();

                if splitted_ref.len() == 1 {
                    // Only todo was given
                    let todo_name = splitted_ref[0];
                    let todo_id: &todo::core::IdType = self.dao.get_todo_by_name(todo_name).unwrap().id();
                    if let Some(todo) = self.dao.get_todo_with_all_tasks(*todo_id) {
                        println!("Setting all tasks on '{}' as done", todo.name());
                    }
                } else if splitted_ref.len() == 2 {
                    // Todo and task number
                } else {
                    let msg = format!("Invalid ref given: '{}'", reference);
                    printerr!(msg, 1);
                }
            },
            _ => ()
        }

    }
}

fn main() {
    let mut app = App::new("TodoApp", "2.2.0");
    app.run();
}
