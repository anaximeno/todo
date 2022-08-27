// #[cfg(test)]
// mod tests {
// }

pub mod prelude {
    pub use super::core::*;
    pub use super::data_access_layer::*;
}

mod database {
    use sqlite::{self, Connection};

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
            DB.exec_sttmt(&statement).is_ok()
        }
    }

    // TODO: create a mutex
    pub static DB: Database = Database::new(":memory:");
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

        fn init_table() {
            <Self as DatabaseConnector>::init_table();
        }
    }

    /// Todo Data Access Object Trait
    pub trait TodoDAO: DAO<ObjType = Todo> {
        fn all() -> Vec<Todo>;
        fn find(id: IdType) -> Result<Todo, InternalError>;
        fn add(name: String, description: Option<String>) -> Result<Todo, InternalError>;
        fn update(id: IdType, new_name: Option<String>, new_description: Option<String>) -> Result<Todo, InternalError>;
        fn delete(id: IdType) -> Result<(), InternalError>;
        fn tasks(&self) -> Vec<Task>;

        fn init_table() {
            <Self as DAO>::init_table();
        }
    }

    /// Task Data Access Object Trait
    pub trait TaskDAO: DAO<ObjType = Task> {
        fn all() -> Vec<Task>;
        fn find(id: IdType) -> Result<Task, InternalError>;
        fn add(what: String, todo_id: IdType) -> Result<Task, InternalError>;
        fn update(id: IdType, what_new: Option<String>, new_status: Option<Status>) -> Result<Task, InternalError>;
        fn delete(id: IdType) -> Result<(), InternalError>;
        // FIXME: change from option to normal
        fn todo(&self) -> Option<Todo>;

        fn init_table() {
            <Todo as DAO>::init_table();
            <Self as DAO>::init_table();
        }
    }


    impl DatabaseConnector for Todo {
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

            DB.create_table(&sttmt)
        }
    }

    impl DAO for Todo {
        type ObjType = Todo;

        fn all() -> Vec<Self::ObjType> {
            let mut todos: Vec<Todo> = Vec::new();

            if Self::is_table_initialized() {
                let query = format!("SELECT id, name, description FROM {}", Self::table_name());

                if let Ok(mut cursor) = DB.select_query(&query) {
                    while let Some(mut result) = cursor.next().unwrap() {
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

            let todo: Option<Todo> = if let Ok(mut cursor) = DB.select_query(&query) {
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

            let todo = <Self as DAO>::find(*obj.id()) ? ;

            /* Here what can be changed currently are the: name and description. */

            if obj.name() != todo.name() {
                let statement = format!(
                    "INSERT INTO {}(name, updated_at) VALUES ('{}', CURRENT_DATE);",
                    Self::table_name(), obj.name()
                );

                let res = DB.exec_sttmt(&statement);

                if let Err(e) = res {
                    return Err(InternalError::new(&e.to_string()));
                }
            }

            if obj.description() != todo.description() {
                let description = obj.description()
                                            .map(|desc| format!("'{}'", desc))
                                            .unwrap_or(String::from("NULL"));

                let statement = format!(
                    "INSERT INTO {}(description, updated_at) VALUES ({}, CURRENT_DATE);",
                    Self::table_name(), description
                );

                let res = DB.exec_sttmt(&statement);

                if let Err(e) = res {
                    return Err(InternalError::new(&e.to_string()));
                }
            }

            <Self as DAO>::find(*obj.id())
        }

        fn add(obj: Self::ObjType) -> Result<Todo, InternalError> {
            if !Self::is_table_initialized() {
                return Err(InternalError::table_not_initialized(&Self::table_name()));
            } else if let Ok(todo) = <Self as DAO>::find(*obj.id()) {
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

                let res = DB.exec_sttmt(&statement);

                if let Err(e) = res {
                    return Err(InternalError::new(&e.to_string()));
                }

                // FIXME: Not very useful in current environmnets since if another value is added before this
                // query, the wrong todo will be returned. Maybe consider using a mutex in the DB.
                let query = format!("
                    SELECT id, name, description, created_at, updated_at FROM {} WHERE id = (SELECT MAX(id) FROM {});",
                    Self::table_name(), Self::table_name()
                );

                let todo: Option<Todo> = if let Ok(mut cursor) = DB.select_query(&query) {
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
            let res = <Self as DAO>::find(id) ? ;
            let statement = format!("DELETE FROM {} WHERE id = {};", Self::table_name(), id);
            let res = DB.exec_sttmt(&statement);
            res.map_err(|e| InternalError::new(&e.to_string()))
        }
    }

    impl TodoDAO for Todo {
        fn all() -> Vec<Todo> {
            <Todo as DAO>::all()
        }

        fn add(name: String, description: Option<String>) -> Result<Todo, InternalError> {
            let id: IdType = 0;
            let created_at = "CURRENT_DATE";
            let updated_at = "CURRENT_DATE";

            let todo = Todo::new(id, name, description, created_at.into(), updated_at.into());

            <Todo as DAO>::add(todo)
        }

        fn update(id: IdType, new_name: Option<String>, new_description: Option<String>) -> Result<Todo, InternalError> {
            let mut todo = <Todo as DAO>::find(id) ? ;

            if let Some(name) = new_name {
                todo.set_name(&name);
            }

            if let Some(description) = new_description {
                todo.set_description(&description);
            }

            <Todo as DAO>::update(todo)
        }

        fn find(id: IdType) -> Result<Todo, InternalError> {
            <Todo as DAO>::find(id)
        }

        fn delete(id: IdType) -> Result<(), InternalError> {
            <Todo as DAO>::delete(id)
        }

        fn tasks(&self) -> Vec<Task> {
            // TODO
            Vec::new()
        }
    }

    impl DatabaseConnector for Task {
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

            DB.create_table(&sttmt)
        }
    }

    impl DAO for Task {
        type ObjType = Task;

        fn all() -> Vec<Self::ObjType> {
            let mut tasks: Vec<Task> = Vec::new();

            if Self::is_table_initialized() {
                let query = format!("SELECT id, what, todo_id, created_at, updated_at, completed_at FROM {};", Self::table_name());

                if let Ok(mut cursor) = DB.select_query(&query) {
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

            let task: Option<Task> = if let Ok(mut cursor) = DB.select_query(&query) {
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
            } else if let Ok(task) = <Self as DAO>::find(*obj.id()) {
                let details = format!("task with id = {}, is already in use in the table", obj.id());
                return Err(InternalError::new(&details));
            } else {
                let todo = <Todo as DAO>::find(*obj.todo_id()) ? ;

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

                let res = DB.exec_sttmt(&statement);

                if let Err(e) = res {
                    return Err(InternalError::new(&e.to_string()));
                }

                // FIXME: Not very useful in current environmnets since if another value is added before this
                // query, the wrong todo will be returned. Maybe consider using a mutex in the DB.
                let query = format!("
                    SELECT id, what, todo_id, created_at, updated_at, completed_at FROM {} WHERE id = (SELECT MAX(id) FROM {});",
                    Self::table_name(), Self::table_name()
                );

                let task: Option<Task> = if let Ok(mut cursor) = DB.select_query(&query) {
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

            let task = <Self as DAO>::find(*obj.id()) ? ;

            // Here what can be changed currently are: what, and status (actually completed_at date).

            if obj.what() != task.what() {
                let statement = format!(
                    "INSERT INTO {}(what, updated_at) VALUES ('{}', CURRENT_DATE);",
                    Self::table_name(), obj.what()
                );

                let res = DB.exec_sttmt(&statement);

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
                    "INSERT INTO {}(completed_at, updated_at) VALUES ({}, CURRENT_DATE);",
                    Self::table_name(), completed_at
                );

                let res = DB.exec_sttmt(&statement);

                if let Err(e) = res {
                    return Err(InternalError::new(&e.to_string()));
                }
            }

            <Self as DAO>::find(*obj.id())
        }

        fn delete(id: IdType) -> Result<(), InternalError> {
            let res = <Self as DAO>::find(id) ? ;
            let statement = format!("DELETE FROM {} WHERE id = {};", Self::table_name(), id);
            let res = DB.exec_sttmt(&statement);
            res.map_err(|e| InternalError::new(e.description()))
        }
    }

    impl TaskDAO for Task {
        fn all() -> Vec<Task> {
            <Task as DAO>::all()
        }

        fn find(id: IdType) -> Result<Task, InternalError> {
            <Task as DAO>::find(id)
        }

        fn delete(id: IdType) -> Result<(), InternalError> {
            <Task as DAO>::delete(id)
        }

        fn add(what: String, todo_id: IdType) -> Result<Task, InternalError> {
            let id: IdType = 0; // Only a placeholder
            let created_at = "CURRENT_DATE";
            let updated_at = "CURRENT_DATE";
            let status = Status::Todo;

            let task = Task::new(id, todo_id, &what, created_at, updated_at, status);

            <Task as DAO>::add(task)
        }

        fn update(id: IdType, what_new: Option<String>, new_status: Option<Status>) -> Result<Task, InternalError> {
            let mut task = <Task as DAO>::find(id) ? ;

            if let Some(what) = what_new {
                task.set_what(&what);
            }

            if let Some(status) = new_status {
                task.set_status(status);
            }

            <Task as DAO>::update(task)
        }

        fn todo(&self) -> Option<Todo> {
            // To Implement
            None
        }
    }
}
