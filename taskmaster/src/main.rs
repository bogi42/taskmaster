use tasks::{TaskError, TaskManager};
mod interactive;
use crate::interactive::InteractiveMode;

use clap::{Parser, Subcommand};
use colored::Colorize;
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    author,
    version,
    about,
    long_about = "A simple commandline task manager tool",
    after_help = "For more detailed help on a specific command, use:\n  taskmaster <COMMAND> --help \n  taskmaster help <COMMAND"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new task
    #[command(visible_alias = "a")]
    Add {
        /// The description of the task to be added
        #[arg(required = true)]
        description: Vec<String>,
    }, // Vec<String> to capture multiple words
    /// change description of Task
    #[command(visible_alias = "ch")]
    Change {
        /// The ID of the task you want to change
        #[arg(required = true)]
        id: usize,
        /// The new description for the task
        #[arg(required = true)]
        description: Vec<String>,
    },
    /// List all tasks
    #[command(visible_alias = "l")]
    List,
    /// Mark a task as completed
    #[command(visible_alias = "c")]
    Complete {
        /// The ID of the task to mark as complete
        #[arg(required = true)]
        id: usize,
    },
    /// Ranks up the task's priority
    Up {
        /// The ID of the task who's priority should be upranked
        id: usize,
    },
    /// Ranks down the task's priority
    Down {
        /// The ID of the task who's priority should be downranked
        id: usize,
    },
    /// Delete a task
    #[command(visible_alias = "d")]
    Delete {
        /// The ID of the task to delete
        #[arg(required = true)]
        id: usize,
    },
    /// Clear all completed task from the list
    #[command(visible_alias = "clr")]
    Clear,
    /// Changes into an interactive mode
    #[command(visible_alias = "i")]
    Interactive,
}

/* the work is done in run_app - main just encapsulates it and makes
 * sure the Display value of the returned Error is printed (instead of Debug)
 */
fn main() {
    if let Err(e) = run_app() {
        let ems = format!("Error: {}", e).red().bold();
        eprintln!("{}", ems); // macro uses Display by default!
        std::process::exit(1);
    }
}

fn run_app() -> Result<(), TaskError> {
    // 0. parse Arguments
    let cli = Cli::parse();

    // 1. determine file path and create new TaskManager from it
    let todo_file_path = get_todo_file_path()?;
    let mut task_manager = TaskManager::new(todo_file_path);
    task_manager.load_tasks()?;

    // 2. work on given command
    match &cli.command {
        Commands::Add { description } => {
            let desc_str = build_description(description)?;
            let new_index = task_manager.add_task(desc_str);
            println!(
                "Added Task #{}: {}",
                new_index,
                task_manager.at(new_index).unwrap().get_description()
            );
        }
        Commands::Change { id, description } => {
            let desc_str = build_description(description)?;
            let msg = task_manager.change_description(*id, desc_str)?;
            println!("{}", msg);
        }
        Commands::List => {
            task_manager.list_tasks();
        }
        Commands::Complete { id } => {
            let msg = task_manager.complete_task(*id)?;
            println!("{}", msg);
        }
        Commands::Up { id } => {
            let msg = task_manager.prioritize_task(*id)?;
            println!("{}", msg);
        }
        Commands::Down { id } => {
            let msg = task_manager.deprioritize_task(*id)?;
            println!("{}", msg);
        }
        Commands::Delete { id } => {
            let msg = task_manager.delete_task(*id)?;
            println!("{}", msg);
        }
        Commands::Clear => {
            let cleared_count = task_manager.clear_completed_tasks();
            println!("Cleared {} completed tasks", cleared_count);
        }
        Commands::Interactive => {
            let mut interactive_mode = InteractiveMode::new(&mut task_manager)?;
            interactive_mode.start_interactive_mode()?;
        }
    };

    // 3. save tasks at the end
    task_manager.save_tasks()?;

    Ok(()) // indicate succesful execution
}

fn get_todo_file_path() -> Result<PathBuf, TaskError> {
    let mut path = dirs::home_dir()
        .ok_or_else(|| TaskError::Unknown("Could not determine home directory".to_string()))?;
    path.push(".tasks.json");
    Ok(path)
}

fn build_description(description: &Vec<String>) -> Result<String, TaskError> {
    let desc_str = description.join(" ").trim().to_string();
    if !desc_str.is_empty() {
        Ok(desc_str)
    } else {
        Err(TaskError::Empty("Description".to_string()))
    }
}
