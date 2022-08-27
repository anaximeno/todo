#![allow(unused)]

#[cfg(test)]
mod tests {
    use super::database::*;
    use super::data_access_layer::*;
    use super::core::*;


    #[test]
    fn test_todo_add() {
        Todo::init_table().unwrap();

        let name = String::from("test");
        let description = Some(String::from("test cases for this app"));
        let todo = Todo::add(name.clone(), description).unwrap();

        assert_eq!(name, *todo.name());
    }

    #[test]
    fn test_todo_find() {
        Todo::init_table().unwrap();

        let name = String::from("test");
        let description = Some(String::from("test cases for this app"));
        let todo = Todo::add(name.clone(), description).unwrap();
        let res = Todo::find(*todo.id()).unwrap();

        assert_eq!(name, *todo.name());
        assert_eq!(todo.name(), res.name());
    }

    #[test]
    fn test_todo_update() {
        Todo::init_table().unwrap();

        let name = String::from("test");
        let description = Some(String::from("test cases for this app"));
        let todo = Todo::add(name.clone(), description).unwrap();
        let new_name = "Test Cases".to_string();
        let res = Todo::update(*todo.id(), Some(new_name.clone()), None).unwrap();

        assert_eq!(todo.id(), res.id());
        assert_ne!(&new_name, todo.name());
        assert_eq!(&new_name, res.name());
    }

    #[test]
    #[should_panic]
    fn test_todo_delete() {
        Todo::init_table().unwrap();

        let name = String::from("test");
        let description = Some(String::from("test cases for this app"));
        let todo = Todo::add(name.clone(), description).unwrap();

        assert_ne!(Todo::delete(*todo.id()).is_err(), true);

        /* Should Panic! */
        Todo::find(*todo.id()).unwrap();

    }

    #[test]
    fn test_todo_all() {
        Todo::init_table().unwrap();

        Todo::add("uno".into(), Some("the first number".to_string())).unwrap();
        Todo::add(String::from("dos"), None).unwrap();

        let todos = Todo::all();

        /* NOTE: Other tests are executed in the same context,
         * so, at least the number of todos in the list should be equal or
         * greater to the number of todos added above.
         * */
        assert!(todos.len() >= 2);
    }

    #[test]
    fn test_task_add() {
        Todo::init_table().unwrap();
        Task::init_table().unwrap();

        let todo = Todo::add("test tasks".into(), None).unwrap();
        let task = Task::add("test task model".into(), *todo.id()).unwrap();

        assert_eq!(task.todo_id(), todo.id());
        assert_eq!(*task.status(), Status::Todo);
        assert_eq!(task.what(), "test task model");
    }

    #[test]
    fn test_task_find() {
        Todo::init_table().unwrap();
        Task::init_table().unwrap();

        let todo = Todo::add("test tasks".into(), None).unwrap();
        let task = Task::add("test task model".into(), *todo.id()).unwrap();

        let res = Task::find(*task.id()).unwrap();

        assert_eq!(task.id(), res.id());
        assert_eq!(task.todo_id(), res.todo_id());
        assert_eq!(task.what(), res.what());
    }

    #[test]
    fn test_task_update() {
        Todo::init_table().unwrap();
        Task::init_table().unwrap();

        let todo = Todo::add("test tasks".into(), None).unwrap();

        let task = Task::add("test task model".into(), *todo.id()).unwrap();

        Task::update(
            *task.id(), Some("testing this task".to_string()),
            Some(Status::Done("CURRENT_DATE".to_string()))
        ).unwrap();

        let res = Task::find(*task.id()).unwrap();

        assert_eq!(res.id(), task.id());
        assert_ne!(res.what(), "test task model");
        assert_eq!(res.what(), "testing this task");
    }

    #[test]
    #[should_panic]
    fn test_task_delete() {
        Todo::init_table().unwrap();
        Task::init_table().unwrap();

        let todo = Todo::add("test tasks".into(), None).unwrap();

        let task = Task::add("test task model".into(), *todo.id()).unwrap();

        assert_ne!(Task::delete(*task.id()).is_err(), true);

        /* Should Panic! */
        Task::find(*task.id()).unwrap();
    }

