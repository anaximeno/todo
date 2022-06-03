use sqlite::Connection;

/// Database handler for the aplication
struct Database {
    path: String,
    conn: Connection
}

/// Represents the todo application
struct App {
    db: Database,
    version: String,
    name: String
}

impl From<&str> for Database {
    /// Initializes the database from a given path
    fn from(path: &str) -> Self {
        let path = String::from(path);
        let conn = Connection::open(&path).unwrap();
        Self{path, conn}
    }
}

impl Database {
    /// Initializes the database from the memory
    fn new() -> Self {
        let path = String::from(":memory:");
        let conn = Connection::open(&path).unwrap();
        Self{path, conn}
    }

    /// References the path of the db
    fn path(&self) -> &String {
        &self.path
    }

    /// References the connection to the database
    fn connection(&self) -> &Connection {
        &self.conn
    }

    /// Executes a query command into the database
    fn exec(&mut self, statement: &str) -> Result<(), sqlite::Error> {
        self.conn.execute(statement)
    }
}

impl App {
    /// Used to create an app
    fn new(name: String, version: String) -> Self {
        let db = Database::new();
        Self{db, name, version}
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
    let app = App::new("TodoApp".into(), "1.0.0".into());
    println!("Just created the app {} at version {}", app.name(), app.version());
}
