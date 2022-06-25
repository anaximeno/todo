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
        let mut dao = TodoDatabaseDAO::new(":memory:");
        dao.add_todo("test", None).unwrap();
        let todo = dao.get_todo_by_name("test");
        assert_ne!(todo, None);
    }

    #[test]
    fn test_add_and_get_a_task_by_id() {
        let mut dao = TodoDatabaseDAO::new(":memory:");
        dao.add_task("testing task to test", "test").unwrap();
        let task = dao.get_task_by_id(1);
        assert_ne!(task, None);
        assert_eq!(task.unwrap().task(), "testing task to test");
    }

    #[test]
    fn test_get_all_tasks() {
        let mut dao = TodoDatabaseDAO::new(":memory:");
        dao.add_todo("test", Some("list of my test items")).unwrap();
        dao.add_task("test insertion 1", "test").unwrap();
        dao.add_task("test insertion 2", "test").unwrap();
        let tasks = dao.get_all_tasks();
        assert_ne!(tasks, None);
        assert_eq!(tasks.unwrap().len(), 2);
    }

    #[test]
    fn test_delete_task() {
        let mut dao = TodoDatabaseDAO::new(":memory:");
        dao.add_task("task this tesk", "test").unwrap();
        dao.delete_task(1).unwrap();
        assert_eq!(dao.get_all_tasks(), None);
    }

    #[test]
    fn test_delete_todo() {
        let mut dao = TodoDatabaseDAO::new(":memory:");
        dao.add_todo("test", None).unwrap();
        dao.delete_todo(1).unwrap();
        assert_eq!(dao.get_all_todos(), None);
    }

    #[test]
    fn test_update_task() {
        let mut dao = TodoDatabaseDAO::new(":memory:");
        dao.add_task("test tart", "test").unwrap();
        dao.update_task(1, "test task").unwrap();
        let task = dao.get_task_by_id(1).unwrap();
        assert_eq!(task.task(), "test task");
    }

    #[test]
    fn test_update_todo() {
        let mut dao = TodoDatabaseDAO::new(":memory:");
        dao.add_todo("tt", None).unwrap();
        dao.update_todo(1, "test", Some("testing todo")).unwrap();
        let todo = dao.get_todo_by_id(1).unwrap();
        assert_eq!(todo.name(), "test");
        assert_eq!(todo.description().unwrap(), "testing todo");
    }

    #[test]
    fn test_db_dao_add_and_get_todo() {
        
    }

    #[test]
    fn test_db_dao_get_todo_by_id() {
        let mut dao = TodoDatabaseDAO::new(":memory:");
        dao.add_todo("test", None).unwrap();
        let todo = dao.get_todo_by_id(1);
        assert_ne!(todo, None);
    }

    #[test]
    fn test_get_all_todos() {
        let mut dao = TodoDatabaseDAO::new(":memory:");
        dao.add_todo("test", None).unwrap();
        dao.add_todo("tast2", Some("testing add multiple todos")).unwrap();
        let todo = dao.get_all_todos().unwrap();
        assert_eq!(todo.len(), 2);
    }

    #[test]
    fn test_get_all_tasks_from_todo() {
        let mut dao = TodoDatabaseDAO::new(":memory:");
        dao.add_task("test insertion 1", "test").unwrap();
        dao.add_task("test insertion 2", "test").unwrap();
        dao.add_task("task from another todo 1", "another").unwrap();
        assert_eq!(dao.get_all_tasks_from_todo(1).unwrap().len(), 2);
        assert_eq!(dao.get_all_tasks_from_todo(2).unwrap().len(), 1);
    }

    #[test]
    fn test_get_todo_with_all_tasks() {
        let mut dao = TodoDatabaseDAO::new(":memory:");
        dao.add_task("test insertion 1", "test").unwrap();
        dao.add_task("test insertion 2", "test").unwrap();
        let todo = dao.get_todo_with_all_tasks(1).unwrap();
        assert_eq!(todo.number_of_tasks(), 2);
    }

    #[test]
    fn test_get_all_todos_with_all_tasks() {
        let mut dao = TodoDatabaseDAO::new(":memory:");
        dao.add_task("test insertion 1", "test").unwrap();
        dao.add_task("test insertion 2", "test").unwrap();
        dao.add_task("task from another todo 1", "another").unwrap();
        dao.add_task("Just one more", "lastone").unwrap();
        let todos = dao.get_all_todos_with_all_tasks().unwrap();
        assert_eq!(todos.get(0).unwrap().number_of_tasks(), 2);
        assert_eq!(todos.get(1).unwrap().number_of_tasks(), 1);
        assert_eq!(todos.get(2).unwrap().number_of_tasks(), 1);
    }
}