    #[test]
    fn test_task_all() {
        Todo::init_table().unwrap();
        Task::init_table().unwrap();

        let todo1 = Todo::add("test tasks".into(), None).unwrap();
        let todo2 = Todo::add("test tasks again".into(), None).unwrap();

        Task::add("test task model 1 times".into(), *todo1.id()).unwrap();
        Task::add("test task model 2 times".into(), *todo1.id()).unwrap();
        Task::add("test task model once again".into(), *todo2.id()).unwrap();

        let tasks = Task::all();

        /* NOTE: Other tests are executed in the same context,
         * so, at least the number of todos in the list should be equal or
         * greater to the number of todos added above.
         * */
        assert!(tasks.len() >= 3);
    }

    #[test]
    fn test_todo_tasks() {
        Todo::init_table().unwrap();
        Task::init_table().unwrap();

        let todo1 = Todo::add("test tasks".into(), None).unwrap();
        let todo2 = Todo::add("test tasks again".into(), None).unwrap();

        Task::add("test task model 1 times".into(), *todo1.id()).unwrap();
        Task::add("test task model 2 times".into(), *todo1.id()).unwrap();
        Task::add("test task model once again".into(), *todo2.id()).unwrap();

        let todo1_tasks = todo1.tasks();

        assert_eq!(todo1_tasks.len(), 2);
    }
}

pub mod prelude {
    pub use super::core::*;
    pub use super::data_access_layer::*;
}

mod database {
    use sqlite::{self, Connection};
    use lazy_static::lazy_static;
    use std::sync::Mutex;

    /// Database handler for the aplication
    pub struct Database {
        path: String,
        connection: Connection
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
            Self{path, connection}
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
        pub static ref DB: Mutex<Database> = Mutex::new(Database::new(":memory:"));
    }

}

mod core {
    use std::fmt::{self, Display};
    pub use std::error::Error;

    #[derive(Debug)]
    pub struct InternalError {
        details: String
    }

    impl InternalError {
        pub fn new(details: &str) -> Self {
            Self{ details: String::from(details) }
        }

        pub fn table_not_initialized(name: &str) -> Self {
            let details = format!("table {} was not initialed!", name);
            Self::new(&details)
        }
    }

    impl Display for InternalError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.details)
        }
    }

    impl Error for InternalError {
        fn description(&self) -> &str {
            &self.details
        }
    }

    /// The type of the IDs used on the program.
    pub type IdType = u64;

    #[derive(Debug, PartialEq)]
    /// Used to define the current status of
    /// a task. The done pattern should be used to
    /// store the date that the task was set as done.
    pub enum Status {
        Done(String),
        Todo,
    }

    impl<T: AsRef<str>> From<T> for Status {
        fn from(date_completed: T) -> Self {
            Status::Done(String::from(date_completed.as_ref()))
        }
    }

    #[derive(Debug, PartialEq)]
    /// Todo is a structure used to store
    /// a set of task to be done.
    pub struct Todo {
        id:          IdType,
        name:        String,
        description: Option<String>,
        created_at: String, // TODO: Convert to datetime format
        updated_at: String,
    }

    #[derive(Debug, PartialEq)]
    /// A task is something the user
    /// wants or have to do. They are stored
    /// inside the Todos.
    pub struct Task {
        id: IdType,
        todo_id: IdType,
        what: String,
        created_at: String, // TODO: Convert to datetime format
        updated_at: String,
        status: Status
    }

    impl Task {
        pub fn new(id: IdType, todo_id: IdType, what: &str, created_at: &str, updated_at: &str, status: Status) -> Self {
            Self {
                id,
                todo_id,
                status,
                what: String::from(what),
                created_at: String::from(created_at),
                updated_at: String::from(updated_at)
            }
        }

        pub fn set_status(&mut self, status: Status) {
            self.status = status;
        }

        pub fn set_updated_at(&mut self, datetime: &str) {
            self.updated_at = String::from(datetime);
        }

        pub fn set_what(&mut self, what_new: &str) {
            self.what = String::from(what_new);
        }

        /// References the task's id.
        pub fn id(&self) -> &IdType {
            &self.id
        }

        pub fn todo_id(&self) -> &IdType {
            &self.todo_id
        }

        pub fn created_at(&self) -> &String {
            &self.created_at
        }

        pub fn updated_at(&self) -> &String {
            &self.updated_at
        }

        /// Reference the status of the task.
        pub fn status(&self) -> &Status {
            &self.status
        }

        /// References the task of the Task struct.
        pub fn what(&self) -> &String {
            &self.what
        }
    }

    impl Todo {
        pub fn new(id: IdType, name: String, description: Option<String>, created_at: String, updated_at: String) -> Self {
            Self { id, name, description, created_at, updated_at }
        }

        /// Sets a new value to the name.
        pub fn set_name(&mut self, name: &str) {
            self.name = String::from(name);
        }

        /// Sets a new value to the description.
        pub fn set_description(&mut self, description: &str) {
            self.description = Some(String::from(description));
        }

        pub fn set_updated_at(&mut self, datetime: &str) {
            self.updated_at = String::from(datetime);
        }

        /// Reference the id.
        pub fn id(&self) -> &IdType {
            &self.id
        }

        /// References the name.
        pub fn name(&self) -> &String {
            &self.name
        }

        /// References the description.
        pub fn description(&self) -> Option<&String> {
            self.description.as_ref()
        }
    }
}

