#![allow(unused)]
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
        let mut art = Artisan::new(":memory:");
        art.add_todo("test", "this a test todo!").unwrap();
        assert_ne!(art.get_todo("test"), None);
    }

    #[test]
    fn test_add_and_get_a_task_by_id() {
        let mut art = Artisan::new(":memory:");
        art.add_task("check", "test-todo").unwrap();
        assert_eq!(art.get_task_by_id(1).unwrap().task(), "check");
    }
}

/// Database handler for the aplication
struct Database {
    path: String,
    conn: Connection
}

/// Responsible for the interations
/// with the Database
struct Artisan {
    db: Database
}

/// Represents the todo application
struct App {
    name: String,
    version: String,
    artisan: Artisan,
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

impl Artisan {
    /// Creates a new artisan
    fn new(db_path: &str) -> Self {
        let this = Self{db: Database::from(db_path)};
        this.init_db().expect("Error Initializing the DB!");
        this
    }

    /// Returns a reference to the path of the db
    fn get_db_path(&self) -> &String {
        self.db.path()
    }

    /// Initializes the sqlite database with the default relations,
    /// if not already created.
    fn init_db(&self) -> Result<(), sqlite::Error> {
        self.db.exec("
        CREATE TABLE IF NOT EXISTS Todos(
            todo_id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
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
        Ok(())
    }

    /// Add a new todo to the database
    fn add_todo(&self, name: &str, description: &str) -> Result<(), sqlite::Error> {
        self.db.exec(&format!("INSERT INTO Todos(name, description) VALUES ('{}', '{}')", name, description)) ? ;
        Ok(())
    }

    /// Queries and returns if found a todo from the
    /// database using the name.
    fn get_todo(&mut self, name: &str) -> Option<Todo> {
        let result = self.db.select_query(&format!(
            "SELECT todo_id, description FROM
             Todos WHERE name = '{}'", name));
        if let Ok(mut cursor) = result {
            cursor.next().unwrap().map(|todo| {
                let todo_id = todo[0].as_integer().unwrap();
                let description = todo[1].as_string().unwrap();
                Todo::with_description(todo_id as IdIntType, name, description)
            })
        } else {
            None
        }
    }

    fn get_task_id(&mut self, task: &str) -> Option<IdIntType> {
        self.db
        .select_query(&format!("SELECT task_id FROM Tasks WHERE task = '{}'", task))
        .expect("Could not query for the task's id into the database!")
        .next()
        .unwrap()
        .map(|res| {
            let id = res[0].as_integer().unwrap();
            id as IdIntType
        })
    }

    fn insert_task_into_the_db(&mut self, task: &str, todo_id: IdIntType) -> Result<(), sqlite::Error> {
        self.db.exec(&format!("INSERT INTO Tasks(task, todo_id) VALUES('{}', {})", task, todo_id))
    }

    /// Add a new task to the database
    fn add_task(&mut self, task: &str, todo_name: &str) -> Result<(), &str> {
        if let Some(todo) = self.get_todo(todo_name) {
            let task_id = self.get_task_id(task);

            if let Some(id) = task_id {
                return Err("Task added more than one time to the todo!");
            }

            if let Err(_) = self.insert_task_into_the_db(task, *todo.id()) {
                return Err("Error inserting the task into the Database!");
            }
        } else {
            self.add_todo(todo_name, "").unwrap();
            self.add_task(task, todo_name) ? ;
        }
        Ok(())
    }

    /// Returns a task of the database if found
    fn get_task_by_id(&mut self, task_id: IdIntType) -> Option<Task> {
        let result = self.db.select_query(&format!(
            "SELECT task, date_added, date_completed FROM
             Tasks WHERE task_id = {}", task_id)
        );
        if let Ok(mut cursor) = result {
            cursor.next().unwrap().map(|task| {
                create_task(task_id,
                    task[0].as_string().unwrap(),
                    task[1].as_string().unwrap(),
                    task[2].as_string())
            })
        } else {
            None
        }
    }
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

/// Gets the data from the query and creates a task
fn create_task(task_id: IdIntType, task: &str, date_added: &str, date_completed: Option<&str>) -> Task {
    let status = match date_completed {
        Some(date) => Status::from(date),
        None => Status::Todo
    };
    Task::with_status(task_id, task, date_added, status)
}

fn main() {
    let mut app = App::new("TodoApp", "2.1.0");
    println!(
        "Welcome to the {}, version {}",
        app.name(), app.version()
    );
}
