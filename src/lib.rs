#![allow(unused)]

pub mod data_logic;

#[cfg(test)]
mod tests {
    use crate::data_logic;

    use super::core::*;
    use super::data_access_layer::*;
    use data_logic::db::*;

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
            *task.id(),
            Some("testing this task".to_string()),
            Some(Status::Done("CURRENT_TIMESTAMP".to_string())),
        )
        .unwrap();

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

    #[test]
    fn test_task_s_todo() {
        Todo::init_table().unwrap();
        Task::init_table().unwrap();

        let todo = Todo::add("test tasks".into(), None).unwrap();

        let task = Task::add("test task model".into(), *todo.id()).unwrap();

        let task_s_todo = task.todo().unwrap();

        assert_eq!(task_s_todo.id(), todo.id());
        assert_eq!(task_s_todo.name(), todo.name());
    }
}

pub mod prelude {
    pub use super::core::*;
    pub use super::data_access_layer::*;
}

mod core {
    pub use std::error::Error;
    use std::fmt::{self, Display};

    #[derive(Debug)]
    pub struct InternalError {
        details: String,
    }

    impl InternalError {
        pub fn new(details: &str) -> Self {
            Self {
                details: String::from(details),
            }
        }

        pub fn table_not_initialized(name: &str) -> Self {
            let details = format!("table '{}' was not initialed!", name);
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
        id: IdType,
        name: String,
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
        status: Status,
    }

    impl Task {
        pub fn new(
            id: IdType,
            todo_id: IdType,
            what: &str,
            created_at: &str,
            updated_at: &str,
            status: Status,
        ) -> Self {
            Self {
                id,
                todo_id,
                status,
                what: String::from(what),
                created_at: String::from(created_at),
                updated_at: String::from(updated_at),
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
        pub fn new(
            id: IdType,
            name: String,
            description: Option<String>,
            created_at: String,
            updated_at: String,
        ) -> Self {
            Self {
                id,
                name,
                description,
                created_at,
                updated_at,
            }
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
    
}
