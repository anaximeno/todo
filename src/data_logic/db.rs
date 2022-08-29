use lazy_static::lazy_static;
use sqlite::{self, Connection};
use std::sync::{Arc, Mutex};
use std::thread;

/// Database handler for the aplication
pub struct Database {
    path: String,
    connection: Connection,
}

impl From<&str> for Database {
    /// Initializes the database from a given path
    fn from(path: &str) -> Self {
        Self::new(path)
    }
}

impl Database {
    pub fn new(path: &str) -> Self {
        let path = String::from(path);
        let connection = Connection::open(&path).unwrap();
        Self { path, connection }
    }

    /// References the path of the db
    pub fn path(&self) -> &String {
        &self.path
    }

    /// References the connection to the database
    pub fn connection(&self) -> &Connection {
        &self.connection
    }

    /// Executes a query command into the database
    pub fn exec_sttmt(&self, statement: &str) -> Result<(), sqlite::Error> {
        self.connection().execute(statement)
    }

    /// Executes a select query and returns a cursor
    pub fn select_query(&mut self, query: &str) -> Result<sqlite::Cursor, sqlite::Error> {
        Ok(self.connection.prepare(query)?.into_cursor())
    }

    pub fn create_table(&mut self, sttmt: &str) -> Result<(), sqlite::Error> {
        let statement = format!("CREATE TABLE IF NOT EXISTS {};", sttmt);
        self.exec_sttmt(&statement)
    }
}

pub trait DatabaseConnector {
    fn table_name() -> &'static str;
    fn init_table() -> Result<(), sqlite::Error>;

    fn is_table_initialized() -> bool {
        let statement = format!("SELECT * FROM {} LIMIT 1;", Self::table_name());
        DB.lock().unwrap().exec_sttmt(&statement).is_ok()
    }
}

lazy_static! {
    pub static ref DB: Arc<Mutex<Database>> = Arc::new(Mutex::new(Database::new(":memory:")));
}