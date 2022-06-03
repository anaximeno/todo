use sqlite::Connection;

struct Database {
    path: String,
    conn: Connection
}

impl Database {
    fn new(path: &str) -> Self {
        let path = String::from(path);
        let conn = Connection::open(&path).unwrap();
        Self{path, conn}
    }
}

fn main() {
    let db = Database::new(":memory:");
}
