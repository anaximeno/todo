use sqlite::Connection;

/// Database handler for the aplication
struct Database {
    path: String,
    conn: Connection
}

impl From<&str> for Database {
    /// Get's a new instance of the struct
    fn from(path: &str) -> Self {
        let path = String::from(path);
        let conn = Connection::open(&path).unwrap();
        Self{path, conn}
    }
}

impl Database {
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
}

fn main() {
    let db = Database::new();
    println!("Connected db at '{}'", db.path());
}
