use std::{
    error::Error,
    fmt
};

#[cfg(test)]
mod tests {
    use super::*;

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
}

/// The type of the id's used on the program.
pub type IdIntType = u64;

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
    task_id: IdIntType,
    task: String,
    date_added: Option<String>,
    status: Status
}

#[derive(Debug, PartialEq)]
/// Todo structure used to store
/// a set of task to be done.
pub struct Todo {
    todo_id: IdIntType,
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

#[allow(unused)]
impl TaskInsertionErr {
    
    fn new() -> Self {
        Self{details: "Task id was inserted more than once!".into()}
    }

    fn with_task_id(task_id: &IdIntType) -> Self {
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
    pub fn new(task_id: IdIntType, task: &str) -> Self {
        let task = String::from(task);
        Self{task_id, task, date_added: None, status: Status::Todo}
    }

    /// Creates a new task with description.
    pub fn with_date(task_id: IdIntType, task: &str, date_added: &str) -> Self {
        let task = String::from(task);
        let date_added = String::from(date_added);
        Self{task_id, task, date_added: Some(date_added), status: Status::Todo}
    }

    /// Creates a new task with a pre-defined status.
    pub fn with_status(task_id: IdIntType, task: &str, date_added: &str, status: Status) -> Self {
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
    pub fn id(&self) -> &IdIntType {
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
    pub fn new(todo_id: IdIntType, name: &str) -> Self {
        let name = String::from(name);
        Self{todo_id, name, description: None, tasks: Vec::new()}
    }
    
    /// Creates a new list with description
    pub fn with_description(todo_id: IdIntType, name: &str, desc: &str) -> Self {
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
    pub fn id(&self) -> &IdIntType {
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
    fn get_task_index(&self, taskid: IdIntType) -> Option<usize> {
        self.tasks.iter().position(|t| *t.id() == taskid)
    }

    /// Gets the task by searching for its ID on the task's list
    pub fn get_task(&self, taskid: IdIntType) -> Option<&Task> {
        let index = self.get_task_index(taskid);
        if let Some(idx) = index { self.tasks.get(idx) } else { None }
    }

    /// Gets the task by id, returning a mutable reference
    pub fn get_task_mut(&mut self, taskid: IdIntType) -> Option<&mut Task> {
        let index = self.get_task_index(taskid);
        if let Some(idx) = index { self.tasks.get_mut(idx) } else { None }
    }
}