pub mod core {
    #![allow(unused)]

    use std::{
        error::Error,
        fmt
    };

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
    /// A task is something the user
    /// wants or have to do. They are stored
    /// inside the Todos.
    pub struct Task {
        task_id: IdType,
        task: String,
        date_added: Option<String>,
        status: Status
    }

    #[derive(Debug, PartialEq)]
    /// Todo is a structure used to store
    /// a set of task to be done.
    pub struct Todo {
        todo_id: IdType,
        name: String,
        description: Option<String>,
        tasks: Vec<Task>
    }

    #[derive(Debug)]
    /// This error may be returned if one task been inserted
    /// more than once inside the same Todo.
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
        /// Creates a new Task.
        pub fn new(task_id: IdType, task: &str) -> Self {
            let task = String::from(task);
            Self{task_id, task, date_added: None, status: Status::Todo}
        }

        /// Creates a new task with the date it was added.
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
        pub fn description(&self) -> Option<&String> {
            self.description.as_ref()
        }

        /// Sets the tasklist to another tasklisr
        pub fn set_tasks(&mut self, tasks: Vec<Task>) {
            self.tasks = tasks;
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

        pub fn get_task_at_index(&self, index: usize) -> Option<&Task> {
            self.tasks.get(index)
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

    /// Todo DAO trait
    pub trait TodoDAOLike {
        fn get_all_todos(&mut self) -> Option<Vec<Todo>>;
        fn get_todo_by_name(&mut self, name: &str) -> Option<Todo>;
        fn get_todo_by_id(&mut self, id: IdType) -> Option<Todo>;
        fn update_todo(&mut self, todo_id: IdType, name: &str, desc: Option<&str>) -> Result<(), sqlite::Error>;
        fn delete_todo(&mut self, todo_id: IdType) -> Result<(), sqlite::Error>;
        fn add_todo(&mut self, name: &str, desc: Option<&str>) -> Result<(), sqlite::Error>;
    }

    /// Task DAO trait
    pub trait TaskDAOLike {
        fn get_all_tasks(&mut self) -> Option<Vec<Task>>;
        fn get_task_id_from_db(&mut self, task: &str) -> Option<IdType>;
        fn get_task_by_id(&mut self, task_id: IdType) -> Option<Task>;
        fn update_task(&mut self, task_id: IdType, task: &str) -> Result<(), sqlite::Error>;
        fn delete_task(&mut self, task_id: IdType) -> Result<(), sqlite::Error>;
        fn add_task(&mut self, task: &str, todo_name: &str) -> Result<(), sqlite::Error>;
    }

    /// The complete todo app database DAO trait
    pub trait TodoDatabaseDAOLike: TodoDAOLike + TaskDAOLike {
        fn get_all_tasks_from_todo(&mut self, todo_id: IdType) -> Option<Vec<Task>>;
        fn get_todo_with_all_tasks(&mut self, todo_id: IdType) -> Option<Todo>;
        fn get_all_todos_with_all_tasks(&mut self) -> Option<Vec<Todo>>;
    }
    
    /// Data Access Object for the Todo Database
    pub struct TodoDatabaseDAO {
        db: Database
    }

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

    impl TodoDatabaseDAO {
        pub fn new(db_path: &str) -> Self {
            let this = Self{db: Database::from(db_path)};
            this.init_db().expect("Error Initializing the DB!");
            this
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
                todo_id INTEGER,
                date_added DATETIME NOT NULL DEFAULT CURRENT_DATE,
                date_completed DATETIME,
                FOREIGN KEY (todo_id) REFERENCES Todos(todo_id) ON DELETE SET NULL
            );") ? ;
            Ok(())
        }

        /// Returns a reference to the path of the db
        pub fn get_db_path(&self) -> &String {
            self.db.path()
        }
    }

    impl TodoDAOLike for TodoDatabaseDAO {
        fn add_todo(&mut self, name: &str, desc: Option<&str>) -> Result<(), sqlite::Error> {
            let desc = desc.unwrap_or("");
            self.db.exec(&format!("INSERT INTO Todos(name, description) VALUES ('{}', '{}')", name, desc)) ? ;
            Ok(())
        }

        fn get_todo_by_id(&mut self, id: IdType) -> Option<Todo> {
            self.db
            .select_query(&format!("SELECT name, description FROM Todos WHERE todo_id = '{}'", &id))
            .expect(&format!("Could not query the todo with id: '{}'", &id))
            .next()
            .unwrap()
            .map(|todo| {
                let name = todo[0].as_string().unwrap();
                let description = todo[1].as_string().unwrap();
                Todo::with_description(id, name, description)
            })
        }

        fn get_todo_by_name(&mut self, name: &str) -> Option<Todo> {
            self.db
            .select_query(&format!("SELECT todo_id, description FROM Todos WHERE name = '{}'", &name))
            .expect(&format!("Could not query the todo: '{}'", &name))
            .next()
            .unwrap()
            .map(|todo| {
                let id = todo[0].as_integer().unwrap() as IdType;
                let description = todo[1].as_string().unwrap();
                Todo::with_description(id, name, description)
            })
        }

        fn get_all_todos(&mut self) -> Option<Vec<Todo>> {
            let mut cursor = self.db
            .select_query("SELECT todo_id, name, description FROM Todos")
            .expect("Error quering for todos on the database!");
            let mut todos: Vec<Todo> = Vec::new();
            while let Some(mut result) = cursor.next().unwrap() {
                let id = result[0].as_integer().unwrap() as IdType;
                let name = result[1].as_string().unwrap();
                let desc = result[2].as_string().unwrap();
                todos.push(Todo::with_description(id, name, desc))
            }
            if todos.len() > 0 {
                Some(todos)
            } else {
                None
            }
        }

        fn update_todo(&mut self, todo_id: IdType, name: &str, desc: Option<&str>) -> Result<(), sqlite::Error> {
            self.db.exec(&format!("UPDATE Todos SET name = '{}' WHERE todo_id = {}", name, todo_id)) ? ;
            self.db.exec(&format!("UPDATE Todos SET description = '{}' WHERE todo_id = {}", desc.unwrap_or(""), todo_id))
        }

        fn delete_todo(&mut self, todo_id: IdType) -> Result<(), sqlite::Error> {
            self.db.exec(&format!("DELETE FROM Todos WHERE todo_id = {}", todo_id))
        }
    }

    fn gen_sqlite_err(message: &str, code: Option<isize>) -> sqlite::Error {
        let message = Some(String::from(message));
        sqlite::Error{message, code}
    }

    impl TaskDAOLike for TodoDatabaseDAO {
        fn add_task(&mut self, task: &str, todo_name: &str) -> Result<(), sqlite::Error> {
            if let Some(todo) = self.get_todo_by_name(todo_name) {
                let task_id = self.get_task_id_from_db(task);

                if let Some(id) = task_id {
                    return Err(gen_sqlite_err("Task id added more than once", Some(1)));
                }

                let result = self.db.exec(
                    &format!("INSERT INTO Tasks(task, todo_id) VALUES('{}', {})", task, todo.id())
                );
    
                if let Err(_) = result {
                    return Err(gen_sqlite_err("Error inserting a task into the Database!", Some(1)));
                }
            } else {
                if let Err(_) = self.add_todo(todo_name, None) {
                    return Err(gen_sqlite_err("Error trying to add todo to the database!", Some(1)));
                }
                self.add_task(task, todo_name) ? ;
            }
            Ok(())
        }

        fn get_task_id_from_db(&mut self, task: &str) -> Option<IdType> {
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
        
        fn get_task_by_id(&mut self, task_id: IdType) -> Option<Task> {
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

        fn get_all_tasks(&mut self) -> Option<Vec<Task>> {
            let mut cursor = self.db
            .select_query("SELECT task_id, task, date_added, date_completed FROM Tasks")
            .expect("Error quering for todos on the database!");
            let mut tasks: Vec<Task> = Vec::new();
            while let Some(mut result) = cursor.next().unwrap() {
                let id = result[0].as_integer().unwrap() as IdType;
                let task = result[1].as_string().unwrap();
                let date_added = result[2].as_string().unwrap();
                let status = match result[3].as_string() {
                    Some(date) => Status::Done(String::from(date)),
                    None => Status::Todo
                };
                tasks.push(Task::with_status(id, task, date_added, status));
            }
            if tasks.len() > 0 {
                Some(tasks)
            } else {
                None
            }
        }

        fn update_task(&mut self, task_id: IdType, task: &str) -> Result<(), sqlite::Error> {
            self.db.exec(&format!("UPDATE Tasks SET task = '{}' WHERE task_id = {}", task, task_id))
        }

        fn delete_task(&mut self, task_id: IdType) -> Result<(), sqlite::Error> {
            self.db.exec(&format!("DELETE FROM Tasks WHERE task_id = {}", task_id))
        }
    }

    impl TodoDatabaseDAOLike for TodoDatabaseDAO { // TODO
        fn get_all_tasks_from_todo(&mut self, todo_id: IdType) -> Option<Vec<Task>> {
            /** 
             * NOTE: Maybe use the method get all task, checking its todo_id to return the result
             * (keep in mind that the todo_id must be added to the struct Task first).
             * */
            let mut cursor = self.db
            .select_query(
                &format!("SELECT task_id, task, date_added, date_completed FROM Tasks WHERE todo_id = {}", todo_id)
            ).expect("Error quering for todos on the database!");
            let mut tasks: Vec<Task> = Vec::new();
            while let Some(mut result) = cursor.next().unwrap() {
                let id = result[0].as_integer().unwrap() as IdType;
                let task = result[1].as_string().unwrap();
                let date_added = result[2].as_string().unwrap();
                let status = match result[3].as_string() {
                    Some(date) => Status::Done(String::from(date)),
                    None => Status::Todo };
                tasks.push(Task::with_status(id, task, date_added, status));
            }
            if tasks.len() > 0 {
                Some(tasks)
            } else {
                None
            }
        }

        fn get_todo_with_all_tasks(&mut self, todo_id: IdType) -> Option<Todo> {
            if let Some(mut todo) = self.get_todo_by_id(todo_id) {
                if let Some(tasks) = self.get_all_tasks_from_todo(todo_id) {
                    todo.set_tasks(tasks);
                }
                Some(todo)
            } else {
                None
            }
        }

        fn get_all_todos_with_all_tasks(&mut self) -> Option<Vec<Todo>> {
            if let Some(mut todos) = self.get_all_todos() {
                for mut todo in &mut todos {
                    if let Some(tasks) = self.get_all_tasks_from_todo(*todo.id()) {
                        todo.set_tasks(tasks);
                    }
                }
                Some(todos)
            } else {
                None
            }
        }
    }
}