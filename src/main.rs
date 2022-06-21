use todo::back::*;

use sqlite;

use clap::{
    Command,
    ArgMatches,
    Arg,
};

macro_rules! print_err {
    ($msg:tt) => {
        println!("Err: {}", $msg)
    };

    ($msg:tt, $exit_code:expr) => {
        println!("Err: {}", $msg);
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
}


fn main() {
    let app = App::new("TodoApp", "2.2.0");
    println!("Welcome to the {}, version {}", app.name(), app.version());
}
