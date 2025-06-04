use colored::Colorize;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use std::path::PathBuf;
use tasks::{TaskError, TaskManager};

/* this structure has a lifetime parameter - for the duration of its lifetime, there is a mutable
 * borrow of a reference to a TaskManager */
pub struct InteractiveMode<'a> {
    manager: &'a mut TaskManager,
    ed: DefaultEditor,
    history_path: Option<PathBuf>,
}

impl<'a> InteractiveMode<'a> {
    /// The new method can fail
    pub fn new(manager: &'a mut TaskManager) -> Result<Self, TaskError> {
        // 1. create a new Editor instance
        let mut rl = DefaultEditor::new()?;
        // Optional: load history from a file
        let history_path = dirs::home_dir().map(|mut path| {
            path.push(".taskmaster_history");
            path
        });

        // 2. optionally load history
        if let Some(path) = &history_path {
            if rl.load_history(path).is_err() {
                // ingore if history doesn't exit
            }
        }
        Ok(InteractiveMode {
            manager,
            ed: rl,
            history_path,
        })
    }

    fn print_interactive_help() {
        println!("{}", "\nInteractive Mode Commands:".bold().underline());
        println!("  {:<25} - {}", "l / list".cyan().bold(), "List all tasks");
        println!(
            "  {:<25} - {}",
            "a / add <desc>".cyan().bold(),
            "Add a new task"
        );
        println!(
            "  {:<25} - {}",
            "c / complete <id>".cyan().bold(),
            "Mark a task as completed"
        );
        println!(
            "  {:<25} - {}",
            "up / + <id>".cyan().bold(),
            "Increase a task's priority"
        );
        println!(
            "  {:<25} - {}",
            "down / - <id>".cyan().bold(),
            "Decrease a task's priority"
        );
        println!(
            "  {:<25} - {}",
            "d / delete <id>".cyan().bold(),
            "Delete a task"
        );
        println!(
            "  {:<25} - {}",
            "ch / change <id> <desc>".cyan().bold(),
            "Change a task's description"
        );
        println!(
            "  {:<25} - {}",
            "clr / clear".cyan().bold(),
            "Clear all completed tasks"
        );
        println!(
            "  {:<25} - {}",
            "h / help / ?".yellow().bold(),
            "Show this help message"
        );
        println!(
            "  {:<25} - {}",
            "q / quit / x / exit".red().bold(),
            "Exit interactive mode"
        );
        println!("");
    }

    pub fn start_interactive_mode(&mut self) -> Result<(), TaskError> {
        println!("Starting interactive mode. Type 'h' or 'help' for commands.");
        Self::print_interactive_help();

        loop {
            // 2. use rl.readline() instead of std::io::stdin().read_line()
            let input_result = self.read_input(&format!("{} ", "»".green().bold()));
            let input = match input_result {
                Ok(line) => line,
                Err(TaskError::InputCancelled) => {
                    println!("\n{}", "Exiting interactive mode.".yellow());
                    break;
                }
                Err(err) => {
                    eprintln!("{}", format!("Error reading input: {}", err).red());
                    return Err(err);
                }
            };

            if input.is_empty() {
                continue; // empty input just shows the prompt again
            }

            /* split input into commmand and arguments */
            let parts: Vec<&str> = input.split_whitespace().collect();
            if parts.is_empty() {
                continue; // ignore empty input
            }

            let command = parts[0].to_lowercase();
            let args = &parts[1..];

            let cmd_exec_result = match command.as_str() {
                "l" | "list" => self.handle_list(),
                "a" | "add" => self.handle_add(args),
                "c" | "complete" => self.handle_complete(args),
                "+" | "up" => self.handle_prio_change(args, true),
                "-" | "down" => self.handle_prio_change(args, false),
                "d" | "delete" => self.handle_delete(args),
                "ch" | "change" => self.handle_change(args),
                "clr" | "clear" => self.handle_clear(),
                "h" | "help" | "?" => {
                    Self::print_interactive_help();
                    Ok(())
                }
                "q" | "quit" | "x" | "exit" => break,
                _ => {
                    eprintln!("unknown command: '{}'. Type 'h' for help.", command);
                    Ok(()) // unknown commands don't stop the loop
                }
            };
            if let Err(e) = cmd_exec_result {
                eprintln!("{}", e.to_string().red());
            }
        }
        // Optional: save history to a file before exiting
        if let Some(path) = &self.history_path {
            if let Err(err) = self.ed.save_history(path) {
                eprintln!("{}", format!("Error saving history: {:?}", err).red());
            }
        }
        Ok(())
    }

    /// Returns the input from the user and True if there was a valid input; error message and False
    /// otherwise
    fn read_input(&mut self, prompt: &str) -> Result<String, TaskError> {
        match self.ed.readline(prompt) {
            Ok(line) => {
                self.ed.add_history_entry(line.as_str())?;
                Ok(line.trim().to_string())
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                Err(TaskError::InputCancelled)
            }
            Err(err) => Err(err.into()),
        }
    }

    /// Returns the input from the user and True if there was a valid input; error message and False
    /// otherwise
    fn read_input_initial(
        &mut self,
        prompt: &str,
        initial_text: &str,
    ) -> Result<String, TaskError> {
        match self.ed.readline_with_initial(prompt, (initial_text, "")) {
            Ok(line) => {
                self.ed.add_history_entry(line.as_str())?;
                Ok(line.trim().to_string())
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                Err(TaskError::InputCancelled)
            }
            Err(err) => Err(err.into()),
        }
    }

    fn handle_list(&mut self) -> Result<(), TaskError> {
        self.manager.list_tasks();
        Ok(())
    }

