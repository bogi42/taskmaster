use super::task::Task;
use super::task_error::TaskError;
use colored::Colorize;
use serde_json;
use std::fs;
use std::io::{Read, Write};
use std::path::PathBuf;

#[derive(Debug)]
pub struct TaskManager {
    tasks: Vec<Task>,
    file_path: PathBuf,
    next_available_id: usize,
}

impl TaskManager {
    pub fn new(file_path: PathBuf) -> Self {
        TaskManager {
            tasks: Vec::new(),
            file_path,
            next_available_id: 1,
        }
    }

    // Load tasks from given file if possible
    pub fn load_tasks(&mut self) -> Result<(), TaskError> {
        if !self.file_path.exists() {
            self.tasks = Vec::new();
            self.next_available_id = 1;
            return Ok(()); // No file, no problem - new vector;
        }

        let mut file = fs::File::open(&self.file_path)?;
        /* read entire file content into a string */
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        if contents.trim().is_empty() {
            self.tasks = Vec::new();
            self.next_available_id = 1;
            return Ok(());
        }

        /* Deserialze the JSON string into Vec<Task>
         * the ? operator will propagate any serde_json::Error into io::Error
         */
        self.tasks = serde_json::from_str(&contents)?;

        /* ID Renumberation logic: id was introduced in 0.3.0 - to be compatible with < 0.3.0,
         * the default value for ID is 0. Real ID is 1-based, so every id that euqals zero, needs
         * to be renumerated */
        let mut current_max_id = 0;
        /* First pass: Find the maximum ID already present */
        for task in &self.tasks {
            if task.get_id() > current_max_id {
                current_max_id = task.get_id();
            }
        }

        /* Second pass: Assign IDs to tasks with id == 0 and update max_id */
        for task in &mut self.tasks {
            if task.get_id() == 0 {
                current_max_id += 1;
                task.set_id(current_max_id);
            }
        }
        self.next_available_id = current_max_id + 1;
        Ok(())
    }

    // Save tasks to given file
    pub fn save_tasks(&self) -> Result<(), TaskError> {
        /* serialize the Vec<Task> into a pretty-printed JSON string */
        let json_string = serde_json::to_string_pretty(&self.tasks)?;
        /* write the JSOn string to the file, overwriting it */
        let mut file = fs::File::create(&self.file_path)?; // create ovverrides file if they exist
        file.write_all(json_string.as_bytes())?;
        Ok(())
    }

    /* creates a new task and adds it to the vector */
    pub fn add_task<S: Into<String>>(&mut self, description: S) -> usize {
        let new_id = self.next_available_id;
        let new_task = Task::new_task(description, self.next_available_id, crate::Priority::Medium);
        self.next_available_id += 1;
        self.tasks.push(new_task);
        new_id // return ID of newly created task
    }

    /* show tasks */
    pub fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("{}", "No tasks, all done!".green());
        } else {
            /* calculate how many spaces should be used for the numbers. */
            let num_width = self.next_available_id / 10 + 2;
            println!("{}", "Your tasks:".bold().underline());
            for task in &self.tasks {
                let index_str = format!("{1:>0$}", num_width, task.get_id()).cyan().bold();
                let status_str = task.get_status();
                let colored_status = if task.get_completed() {
                    status_str.green().bold()
                } else {
                    status_str.magenta()
                };
                let desc = task.get_description();
                let colored_desc = if task.get_completed() {
                    desc.dimmed()
                } else {
                    desc.normal()
                };
                println!(
                    "{}: {} {} {}",
                    index_str,
                    task.get_priority(),
                    colored_status,
                    colored_desc
                );
            }
        }
    }

    pub fn complete_task(&mut self, id: usize) -> Result<String, TaskError> {
        if let Some(task) = self.at_mut(id) {
            task.mark_completed();
            Ok(format!("Completed Task: {}", task.get_description()))
        } else {
            Err(TaskError::TaskNotFound(id))
        }
    }

    pub fn prioritize_task(&mut self, id: usize) -> Result<String, TaskError> {
        if let Some(task) = self.at_mut(id) {
            task.prio_up();
            Ok(format!("Prioritized Task: {}", task.get_description()))
        } else {
            Err(TaskError::TaskNotFound(id))
        }
    }

    pub fn deprioritize_task(&mut self, id: usize) -> Result<String, TaskError> {
        if let Some(task) = self.at_mut(id) {
            task.prio_down();
            Ok(format!("Deprioritized Task: {}", task.get_description()))
        } else {
            Err(TaskError::TaskNotFound(id))
        }
    }

    pub fn change_priority(&mut self, id: usize, prioritize: bool) -> Result<String, TaskError> {
        if prioritize {
            self.prioritize_task(id)
        } else {
            self.deprioritize_task(id)
        }
    }

    /// Deletes all tasks that are marked as completed.
    /// Returns the number of tasks cleared.
    pub fn clear_completed_tasks(&mut self) -> usize {
        let initial_len = self.tasks.len();
        self.tasks.retain(|task| !task.get_completed());

        initial_len - self.tasks.len()
    }

    /// Changes the description of a task with a given ID
    pub fn change_description<S: Into<String>>(
        &mut self,
        id: usize,
        new_description: S,
    ) -> Result<String, TaskError> {
        if let Some(task) = self.at_mut(id) {
            let old_desc: String = task.get_description().to_string();
            task.set_description(new_description);
            Ok(format!(
                "Description of task {} changed.\n\tOld: \"{}\"\n\tNew: \"{}\"",
                id,
                old_desc,
                task.get_description()
            ))
        } else {
            Err(TaskError::TaskNotFound(id))
        }
    }

    /// Deletes the task with the given ID
    pub fn delete_task(&mut self, id: usize) -> Result<String, TaskError> {
        if let Some(idx) = self.find_id(id) {
            let old_task = self.tasks.remove(idx);
            Ok(format!(
                "Deleted task ID {}\n\t'{}'",
                id,
                old_task.get_description()
            ))
        } else {
            Err(TaskError::TaskNotFound(id))
        }
    }

    /// find Task with given id, if it exits, and returns index
    pub fn find_id(&self, id: usize) -> Option<usize> {
        self.tasks.iter().position(|t| t.get_id() == id)
    }

    /// return Task with given id, if it exists
    pub fn at(&self, id: usize) -> Option<&Task> {
        self.tasks.iter().find(|t| t.get_id() == id)
    }

    /// return mutable Task with given id, if it exists
    pub fn at_mut(&mut self, id: usize) -> Option<&mut Task> {
        self.tasks.iter_mut().find(|t| t.get_id() == id)
    }
}
