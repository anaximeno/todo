use sqlite::Connection;
use todo::*;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_database_default_instantiation() {
        let db = Database::new();
        assert_eq!(db.path(), ":memory:");
    }

    #[test]
    fn test_database_instantiation_from_path() {
        let db = Database::from(":memory:");
        assert_eq!(db.path(), ":memory:");
    }

    #[test]
    fn test_app_instantiation() {
        let app = App::new("test-app", "0.0.1");
        assert_eq!(app.version(), "0.0.1");
        assert_eq!(app.name(), "test-app");

    }

    #[test]
    fn test_add_and_get_a_todo() {
        let mut app = App::new("test-app", "0.0.1");
        app.add_todo("test", "this a test todo!").unwrap();
        assert_ne!(app.get_todo("test"), None);
    }

    #[test]
    fn test_add_and_get_a_task() {
        let mut app = App::new("test-app", "0.0.1");
        app.add_todo("test-todo", "test use of tasks").unwrap();
        app.add_task("check if this todo is working", "test-todo").unwrap();
        assert_ne!(app.get_task(1), None);
    }
}

/// Database handler for the aplication
struct Database {
    path: String,
    conn: Connection
}

/// Representtodo, s th application
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
    fn new(name: &str, version: &str) -> Self {
        let db = Database::new();
        let name = String::from(name);
        let version = String::from(version);
        let this = Self{db, name, version};
        this.init_db().expect("Error Initializing the DB!");
        this
    }

    /// Creates an app with db set on a specific path
    fn with_db_path(name: &str, version: &str, db_path: &str) -> Self {
        let db = Database::from(db_path);
        let name = String::from(name);
        let version = String::from(version);
        let this = Self{db, name, version};
        this.init_db().expect("Error Initializing the DB!");
        this
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
    fn init_db(&self) -> Result<(), sqlite::Error> {
        self.db.exec("
        CREATE TABLE IF NOT EXISTS Todos(
            todo_id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            description TEXT NOT NULL
        );") ? ;
        self.db.exec("
        CREATE TABLE IF NOT EXISTS Tasks(
            task_id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
            task TEXT NOT NULL,
            todo_id INTEGER NOT NULL,
            date_added DATETIME NOT NULL DEFAULT CURRENT_DATE,
            date_completed DATETIME,
            FOREIGN KEY (todo_id) REFERENCES Todos(todo_id)
        );") ? ;
        self.db.exec("
        CREATE TABLE IF NOT EXISTS TaskOrder(
            todo_id INTEGER NOT NULL,
            task_id INTEGER NOT NULL,
            task_order INTEGER NOT NULL,
            FOREIGN KEY (todo_id) REFERENCES Todos(todo_id),
            FOREIGN KEY (task_id) REFERENCES Tasks(task_id),
            PRIMARY KEY (todo_id, task_id)
        );") ? ;
        Ok(())
    }

    /// Add a new todo to the database
    fn add_todo(&self, name: &str, description: &str) -> Result<(), sqlite::Error> {
        let statement = format!("
        INSERT INTO
            Todos(name, description)
        VALUES
            ('{}', '{}')", name, description);
        self.db.exec(&statement) ? ;
        Ok(())
    }

    /// Queries and returns if found a todo from the
    /// database using the name.
    fn get_todo(&mut self, name: &str) -> Option<Todo> {
        let query = format!("
        SELECT
            todo_id, description
        FROM
            Todos
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

    /// Add a new task to the database
    fn add_task(&mut self, task: &str, todo_name: &str) -> Result<(), sqlite::Error> {
        if let Some(todo) = self.get_todo(todo_name) {
            let statement = format!("
            INSERT INTO
                Tasks(task, todo_id)
            VALUES
                ('{}', {})", task, todo.id());
            self.db.exec(&statement) ? ;
        } else {
            self.add_todo(todo_name, "") ? ;
            self.add_task(task, todo_name) ? ;
        }
        Ok(())
    }

    

    /// Returns a task of the database if found
    fn get_task(&mut self, task_id: IdIntType) -> Option<Task> {
        let query = format!("
        SELECT
            task, date_added, date_completed
        FROM
            Tasks
        WHERE task_id = {}", task_id);
        if let Ok(mut cursor) = self.db.select_query(&query) {
            if let Some(task) = cursor.next().unwrap() {
                let task = create_task(
                    task_id, task[0].as_string().unwrap(),
                    task[1].as_string().unwrap(),
                    task[2].as_string());
                Some(task)    
            } else {
                None
            }
        } else {
            None
        }
    }
}

/// Gets the data from the query and creates a task
fn create_task(task_id: IdIntType, task: &str, date_added: &str, date_completed: Option<&str>) -> Task {
    let status = match date_completed {
        Some(date) => Status::from(date),
        None => Status::Todo
    };
    Task::with_status(task_id, task, date_added, status)
}

fn main() {
    let mut app = App::new("TodoApp", "0.1.0");

    app.add_todo("test", "testing todo list")
       .expect("Could not add a todo!");
    app.add_task("first test adding new tasks", "test")
       .expect("Could not add a task!");
       
    let mut todo = app.get_todo("test").unwrap();
    let task = app.get_task(1).unwrap();

    todo.add_task(task).unwrap();
    
    println!("{:#?}", todo);
}
