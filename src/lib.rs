#[cfg(test)]
mod tests {
    use super::core::*;
    use super::back::*;

    #[test]
    fn test_task_creation() {
        let task = Task::new(1, "This is a testing task.");
        assert_eq!(*task.id(), 1);
    }

    #[test]
    fn test_set_task_status() {
        let mut task = Task::new(1, "Think about pineapples.");
        task.set_status(Status::Done("20-05-2022".into()));
        assert_eq!(*task.status(), "20-05-2022".into());
    }

    #[test]
    fn test_todo_creation() {
        let list_id = 4;
        let list = Todo::new(list_id, "Things");
        assert_eq!(*list.id(), list_id);
    }

    #[test]
    #[should_panic]
    fn test_add_repeated_tasks() {
        let mut list = Todo::new(1, "test");

        let _res = list.add_task(
            Task::new(1, "check err")
        ).unwrap();

        let _res = list.add_task(
            Task::new(1, "raise err")
        ).unwrap();
    }

    #[test]
    fn test_get_task_by_id_on_todo() {
        let mut list = Todo::new(1, "test");
        list.add_task(
            Task::new(23, "Test this task")
        ).unwrap();
        assert_ne!(list.get_task(23), None);
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
        assert_eq!(art.get_task(1).unwrap().task(), "check");
    }

    #[test]
    fn test_get_todo_with_tasks() {
        let mut art = Artisan::new(":memory:");
        art.add_todo("test", "list of my test items").unwrap();
        art.add_task("test insertion 1", "test").unwrap();
        art.add_task("test insertion 2", "test").unwrap();
        let todos = art.get_todo_with_tasks("test").unwrap();
        assert_eq!(todos.number_of_tasks(), 2);
    }
}


pub mod core {
    #![allow(unused)]

    use std::{
        error::Error,
        fmt
    };

    /// The type of the id's used on the program.
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
    /// A task is something the user
    /// wants to do.
    pub struct Task {
        task_id: IdType,
        task: String,
        date_added: Option<String>,
        status: Status
    }

    #[derive(Debug, PartialEq)]
    /// Todo structure used to store
    /// a set of task to be done.
    pub struct Todo {
        todo_id: IdType,
        name: String,
        description: Option<String>,
        tasks: Vec<Task>
    }

    #[derive(Debug)]
    /// This error may be returned if one task is inserted
    /// more than one time (repeating taskid) on the list.
    pub struct TaskInsertionErr {
        details: String
    }

    impl TaskInsertionErr {
        
        fn new() -> Self {
            Self{details: "Task id was inserted more than once!".into()}
        }

        fn with_task_id(task_id: &IdType) -> Self {
            Self{details: format!("Task id {} was inserted more than once!", task_id)}
        }

        fn with_details(details: &str) -> Self {
            Self{details: details.into()}
        }
    }

    impl fmt::Display for TaskInsertionErr {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.details)
        }
    }

    impl Error for TaskInsertionErr {
        fn description(&self) -> &str {
            &self.details
        }
    }

    impl Task {
        /// Creates a new task by determining the id and the task,
        /// others fields are set to default.
        pub fn new(task_id: IdType, task: &str) -> Self {
            let task = String::from(task);
            Self{task_id, task, date_added: None, status: Status::Todo}
        }

        /// Creates a new task with description.
        pub fn with_date(task_id: IdType, task: &str, date_added: &str) -> Self {
            let task = String::from(task);
            let date_added = String::from(date_added);
            Self{task_id, task, date_added: Some(date_added), status: Status::Todo}
        }

        /// Creates a new task with a pre-defined status.
        pub fn with_status(task_id: IdType, task: &str, date_added: &str, status: Status) -> Self {
            let task = String::from(task);
            let date_added = String::from(date_added);
            Self{task_id, task, date_added: Some(date_added), status}
        }

        /// Sets the task status to a new one.
        pub fn set_status(&mut self, status: Status) {
            self.status = status;
        }

        /// Sets a new date_added to the task.
        pub fn set_date_added(&mut self, date: String) {
            self.date_added = Some(date);
        }

        /// Sets a new task to the task.
        pub fn set_task(&mut self, task: String) {
            self.task = task;
        }

        /// References the task's id.
        pub fn id(&self) -> &IdType {
            &self.task_id
        }
        
        /// References the date the task was added.
        pub fn date_added(&self) -> &Option<String> {
            &self.date_added
        }

        /// Reference the status of the task.
        pub fn status(&self) -> &Status {
            &self.status
        }

        /// References the task of the Task struct.
        pub fn task(&self) -> &String {
            &self.task
        }
    }

    impl Todo {
        /// Creates a new list using its id and name
        pub fn new(todo_id: IdType, name: &str) -> Self {
            let name = String::from(name);
            Self{todo_id, name, description: None, tasks: Vec::new()}
        }
        
        /// Creates a new list with description
        pub fn with_description(todo_id: IdType, name: &str, desc: &str) -> Self {
            let name = String::from(name);
            let desc = String::from(desc);
            Self{todo_id, name, description: Some(desc), tasks: Vec::new()}
        }
        
        /// Sets a new value to the name.
        pub fn set_name(&mut self, name: String) {
            self.name = name;
        }
        
        /// Sets a new value to the description.
        pub fn set_description(&mut self, desc: String) {
            self.description = Some(desc);
        }

        /// Reference the id.
        pub fn id(&self) -> &IdType {
            &self.todo_id
        }
        
        /// References the name.
        pub fn name(&self) -> &String {
            &self.name
        }
        
        /// References the description.
        pub fn description(&self) -> &Option<String> {
            &self.description
        }

        /// Adds a new task to the tasklist.
        pub fn add_task(&mut self, task: Task) -> Result<(), TaskInsertionErr> {
            if let Some(_) = self.get_task_index(*task.id()) {
                Err(TaskInsertionErr::with_task_id(task.id()))
            } else {
                self.tasks.push(task);
                Ok(())
            }
        }

        /// Returns the task's index
        fn get_task_index(&self, taskid: IdType) -> Option<usize> {
            self.tasks.iter().position(|t| *t.id() == taskid)
        }

        /// Gets the task by searching for its ID on the task's list
        pub fn get_task(&self, taskid: IdType) -> Option<&Task> {
            let index = self.get_task_index(taskid);
            if let Some(idx) = index { self.tasks.get(idx) } else { None }
        }

        /// Gets the task by id, returning a mutable reference
        pub fn get_task_mut(&mut self, taskid: IdType) -> Option<&mut Task> {
            let index = self.get_task_index(taskid);
            if let Some(idx) = index { self.tasks.get_mut(idx) } else { None }
        }

        pub fn number_of_tasks(&self) -> usize {
            self.tasks.len()
        }
    }
}

