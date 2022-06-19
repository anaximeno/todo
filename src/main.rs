use todo::back::Artisan;

/// Represents the todo application
struct App {
    name: String,
    version: String,
    artisan: Artisan,
}

impl App {
    /// Used to create an app
    fn new(name: &str, version: &str) -> Self {
        let name = String::from(name);
        let version = String::from(version);
        let artisan = Artisan::new(":memory:");
        Self{artisan, name, version}
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
    let app = App::new("TodoApp", "2.1.0");
    println!("Welcome to the {}, version {}", app.name(), app.version());
}
