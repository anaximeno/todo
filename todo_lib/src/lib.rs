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