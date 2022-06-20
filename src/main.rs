use todo::back::*;

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
