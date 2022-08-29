use super::core::*;

struct TodoDAO;
struct TaskDAO;

trait DAO: DatabaseConnector {
    type ObjType;

    fn all() -> Vec<Self::ObjType>;
    fn find(id: IdType) -> Result<Self::ObjType, InternalError>;
    fn add(obj: Self::ObjType) -> Result<Self::ObjType, InternalError>;
    fn update(obj: Self::ObjType) -> Result<Self::ObjType, InternalError>;
    fn delete(id: IdType) -> Result<(), InternalError>;
}

impl DatabaseConnector for TodoDAO {
    fn table_name() -> &'static str {
        "todos"
    }

    fn init_table() -> Result<(), sqlite::Error> {
        if !Self::is_table_initialized() {
            let sttmt = format!(
                "{}(
                id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                description TEXT,
                created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP);",
                Self::table_name()
            );

            DB.lock().unwrap().create_table(&sttmt)
        } else {
            Ok(())
        }
    }
}

impl DAO for TodoDAO {
    type ObjType = Todo;

    fn all() -> Vec<Self::ObjType> {
        let mut todos: Vec<Todo> = Vec::new();

        if Self::is_table_initialized() {
            let query = format!(
                "SELECT id, name, description, created_at, updated_at FROM {}",
                Self::table_name()
            );

            if let Ok(mut cursor) = DB.lock().unwrap().select_query(&query) {
                while let Some(result) = cursor.next().unwrap() {
                    let id: IdType = result[0].as_integer().unwrap() as IdType;
                    let name = result[1].as_string().unwrap();
                    let description = result[2].as_string().map(|desc| String::from(desc));
                    let created_at = result[3].as_string().unwrap();
                    let updated_at = result[4].as_string().unwrap();
                    todos.push(Todo::new(
                        id,
                        name.into(),
                        description,
                        created_at.into(),
                        updated_at.into(),
                    ))
                }
            }
        }

        return todos;
    }

