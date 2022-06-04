#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
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