pub mod back {
    #![allow(unused)]

    use super::core::*;

    use sqlite::{
        self,
        Connection
    };
    
    /// Database handler for the aplication
    struct Database {
        path: String,
        conn: Connection
    }

    /// Responsible for interations
    /// with the Database
    pub struct Artisan {
        db: Database
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
        pub fn new(db_path: &str) -> Self {
            let this = Self{db: Database::from(db_path)};
            this.init_db().expect("Error Initializing the DB!");
            this
        }
    
        /// Returns a reference to the path of the db
        pub fn get_db_path(&self) -> &String {
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
        pub fn add_todo(&self, name: &str, description: &str) -> Result<(), sqlite::Error> {
            self.db.exec(&format!("INSERT INTO Todos(name, description) VALUES ('{}', '{}')", name, description)) ? ;
            Ok(())
        }
    
        /// Queries and returns if found a todo from the
        /// database using the name.
        pub fn get_todo(&mut self, name: &str) -> Option<Todo> {
            self.db
            .select_query(&format!("SELECT todo_id, description FROM Todos WHERE name = '{}'", name))
            .expect(&format!("Could not query the todo: '{}'", name))
            .next()
            .unwrap()
            .map(|todo| {
                let id = todo[0].as_integer().unwrap() as IdType;
                let description = todo[1].as_string().unwrap();
                Todo::with_description(id, name, description)
            })
        }

        pub fn get_todo_id(&mut self, name: &str) -> Option<IdType> {
            self.db
            .select_query(&format!("SELECT todo_id FROM Todos WHERE name = '{}'", name))
            .expect("Could not query for the todo's id from the database!")
            .next()
            .unwrap()
            .map(|res| {
                let id = res[0].as_integer().unwrap();
                id as IdType
            })
        }

        pub fn get_todo_with_tasks(&mut self, name: &str) -> Option<Todo> {
            if let Some(mut todo) = self.get_todo(name) {
                let mut cursor = self.db
                .select_query(&format!(
                    "SELECT task_id, task, date_added, date_completed
                    FROM Tasks WHERE todo_id = {}", todo.id()
                ))
                .unwrap();
                while let Some(res) = cursor.next().unwrap() {
                    let task_id = res[0].as_integer().unwrap();
                    let task = res[1].as_string().unwrap();
                    let date_added = res[2].as_string().unwrap();
                    let status = match res[3].as_string() {
                        Some(date) => Status::from(date),
                        None => Status::Todo };
                    todo.add_task(Task::with_status(
                        task_id as IdType,
                        task,
                        date_added,
                        status
                    ));
                }
                Some(todo)
            } else {
                None
            }
            
        }

        pub fn get_task_id(&mut self, task: &str) -> Option<IdType> {
            self.db
            .select_query(&format!("SELECT task_id FROM Tasks WHERE task = '{}'", task))
            .expect("Could not query for the task's id from the database!")
            .next()
            .unwrap()
            .map(|res| {
                let id = res[0].as_integer().unwrap();
                id as IdType
            })
        }
    
        fn insert_task_into_the_db(&mut self, task: &str, todo_id: IdType) -> Result<(), sqlite::Error> {
            self.db.exec(&format!("INSERT INTO Tasks(task, todo_id) VALUES('{}', {})", task, todo_id))
        }
    
        /// Add a new task to the database
        pub fn add_task(&mut self, task: &str, todo_name: &str) -> Result<(), &str> {
            if let Some(todo_id) = self.get_todo_id(todo_name) {
                let task_id = self.get_task_id(task);
    
                if let Some(id) = task_id {
                    return Err("Task added more than one time to the todo!");
                }
    
                if let Err(_) = self.insert_task_into_the_db(task, todo_id) {
                    return Err("Error inserting the task into the Database!");
                }
            } else {
                if let Err(_) = self.add_todo(todo_name, "") {
                    return Err("Error trying to add todo to the database!");
                }
                self.add_task(task, todo_name) ? ;
            }
            Ok(())
        }
    
        /// Returns a task of the database if found
        pub fn get_task(&mut self, task_id: IdType) -> Option<Task> {
            self.db
            .select_query(&format!("SELECT task, date_added, date_completed FROM Tasks WHERE task_id = {}", task_id))
            .unwrap()
            .next()
            .unwrap()
            .map(|res| {
                let task = res[0].as_string().unwrap();
                let date_added = res[1].as_string().unwrap();
                let status = match res[2].as_string() {
                    Some(date) => Status::from(date),
                    None => Status::Todo };
                Task::with_status(
                    task_id,
                    task,
                    date_added,
                    status
                )
            })
        }
    }
}