    fn find(id: IdType) -> Result<Self::ObjType, InternalError> {
        if !Self::is_table_initialized() {
            return Err(InternalError::table_not_initialized(&Self::table_name()));
        }

        let query = format!(
            "
            SELECT id, name, description, created_at, updated_at FROM {} WHERE id = {}",
            Self::table_name(),
            id
        );

        let todo: Option<Todo> = if let Ok(mut cursor) = DB.lock().unwrap().select_query(&query)
        {
            cursor.next().unwrap().map(|t: &[sqlite::Value]| {
                let id: IdType = t[0].as_integer().unwrap() as IdType;
                let name = t[1].as_string().unwrap();
                let description = t[2].as_string().map(|desc| String::from(desc));
                let created_at = t[3].as_string().unwrap();
                let updated_at = t[4].as_string().unwrap();
                Todo::new(
                    id,
                    name.into(),
                    description,
                    created_at.into(),
                    updated_at.into(),
                )
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

        /* Here what can be changed currently are the name and description. */

        if obj.name() != todo.name() {
            let statement = format!(
                "UPDATE {} SET name = '{}', updated_at = CURRENT_TIMESTAMP WHERE id = {};",
                Self::table_name(),
                obj.name(),
                obj.id()
            );

            let res = DB.lock().unwrap().exec_sttmt(&statement);

            if let Err(e) = res {
                return Err(InternalError::new(&e.to_string()));
            }
        }

        if obj.description() != todo.description() {
            let description = obj
                .description()
                .map(|desc| format!("'{}'", desc))
                .unwrap_or(String::from("NULL"));

            let statement =
                format!(
                "UPDATE {} SET description = {}, updated_at = CURRENT_TIMESTAMP WHERE id = {};",
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
            let details = format!(
                "todo with id = {}, is already in use in the table",
                obj.id()
            );
            return Err(InternalError::new(&details));
        } else {
            let name = obj.name();
            let description = obj
                .description()
                .map(|desc| format!("'{}'", desc))
                .unwrap_or(String::from("NULL"));

            let statement = format!(
                "INSERT INTO {}(name, description) VALUES ('{}', {});",
                Self::table_name(),
                name,
                description
            );

            // Using it this way will make it to only be unlocked
            // when the variable below is out of scope.
            let mut db = DB.lock().unwrap();

            let res = db.exec_sttmt(&statement);

            if let Err(e) = res {
                return Err(InternalError::new(&e.to_string()));
            }

            let query = format!("
                SELECT id, name, description, created_at, updated_at FROM {} WHERE id = (SELECT MAX(id) FROM {});",
                Self::table_name(), Self::table_name()
            );

            let todo: Option<Todo> = if let Ok(mut cursor) = db.select_query(&query) {
                cursor.next().unwrap().map(|t: &[sqlite::Value]| {
                    let id: IdType = t[0].as_integer().unwrap() as IdType;
                    let name = t[1].as_string().unwrap();
                    let description = t[2].as_string().map(|desc| String::from(desc));
                    let created_at = t[3].as_string().unwrap();
                    let updated_at = t[4].as_string().unwrap();
                    Todo::new(
                        id,
                        name.into(),
                        description,
                        created_at.into(),
                        updated_at.into(),
                    )
                })
            } else {
                None
            };

            todo.ok_or(InternalError::new(
                "Could not get the todo after adding it to the database.",
            ))
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
        let created_at = "CURRENT_TIMESTAMP";
        let updated_at = "CURRENT_TIMESTAMP";

        let todo = Todo::new(id, name, description, created_at.into(), updated_at.into());

        TodoDAO::add(todo)
    }

    pub fn update(
        id: IdType,
        new_name: Option<String>,
        new_description: Option<String>,
    ) -> Result<Todo, InternalError> {
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
                    let status = result[5]
                        .as_string()
                        .map(|date| Status::Done(date.into()))
                        .unwrap_or(Status::Todo);

                    let task = Task::new(id, todo_id, what, created_at, updated_at, status);

                    tasks.push(task);
                }
            }
        }

        return tasks;
    }

    pub fn init_table() -> Result<(), sqlite::Error> {
        TodoDAO::init_table() ? ;
        Ok(())
    }
}

impl DatabaseConnector for TaskDAO {
    fn table_name() -> &'static str {
        "tasks"
    }

    fn init_table() -> Result<(), sqlite::Error> {
        if !Self::is_table_initialized() {
            let sttmt = format!(
                "{}(
                id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
                what TEXT NOT NULL,
                todo_id INTEGER NOT NULL,
                created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
                completed_at DATETIME,
                FOREIGN KEY (todo_id) REFERENCES Todos(todo_id) ON DELETE SET NULL);",
                Self::table_name()
            );

            DB.lock().unwrap().create_table(&sttmt)
        } else {
            Ok(())
        }
    }
}

impl DAO for TaskDAO {
    type ObjType = Task;

