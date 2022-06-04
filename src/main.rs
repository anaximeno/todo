use sqlite::Connection;

use todo::{
    Todo,
    IdIntType
};

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
        let db = Database::from(":memory:");
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

    /// Initializes the sqlite database with the default relations,
    /// if not already created.
    fn init_db(&self) {
        self.db.exec("
        CREATE TABLE IF NOT EXISTS Todo(
            todo_id  INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            description TEXT NOT NULL
        );").unwrap();
        self.db.exec("
        CREATE TABLE IF NOT EXISTS Task(
            task_id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            todo_id INTEGER,
            date_added DATETIME NOT NULL DEFAULT CURRENT_DATE,
            date_completed DATETIME,
            FOREIGN KEY (todo_id) REFERENCES Todo(todo_id)
        );").unwrap();
    }

    /// Add a new todo to the database
    fn add_todo(&self, name: &str, description: &str) {
        let statement = format!("
        INSERT INTO
            Todo(name, description)
        VALUES
            ('{}', '{}')", name, description);
        self.db.exec(&statement).unwrap();
    }

    /// Queries and returns if found a todo from the
    /// database using the name.
    fn get_todo(&mut self, name: &str) -> Option<Todo> {
        let query = format!("
        SELECT
            todo_id, description
        FROM
            Todo
        WHERE
            name = '{}'", name);
        if let Ok(mut cursor) = self.db.select_query(&query) {
            if let Some(value) = cursor.next().unwrap() {
                let todo_id = value[0].as_integer().unwrap();
                let description = value[1].as_string().unwrap();
                Some(Todo::with_description(todo_id as IdIntType, name, description))
            } else {
                None
            } 
        } else {
            None
        }
    }
}

fn main() {
    let mut app = App::new("TodoApp".into(), "0.1.0".into());

    app.add_todo("test", "testing todo list");

    let todo = app.get_todo("test");

    println!("{:#?}", todo);
}
