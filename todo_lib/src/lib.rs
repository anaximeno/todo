use std::error::Error;
use std::fmt;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_creation() {
        let task = Task::new(1, "This is a testing task.".to_string());
        assert_eq!(*task.id(), 1);
    }

    #[test]
    fn test_set_task_status() {
        let mut task = Task::new(1, String::from("Think about pineapples."));
        task.set_status(Status::Done(String::from("20-05-2022")));
        assert_eq!(*task.status(), "20-05-2022".into());
    }

    #[test]
    fn test_todo_creation() {
        let list_id = 4;
        let list = Todo::new(list_id, String::from("Things"));
        assert_eq!(*list.id(), list_id);
    }

    #[test]
    #[should_panic]
    fn test_add_repeated_tasks() {
        let mut list = Todo::new(1, "test".to_string());

        let _res = list.add_task(
            Task::new(1, "check err".to_string())
        ).unwrap();

        let _res = list.add_task(
            Task::new(1, "raise err".to_string())
        ).unwrap();
    }

    #[test]
    fn test_get_task_by_id_on_todo() {
        let mut list = Todo::new(1, "test".to_string());
        list.add_task(
            Task::new(23, "Test this task".to_string())
        ).unwrap();
        let task = list.get_task(23);
        assert_ne!(task, None);
    }
}

#[derive(Debug, PartialEq)]
/// Used to define the current status of
/// a task. The done pattern should be used to
/// store the date that the task was set as done.
pub enum Status {
    Done(String),
    Todo,
}

impl From<&str> for Status {
    fn from(date_completed: &str) -> Self {
        Status::Done(String::from(date_completed))
    }
}

impl From<String> for Status {
    fn from(date_completed: String) -> Self {
        Status::Done(date_completed)
    }
}

#[derive(Debug, PartialEq)]
/// A task is something the user
/// wants to do.
pub struct Task {
    id: u32,
    title: String,
    date_added: Option<String>,
    status: Status
}

#[derive(Debug, PartialEq)]
/// Todo structure used to store
/// a set of task to be done.
pub struct Todo {
    id:           u32,
    name:         String,
    description:  Option<String>,
    tasklist:     Vec<Task>,
}

#[derive(Debug)]
/// This error may be returned if one task is inserted
/// more than one time (repeating taskid) on the list.
pub struct TaskInsertionErr {
    details: String
}

impl TaskInsertionErr {
    fn from(msg: String) -> Self {
        Self{details: msg}
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
    /// Creates a new task by determining the id and the title,
    /// others fields are set to default.
    pub fn new(id: u32, title: String) -> Self {
        Self{id, title, date_added: None, status: Status::Todo}
    }

    /// Creates a new task with description.
    pub fn with_date(id: u32, title: String, date_added: String) -> Self {
        Self{id, title, date_added: Some(date_added), status: Status::Todo}
    }

    /// Creates a new task with a pre-defined status.
    pub fn with_status(id: u32, title: String, date_added: String, status: Status) -> Self {
        Self{id, title, date_added: Some(date_added), status}
    }

    /// Sets the task status to a new one.
    pub fn set_status(&mut self, status: Status) {
        self.status = status;
    }

    /// Sets a new date_added to the task.
    pub fn set_date_added(&mut self, date: String) {
        self.date_added = Some(date);
    }

    /// Sets a new title to the task.
    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    /// Returns the task's id.
    pub fn id(&self) -> &u32 {
        &self.id
    }
    
    /// Returns a reference to the date the task was added.
    pub fn date_added(&self) -> &Option<String> {
        &self.date_added
    }

    /// Returns a reference to the task's status.
    pub fn status(&self) -> &Status {
        &self.status
    }

    /// Returns a reference to the task's title.
    pub fn title(&self) -> &String {
        &self.title
    }
}

impl Todo {
    /// Creates a new list using its id and name
    pub fn new(id: u32, name: String) -> Self {
        Self{id, name, description: None, tasklist: Vec::new()}
    }
    
    /// Creates a new list with description
    pub fn with_description(id: u32, name: String, desc: String) -> Self {
        Self{id, name, description: Some(desc), tasklist: Vec::new()}
    }
    
    /// Sets a new value to the name.
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    
    /// Sets a new value to the description.
    pub fn set_description(&mut self, desc: String) {
        self.description = Some(desc);
    }

    /// Returns a reference to the id.
    pub fn id(&self) -> &u32 {
        &self.id
    }
    
    /// Returns a reference to the name.
    pub fn name(&self) -> &String {
        &self.name
    }
    
    /// Returns a reference to the description.
    pub fn description(&self) -> &Option<String> {
        &self.description
    }

    /// Adds a new task to the tasklist.
    pub fn add_task(&mut self, task: Task) -> Result<(), TaskInsertionErr> {
        match self.get_task(*task.id()) {
            Some(_) => {
                let msg = format!(
                    "Task id {} has already been inserted into the list!",
                    task.id()
                );
                Err(TaskInsertionErr::from(msg))
            },
            None => {
                self.tasklist.push(task);
                Ok(())
            }
        }
    }

    /// Returns the task's index
    fn get_task_index(&self, taskid: u32) -> Option<usize> {
        self.tasklist.iter().position(|t| *t.id() == taskid)
    }

    /// Gets the task by searching for its ID on the tasklist
    pub fn get_task(&self, taskid: u32) -> Option<&Task> {
        let index = self.get_task_index(taskid);
        if let Some(idx) = index { self.tasklist.get(idx) } else { None }
    }

    /// Gets the task by id, returning a mutable reference
    pub fn get_task_mut(&mut self, taskid: u32) -> Option<&mut Task> {
        let index = self.get_task_index(taskid);
        if let Some(idx) = index { self.tasklist.get_mut(idx) } else { None }
    }
}