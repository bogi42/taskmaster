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
}

impl TaskManager {
    pub fn new(file_path: PathBuf) -> Self {
        TaskManager {
            tasks: Vec::new(),
            file_path,
        }
    }

    // Load tasks from given file if possible
    pub fn load_tasks(&mut self) -> Result<(), TaskError> {
        if !self.file_path.exists() {
            self.tasks = Vec::new();
            return Ok(()); // No file, no problem - new vector;
        }

        let mut file = fs::File::open(&self.file_path)?;
        /* read entire file content into a string */
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        if contents.trim().is_empty() {
            self.tasks = Vec::new();
            return Ok(());
        }

        /* Deserialze the JSON string into Vec<Task>
         * the ? operator will propagate any serde_json::Error into io::Error
         */
        self.tasks = serde_json::from_str(&contents)?;
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
        let new_task = Task::new(description.into());
        self.tasks.push(new_task);
        self.tasks.len() // return user index of freshly created element (user index starts at 1, not 0)
    }

    /* show tasks */
    pub fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("{}", "No tasks, all done!".green());
        } else {
            /* calculate how many spaces should be used for the numbers. */
            let num_width = self.tasks.len() / 10 + 2;
            println!("{}", "Your tasks:".bold().underline());
            for (i, task) in self.tasks.iter().enumerate() {
                let index_str = format!("{1:>0$}", num_width, i + 1).cyan().bold();
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

    pub fn complete_task(&mut self, index: usize) -> Result<String, TaskError> {
        if let Some(task) = self.at_mut(index) {
            task.mark_completed();
            Ok(format!("Completed Task: {}", task.get_description()))
        } else {
            Err(TaskError::TaskNotFound(index))
        }
    }

    pub fn prioritize_task(&mut self, index: usize) -> Result<String, TaskError> {
        if let Some(task) = self.at_mut(index) {
            task.prio_up();
            Ok(format!("Prioritized Task: {}", task.get_description()))
        } else {
            Err(TaskError::TaskNotFound(index))
        }
    }

    pub fn deprioritize_task(&mut self, index: usize) -> Result<String, TaskError> {
        if let Some(task) = self.at_mut(index) {
            task.prio_down();
            Ok(format!("Deprioritized Task: {}", task.get_description()))
        } else {
            Err(TaskError::TaskNotFound(index))
        }
    }

    pub fn change_priority(&mut self, index: usize, prioritize: bool) -> Result<String, TaskError> {
        if prioritize {
            self.prioritize_task(index)
        } else {
            self.deprioritize_task(index)
        }
    }

    pub fn delete_task(&mut self, index: usize) -> Result<String, TaskError> {
        let actual_index = index.saturating_sub(1);
        if actual_index < self.tasks.len() {
            let deleted_task = self.tasks.remove(actual_index);
            Ok(format!("Deleted task: {}", deleted_task.get_description()))
        } else {
            Err(TaskError::TaskNotFound(index))
        }
    }

    /// Deletes all tasks that are marked as completed.
    /// Returns the number of tasks cleared.
    pub fn clear_completed_tasks(&mut self) -> usize {
        let initial_len = self.tasks.len();
        self.tasks.retain(|task| !task.get_completed());

        initial_len - self.tasks.len()
    }

    pub fn change_description<S: Into<String>>(
        &mut self,
        index: usize,
        new_description: S,
    ) -> Result<String, TaskError> {
        if let Some(task) = self.at_mut(index) {
            let old_desc: String = task.get_description().to_string();
            task.set_description(new_description);
            Ok(format!(
                "Description of task {} changed.\n\tOld: \"{}\"\n\tNew: \"{}\"",
                index,
                old_desc,
                task.get_description()
            ))
        } else {
            Err(TaskError::TaskNotFound(index))
        }
    }

    pub fn at(&self, index: usize) -> Option<&Task> {
        let actual_index = index.saturating_sub(1);
        self.tasks.get(actual_index)
    }

    pub fn at_mut(&mut self, index: usize) -> Option<&mut Task> {
        let actual_index = index.saturating_sub(1);
        self.tasks.get_mut(actual_index)
    }
}
