use colored::Colorize;
use serde::{Deserialize, Serialize}; // import the traits
use std::fmt; // Display trait

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum Priority {
    Low,
    Medium,
    High,
}

/* default is needed - Priority is a new field and might not exist in older JSON files */
impl Default for Priority {
    fn default() -> Self {
        Priority::Medium
    }
}
/* this is how the Priority will be displayed */
impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Priority::Low => write!(f, "▼"),
            Priority::Medium => write!(f, "◆"),
            Priority::High => write!(f, "▲"),
        }
    }
}

/**** task_id: new field in version 0.3.0 */
/* default is needed for serde default, backwards compatibility */
fn default_task_id() -> usize {
    0
}

#[derive(Debug, Serialize, Deserialize)] // add Debug trait for easy printing during development
pub struct Task {
    #[serde(default = "default_task_id")]
    id: usize,
    description: String,
    completed: bool,
    #[serde(default)]
    priority: Priority,
}

impl Task {
    /// full-fledged Constructor
    pub fn new_task<S: Into<String>>(description: S, id: usize, priority: Priority) -> Self {
        Task {
            id,
            description: description.into(),
            completed: false,
            priority,
        }
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn set_id(&mut self, new_id: usize) {
        self.id = new_id;
    }

    /// ranks priority up
    pub fn prio_up(&mut self) {
        self.priority = match self.priority {
            Priority::Low => Priority::Medium,
            Priority::Medium => Priority::High,
            Priority::High => Priority::High,
        };
    }

    /// ranks priority down
    pub fn prio_down(&mut self) {
        self.priority = match self.priority {
            Priority::Low => Priority::Low,
            Priority::Medium => Priority::Low,
            Priority::High => Priority::Medium,
        };
    }

    pub fn get_priority(&self) -> colored::ColoredString {
        let prio_string = self.priority.to_string();
        match self.priority {
            Priority::Low => prio_string.green(),
            Priority::Medium => prio_string.yellow(),
            Priority::High => prio_string.red(),
        }
    }

    pub fn set_description<S: Into<String>>(&mut self, description: S) {
        self.description = description.into();
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    pub fn mark_completed(&mut self) {
        self.completed = true;
    }

    pub fn get_completed(&self) -> bool {
        self.completed
    }

    pub fn get_status(&self) -> &str {
        if self.completed {
            "[✓]"
        } else {
            "[·]"
        }
    }
}