    fn all() -> Vec<Self::ObjType> {
        let mut tasks: Vec<Task> = Vec::new();

        if Self::is_table_initialized() {
            let query = format!(
                "SELECT id, what, todo_id, created_at, updated_at, completed_at FROM {};",
                Self::table_name()
            );

            if let Ok(mut cursor) = DB.lock().unwrap().select_query(&query) {
                while let Some(mut result) = cursor.next().unwrap() {
                    let mut result: &[sqlite::Value] = result;

                    let id: IdType = result[0].as_integer().unwrap() as IdType;
                    let what = result[1].as_string().unwrap();
                    let todo_id = result[2].as_integer().unwrap() as IdType;
                    let created_at = result[3].as_string().unwrap();
                    let updated_at = result[4].as_string().unwrap();
                    let status = result[5]
                        .as_string()
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

        let task: Option<Task> = if let Ok(mut cursor) = DB.lock().unwrap().select_query(&query)
        {
            cursor.next().unwrap().map(|t: &[sqlite::Value]| {
                let id: IdType = t[0].as_integer().unwrap() as IdType;
                let what = t[1].as_string().unwrap();
                let todo_id = t[2].as_integer().unwrap() as IdType;
                let created_at = t[3].as_string().unwrap();
                let updated_at = t[4].as_string().unwrap();
                let status = t[5]
                    .as_string()
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
            let details = format!(
                "task with id = {}, is already in use in the table",
                obj.id()
            );
            return Err(InternalError::new(&details));
        } else {
            let todo = TodoDAO::find(*obj.todo_id()) ? ;

            let todo_id = todo.id();
            let what = obj.what();
            let completed_at = match obj.status() {
                Status::Done(date) => date,
                Status::Todo => "NULL",
            };

            let statement = format!(
                "INSERT INTO {}(todo_id, what, completed_at) VALUES ({}, '{}', {});",
                Self::table_name(),
                todo_id,
                what,
                completed_at
            );

            // Using it this way will make it to only be unlocked
            // when the variable below is out of scope.
            let mut db = DB.lock().unwrap();

            let res = db.exec_sttmt(&statement);

            if let Err(e) = res {
                return Err(InternalError::new(&e.to_string()));
            }

            let query = format!("
                SELECT id, what, todo_id, created_at, updated_at, completed_at FROM {} WHERE id = (SELECT MAX(id) FROM {});",
                Self::table_name(), Self::table_name()
            );

            let task: Option<Task> = if let Ok(mut cursor) = db.select_query(&query) {
                cursor.next().unwrap().map(|t: &[sqlite::Value]| {
                    let id: IdType = t[0].as_integer().unwrap() as IdType;
                    let what = t[1].as_string().unwrap();
                    let todo_id = t[2].as_integer().unwrap() as IdType;
                    let created_at = t[3].as_string().unwrap();
                    let updated_at = t[4].as_string().unwrap();
                    let status = t[5]
                        .as_string()
                        .map(|date| Status::Done(date.into()))
                        .unwrap_or(Status::Todo);
                    Task::new(id, todo_id, what, created_at, updated_at, status)
                })
            } else {
                None
            };

            task.ok_or(InternalError::new(
                "Could not get the task after adding it to the database.",
            ))
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
                "UPDATE {} SET what = '{}', updated_at = CURRENT_TIMESTAMP WHERE id = {};",
                Self::table_name(),
                obj.what(),
                obj.id()
            );

            let res = DB.lock().unwrap().exec_sttmt(&statement);

            if let Err(e) = res {
                return Err(InternalError::new(&e.to_string()));
            }
        }

        if obj.status() != task.status() {
            let completed_at = match obj.status() {
                Status::Done(date) => format!("'{}'", date), // Note the single collon
                Status::Todo => String::from("NULL"),
            };

            let statement = format!(
                "UPDATE {} SET completed_at = {}, updated_at = CURRENT_TIMESTAMP WHERE id = {};",
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
        let statement = format!(
            "DELETE FROM {} WHERE id = {};",
            Self::table_name(),
            task.id()
        );
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
        let created_at = "CURRENT_TIMESTAMP";
        let updated_at = "CURRENT_TIMESTAMP";
        let status = Status::Todo;

        let task = Task::new(id, todo_id, &what, created_at, updated_at, status);

        TaskDAO::add(task)
    }

    pub fn update(
        id: IdType,
        what_new: Option<String>,
        new_status: Option<Status>,
    ) -> Result<Task, InternalError> {
        let mut task = TaskDAO::find(id) ? ;

        if let Some(what) = what_new {
            task.set_what(&what);
        }

        if let Some(status) = new_status {
            task.set_status(status);
        }

        TaskDAO::update(task)
    }

    pub fn todo(&self) -> Result<Todo, InternalError> {
        Todo::find(*self.todo_id())
    }

    pub fn init_table() -> Result<(), sqlite::Error> {
        TodoDAO::init_table() ? ;
        TaskDAO::init_table() ? ;
        Ok(())
    }
}