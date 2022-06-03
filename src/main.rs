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
    fn exec(&self, statement: &str) -> Result<(), sqlite::Error> {
        self.connection().execute(statement)
    }

    /// Executes a select query and returns a cursor
    fn select_query(&mut self, query: &str) -> Result<sqlite::Cursor, &str> {
        if let Ok(res) = self.conn.prepare(query) {
            Ok(res.into_cursor())
        } else {
            Err("Error executing the query!")
        }
    }
}

impl App {
    /// Used to create an app
    fn new(name: String, version: String) -> Self {
        let db = Database::new();
        let s = Self{db, name, version};
        s.init_db();
        s
    }

    /// References the name of this app
    fn name(&self) -> &String {
        &self.name
    }

    /// References the version of this app
    fn version(&self) -> &String {
        &self.version
    }

    fn init_db(&self) {
        self.db.exec("
        CREATE TABLE IF NOT EXISTS TodoLists(
            todo_list_id  INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            description TEXT NOT NULL
        );").unwrap();
        self.db.exec("
        CREATE TABLE IF NOT EXISTS Task(
            task_id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            todo_list_id INTEGER,
            date_added DATETIME NOT NULL DEFAULT CURRENT_DATE,
            date_completed DATETIME
        );").unwrap();
    }
}

fn main() {
    let app = App::new("TodoApp".into(), "1.0.0".into());
    println!("Just created the app {} at version {}", app.name(), app.version());
}