    fn handle_add(&mut self, args: &[&str]) -> Result<(), TaskError> {
        let desc: String;
        if args.is_empty() {
            // Sub-prompt:
            let prompt_result = self.read_input(&format!("{}> ", "Description".cyan()));
            match prompt_result {
                Ok(desc_line) => desc = desc_line,
                Err(TaskError::InputCancelled) => return Err(TaskError::InputCancelled),
                Err(TaskError::Readline(e)) => return Err(TaskError::Readline(e)),
                Err(err) => return Err(TaskError::Unknown(err.to_string())),
            }
        } else {
            desc = args.join(" ");
        }
        if desc.is_empty() {
            return Err(TaskError::Empty("Description".to_string()));
        }
        let index = self.manager.add_task(desc);
        println!("{}", format!("Added task with ID {}.", index).green());
        Ok(())
    }

    fn handle_complete(&mut self, args: &[&str]) -> Result<(), TaskError> {
        /* if args has the wrong length, or isn't a number, we'll get a subprompt from the user */
        let istr: String;
        if args.len() != 1 {
            let pr = self.read_input(&format!("{}> ", "ID".cyan()));
            match pr {
                Ok(s) => istr = s,
                Err(TaskError::InputCancelled) => return Err(TaskError::InputCancelled),
                Err(err) => return Err(err),
            }
        } else {
            istr = args[0].to_string();
        }
        match istr.parse::<usize>() {
            Ok(id) => match self.manager.complete_task(id) {
                Ok(msg) => println!("{}", msg.green()),
                Err(_) => return Err(TaskError::TaskNotFound(id)),
            },
            Err(_) => {
                return Err(TaskError::ArgumentMismatch(format!(
                    "wrong argument: '{}' is not a valid task ID.",
                    istr
                )))
            }
        }
        Ok(())
    }

    fn handle_prio_change(&mut self, args: &[&str], prioritize: bool) -> Result<(), TaskError> {
        /* if args has the wrong length, or isn't a number, we'll get a subprompt from the user */
        let istr: String;
        if args.len() != 1 {
            let pr = self.read_input(&format!("{}> ", "ID".cyan()));
            match pr {
                Ok(s) => istr = s,
                Err(TaskError::InputCancelled) => return Err(TaskError::InputCancelled),
                Err(err) => return Err(err),
            }
        } else {
            istr = args[0].to_string();
        }
        match istr.parse::<usize>() {
            Ok(id) => match self.manager.change_priority(id, prioritize) {
                Ok(msg) => println!("{}", msg.green()),
                Err(_) => return Err(TaskError::TaskNotFound(id)),
            },
            Err(_) => {
                return Err(TaskError::ArgumentMismatch(format!(
                    "wrong argument: '{}' is not a valid task ID.",
                    istr
                )))
            }
        }
        Ok(())
    }

    fn handle_delete(&mut self, args: &[&str]) -> Result<(), TaskError> {
        /* if args has the wrong length, or isn't a number, we'll get a subprompt from the user */
        let istr: String;
        if args.len() != 1 {
            let pr = self.read_input(&format!("{}> ", "ID".cyan()));
            match pr {
                Ok(s) => istr = s,
                Err(TaskError::InputCancelled) => return Err(TaskError::InputCancelled),
                Err(err) => return Err(err),
            }
        } else {
            istr = args[0].to_string();
        }
        match istr.parse::<usize>() {
            Ok(id) => match self.manager.delete_task(id) {
                Ok(msg) => println!("{}", msg.green()),
                Err(_) => return Err(TaskError::TaskNotFound(id)),
            },
            Err(_) => {
                return Err(TaskError::ArgumentMismatch(format!(
                    "wrong argument: '{}' is not a valid task ID.",
                    istr
                )))
            }
        }
        Ok(())
    }

    fn handle_change(&mut self, args: &[&str]) -> Result<(), TaskError> {
        /* part 1: check for index */
        let istr: String;
        if args.len() < 1 {
            match self.read_input(&format!("{}> ", "ID".cyan())) {
                Ok(s) => istr = s,
                Err(TaskError::InputCancelled) => return Err(TaskError::InputCancelled),
                Err(e) => return Err(e),
            }
        } else {
            istr = args[0].to_string();
        }
        /* parse the index string, handle invalid parsing */
        let id = istr.parse::<usize>().map_err(|_| {
            TaskError::ArgumentMismatch(format!(
                "wrong argument: '{}' is not a valid number for an index.",
                istr
            ))
        })?;
        /* part 2: temporarily store old description */
        let old_desc: String = {
            let task_ref = self.manager.at(id).ok_or(TaskError::TaskNotFound(id))?;
            task_ref.get_description().to_string()
        };
        /* part 3: check for new description */
        let new_desc: String;
        if args.len() < 2 {
            match self.read_input_initial(
                &format!("{}> ", "Description".cyan()),
                &format!("{}", &old_desc),
            ) {
                Ok(s) => new_desc = s,
                Err(TaskError::InputCancelled) => return Err(TaskError::InputCancelled),
                Err(e) => return Err(e),
            }
        } else {
            new_desc = args[1..].join(" ");
        }
        /* part 4: validate and update the task */
        if new_desc.is_empty() {
            return Err(TaskError::Empty("Description".to_string()));
        }
        let task_to_update = self.manager.at_mut(id).ok_or(TaskError::TaskNotFound(id))?;
        task_to_update.set_description(new_desc);
        println!(
            "Updated task description from '{}' to '{}'.",
            old_desc.yellow(),
            task_to_update.get_description().green()
        );
        Ok(())
    }

    fn handle_clear(&mut self) -> Result<(), TaskError> {
        let cleared_count = self.manager.clear_completed_tasks();
        println!(
            "Cleared {} completed tasks.",
            format!("{}", cleared_count).green().bold()
        );
        Ok(())
    }
}