mod data_access_layer {
    use super::database::*;
    use super::core::*;

    trait DAO: DatabaseConnector {
        type ObjType;

        fn all() -> Vec<Self::ObjType>;
        fn find(id: IdType) -> Result<Self::ObjType, InternalError>;
        fn add(obj: Self::ObjType) -> Result<Self::ObjType, InternalError>;
        fn update(obj: Self::ObjType) -> Result<Self::ObjType, InternalError>;
        fn delete(id: IdType) -> Result<(), InternalError>;
    }

    struct TodoDAO;
    struct TaskDAO;

    impl DatabaseConnector for TodoDAO {
        fn table_name() -> &'static str {
            "todos"
        }

        fn init_table() -> Result<(), sqlite::Error> {
            let sttmt = format!("{}(
                id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                description TEXT,
                created_at DATETIME NOT NULL DEFAULT CURRENT_DATE,
                updated_at DATETIME NOT NULL DEFAULT CURRENT_DATE);",
                Self::table_name()
            );

            DB.lock().unwrap().create_table(&sttmt)
        }
    }

    impl DAO for TodoDAO {
        type ObjType = Todo;

        fn all() -> Vec<Self::ObjType> {
            let mut todos: Vec<Todo> = Vec::new();

            if Self::is_table_initialized() {
                let query = format!("SELECT id, name, description, created_at, updated_at FROM {}", Self::table_name());

                if let Ok(mut cursor) = DB.lock().unwrap().select_query(&query) {
                    while let Some(result) = cursor.next().unwrap() {
                        let id: IdType = result[0].as_integer().unwrap() as IdType;
                        let name = result[1].as_string().unwrap();
                        let description = result[2].as_string().map(|desc| String::from(desc));
                        let created_at = result[3].as_string().unwrap();
                        let updated_at = result[4].as_string().unwrap();
                        todos.push(Todo::new(id, name.into(), description, created_at.into(), updated_at.into()))
                    }
                }
            }

            return todos;
        }

        fn find(id: IdType) -> Result<Self::ObjType, InternalError> {
            if !Self::is_table_initialized() {
                return Err(InternalError::table_not_initialized(&Self::table_name()));
            }

            let query = format!("
                SELECT id, name, description, created_at, updated_at FROM {} WHERE id = {}",
                Self::table_name(), id
            );

            let todo: Option<Todo> = if let Ok(mut cursor) = DB.lock().unwrap().select_query(&query) {
                cursor.next().unwrap().map(|t: &[sqlite::Value]| {
                    let id: IdType = t[0].as_integer().unwrap() as IdType;
                    let name = t[1].as_string().unwrap();
                    let description = t[2].as_string().map(|desc| String::from(desc));
                    let created_at = t[3].as_string().unwrap();
                    let updated_at = t[4].as_string().unwrap();
                    Todo::new(id, name.into(), description, created_at.into(), updated_at.into())
                })
            } else {
                None
            };

            let details = format!("todo with id = {} was not found in the database.", id);
            todo.ok_or(InternalError::new(&details))
        }

        fn update(obj: Self::ObjType) -> Result<Self::ObjType, InternalError> {
            if !Self::is_table_initialized() {
                return Err(InternalError::table_not_initialized(&Self::table_name()));
            }

            let todo = Self::find(*obj.id()) ? ;

            /* Here what can be changed currently are the: name and description. */

            if obj.name() != todo.name() {
                let statement = format!(
                    "UPDATE {} SET name = '{}', updated_at = CURRENT_DATE WHERE id = {};",
                    Self::table_name(), obj.name(), obj.id()
                );

                let res = DB.lock().unwrap().exec_sttmt(&statement);

                if let Err(e) = res {
                    return Err(InternalError::new(&e.to_string()));
                }
            }

            if obj.description() != todo.description() {
                let description = obj.description()
                                            .map(|desc| format!("'{}'", desc))
                                            .unwrap_or(String::from("NULL"));

                let statement = format!(
                    "UPDATE {} SET description = {}, updated_at = CURRENT_DATE WHERE id = {};",
                    Self::table_name(), description, obj.id()
                );

                let res = DB.lock().unwrap().exec_sttmt(&statement);

                if let Err(e) = res {
                    return Err(InternalError::new(&e.to_string()));
                }
            }

            Self::find(*obj.id())
        }

        fn add(obj: Self::ObjType) -> Result<Todo, InternalError> {
            if !Self::is_table_initialized() {
                return Err(InternalError::table_not_initialized(&Self::table_name()));
            } else if let Ok(todo) = Self::find(*obj.id()) {
                let details = format!("todo with id = {}, is already in use in the table", obj.id());
                return Err(InternalError::new(&details));
            } else {
                let name = obj.name();
                let description = obj.description()
                                            .map(|desc| format!("'{}'", desc))
                                            .unwrap_or(String::from("NULL"));

                let statement = format!(
                    "INSERT INTO {}(name, description) VALUES ('{}', {});",
                    Self::table_name(), name, description
                );

                let res = DB.lock().unwrap().exec_sttmt(&statement);

                if let Err(e) = res {
                    return Err(InternalError::new(&e.to_string()));
                }

                // FIXME: Not very useful in current environmnets since if another value is added before this
                // query, the wrong todo will be returned. Maybe consider using a mutex in the DB.lock().unwrap().
                let query = format!("
                    SELECT id, name, description, created_at, updated_at FROM {} WHERE id = (SELECT MAX(id) FROM {});",
                    Self::table_name(), Self::table_name()
                );

                let todo: Option<Todo> = if let Ok(mut cursor) = DB.lock().unwrap().select_query(&query) {
                    cursor.next().unwrap().map(|t: &[sqlite::Value]| {
                        let id: IdType = t[0].as_integer().unwrap() as IdType;
                        let name = t[1].as_string().unwrap();
                        let description = t[2].as_string().map(|desc| String::from(desc));
                        let created_at = t[3].as_string().unwrap();
                        let updated_at = t[4].as_string().unwrap();
                        Todo::new(id, name.into(), description, created_at.into(), updated_at.into())
                    })
                } else {
                    None
                };

                todo.ok_or(InternalError::new("Could not get the todo after adding it to the database."))
            }
        }

        fn delete(id: IdType) -> Result<(), InternalError> {
            let res = Self::find(id) ? ;
            let statement = format!("DELETE FROM {} WHERE id = {};", Self::table_name(), id);
            let res = DB.lock().unwrap().exec_sttmt(&statement);
            res.map_err(|e| InternalError::new(&e.to_string()))
        }
    }

    impl Todo {
        pub fn all() -> Vec<Todo> {
            TodoDAO::all()
        }

        pub fn add(name: String, description: Option<String>) -> Result<Todo, InternalError> {
            let id: IdType = 0;
            let created_at = "CURRENT_DATE";
            let updated_at = "CURRENT_DATE";

            let todo = Todo::new(id, name, description, created_at.into(), updated_at.into());

            TodoDAO::add(todo)
        }

        pub fn update(id: IdType, new_name: Option<String>, new_description: Option<String>) -> Result<Todo, InternalError> {
            let mut todo = TodoDAO::find(id) ? ;

            if let Some(name) = new_name {
                todo.set_name(&name);
            }

            if let Some(description) = new_description {
                todo.set_description(&description);
            }

            TodoDAO::update(todo)
        }

        pub fn find(id: IdType) -> Result<Todo, InternalError> {
            TodoDAO::find(id)
        }

        pub fn delete(id: IdType) -> Result<(), InternalError> {
            TodoDAO::delete(id)
        }

        pub fn tasks(&self) -> Vec<Task> {
            let mut tasks: Vec<Task> = Vec::new();

            if TodoDAO::is_table_initialized() && TaskDAO::is_table_initialized() {
                let query = format!(
                    "SELECT id, what, todo_id, created_at, updated_at, completed_at FROM {} WHERE todo_id = {}",
                    TaskDAO::table_name(), self.id()
                );

                if let Ok(mut cursor) = DB.lock().unwrap().select_query(&query) {
                    while let Some(result) = cursor.next().unwrap() {
                        let id: IdType = result[0].as_integer().unwrap() as IdType;
                        let what = result[1].as_string().unwrap();
                        let todo_id = result[2].as_integer().unwrap() as IdType;
                        let created_at = result[3].as_string().unwrap();
                        let updated_at = result[4].as_string().unwrap();
                        let status = result[5].as_string()
                                                      .map(|date| Status::Done(date.into()))
                                                      .unwrap_or(Status::Todo);

                        let task = Task::new(id, todo_id, what, created_at, updated_at, status);

                        tasks.push(task);
                    }
                }
            }

            return tasks;
        }

        pub fn init_table() -> Result<(), sqlite::Error>{
            TodoDAO::init_table() ? ;
            Ok(())
        }
    }

    impl DatabaseConnector for TaskDAO {
        fn table_name() -> &'static str {
            "tasks"
        }

        fn init_table() -> Result<(), sqlite::Error> {
            let sttmt = format!("{}(
                id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
                what TEXT NOT NULL,
                todo_id INTEGER NOT NULL,
                created_at DATETIME NOT NULL DEFAULT CURRENT_DATE,
                updated_at DATETIME NOT NULL DEFAULT CURRENT_DATE,
                completed_at DATETIME,
                FOREIGN KEY (todo_id) REFERENCES Todos(todo_id) ON DELETE SET NULL);",
                Self::table_name()
            );

            DB.lock().unwrap().create_table(&sttmt)
        }
    }

    impl DAO for TaskDAO {
        type ObjType = Task;

        fn all() -> Vec<Self::ObjType> {
            let mut tasks: Vec<Task> = Vec::new();

            if Self::is_table_initialized() {
                let query = format!("SELECT id, what, todo_id, created_at, updated_at, completed_at FROM {};", Self::table_name());

                if let Ok(mut cursor) = DB.lock().unwrap().select_query(&query) {
                    while let Some(mut result) = cursor.next().unwrap() {
                        let mut result: &[sqlite::Value] = result;

                        let id: IdType = result[0].as_integer().unwrap() as IdType;
                        let what = result[1].as_string().unwrap();
                        let todo_id = result[2].as_integer().unwrap() as IdType;
                        let created_at = result[3].as_string().unwrap();
                        let updated_at = result[4].as_string().unwrap();
                        let status = result[5].as_string()
                                                      .map(|date| Status::Done(date.into()))
                                                      .unwrap_or(Status::Todo);

                        let task = Task::new(id, todo_id, what, created_at, updated_at, status);

                        tasks.push(task);
                    }
                }
            }

            return tasks;
        }

        fn find(id: IdType) -> Result<Self::ObjType, InternalError> {
            if !Self::is_table_initialized() {
                return Err(InternalError::table_not_initialized(&Self::table_name()));
            }

            let query = format!(
                "SELECT id, what, todo_id, created_at, updated_at, completed_at FROM {} WHERE id = {};",
                Self::table_name(), id
            );

            let task: Option<Task> = if let Ok(mut cursor) = DB.lock().unwrap().select_query(&query) {
                cursor.next().unwrap().map(|t: &[sqlite::Value]| {
                    let id: IdType = t[0].as_integer().unwrap() as IdType;
                    let what = t[1].as_string().unwrap();
                    let todo_id = t[2].as_integer().unwrap() as IdType;
                    let created_at = t[3].as_string().unwrap();
                    let updated_at = t[4].as_string().unwrap();
                    let status = t[5].as_string()
                                             .map(|date| Status::Done(date.into()))
                                             .unwrap_or(Status::Todo);
                    Task::new(id, todo_id, what, created_at, updated_at, status)
                })
            } else {
                None
            };

            let details = format!("task with id = {} was not found in the database.", id);
            task.ok_or(InternalError::new(&details))
        }

        fn add(obj: Self::ObjType) -> Result<Self::ObjType, InternalError> {
            if !Self::is_table_initialized() {
                return Err(InternalError::table_not_initialized(&Self::table_name()));
            } else if let Ok(task) = Self::find(*obj.id()) {
                let details = format!("task with id = {}, is already in use in the table", obj.id());
                return Err(InternalError::new(&details));
            } else {
                let todo = TodoDAO::find(*obj.todo_id()) ? ;

                let todo_id = todo.id();
                let what = obj.what();
                let completed_at = match obj.status() {
                    Status::Done(date) => date,
                    Status::Todo => "NULL"
                };

                let statement = format!(
                    "INSERT INTO {}(todo_id, what, completed_at) VALUES ({}, '{}', {});",
                    Self::table_name(), todo_id, what, completed_at
                );

                let res = DB.lock().unwrap().exec_sttmt(&statement);

                if let Err(e) = res {
                    return Err(InternalError::new(&e.to_string()));
                }

                // FIXME: Not very useful in current environmnets since if another value is added before this
                // query, the wrong todo will be returned. Maybe consider using a mutex in the DB.
                let query = format!("
                    SELECT id, what, todo_id, created_at, updated_at, completed_at FROM {} WHERE id = (SELECT MAX(id) FROM {});",
                    Self::table_name(), Self::table_name()
                );

                let task: Option<Task> = if let Ok(mut cursor) = DB.lock().unwrap().select_query(&query) {
                    cursor.next().unwrap().map(|t: &[sqlite::Value]| {
                        let id: IdType = t[0].as_integer().unwrap() as IdType;
                        let what = t[1].as_string().unwrap();
                        let todo_id = t[2].as_integer().unwrap() as IdType;
                        let created_at = t[3].as_string().unwrap();
                        let updated_at = t[4].as_string().unwrap();
                        let status = t[5].as_string()
                                                        .map(|date| Status::Done(date.into()))
                                                        .unwrap_or(Status::Todo);
                        Task::new(id, todo_id, what, created_at, updated_at, status)
                    })
                } else {
                    None
                };

                task.ok_or(InternalError::new("Could not get the task after adding it to the database."))
            }
        }

        fn update(obj: Self::ObjType) -> Result<Self::ObjType, InternalError> {
            if !Self::is_table_initialized() {
                return Err(InternalError::table_not_initialized(&Self::table_name()));
            }

            let task = Self::find(*obj.id()) ? ;

            // Here what can be changed currently are: what, and status (actually completed_at date).

            if obj.what() != task.what() {
                let statement = format!(
                    "UPDATE {} SET what = '{}', updated_at = CURRENT_DATE WHERE id = {};",
                    Self::table_name(), obj.what(), obj.id()
                );

                let res = DB.lock().unwrap().exec_sttmt(&statement);

                if let Err(e) = res {
                    return Err(InternalError::new(&e.to_string()));
                }
            }

            if obj.status() != task.status() {
                let completed_at = match obj.status() {
                    Status::Done(date) => format!("'{}'", date), // Note the single collon
                    Status::Todo => String::from("NULL")
                };

                let statement = format!(
                    "UPDATE {} SET completed_at = {}, updated_at = CURRENT_DATE WHERE id = {};",
                    Self::table_name(), completed_at, obj.id()
                );

                let res = DB.lock().unwrap().exec_sttmt(&statement);

                if let Err(e) = res {
                    return Err(InternalError::new(&e.to_string()));
                }
            }

            Self::find(*obj.id())
        }

        fn delete(id: IdType) -> Result<(), InternalError> {
            let task = Self::find(id) ? ;
            let statement = format!("DELETE FROM {} WHERE id = {};", Self::table_name(), task.id());
            let res = DB.lock().unwrap().exec_sttmt(&statement);
            res.map_err(|e| InternalError::new(&e.to_string()))
        }
    }

    impl Task {
        pub fn all() -> Vec<Task> {
            TaskDAO::all()
        }

        pub fn find(id: IdType) -> Result<Task, InternalError> {
            TaskDAO::find(id)
        }

        pub fn delete(id: IdType) -> Result<(), InternalError> {
            TaskDAO::delete(id)
        }

        pub fn add(what: String, todo_id: IdType) -> Result<Task, InternalError> {
            let id: IdType = 0; // Only a placeholder
            let created_at = "CURRENT_DATE";
            let updated_at = "CURRENT_DATE";
            let status = Status::Todo;

            let task = Task::new(id, todo_id, &what, created_at, updated_at, status);

            TaskDAO::add(task)
        }

        pub fn update(id: IdType, what_new: Option<String>, new_status: Option<Status>) -> Result<Task, InternalError> {
            let mut task = TaskDAO::find(id) ? ;

            if let Some(what) = what_new {
                task.set_what(&what);
            }

            if let Some(status) = new_status {
                task.set_status(status);
            }

            TaskDAO::update(task)
        }

        pub fn todo(&self) -> Option<Todo> {
            // TODO: To Implement
            None
        }

        pub fn init_table() -> Result<(), sqlite::Error> {
            TodoDAO::init_table() ? ;
            TaskDAO::init_table() ? ;
            Ok(())
        }
    }
}
