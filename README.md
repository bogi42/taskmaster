# taskmaster: A Simple Command-Line Task Manager

`taskmaster` is a minimalistic yet powerful command-line interface (CLI) tool designed to help you manage your daily tasks efficiently. Built with Rust, it offers both direct command execution and an interactive mode for a smoother workflow, with features like task prioritization and persistent storage.

---

## Features

* **Task Management:** Add, list, complete, delete, and modify tasks.
* **Task Prioritization:** Assign and adjust priorities (High, Medium, Low) to your tasks. Tasks display their priority visually (▲, ◆, ▼).
* **Interactive Mode:** A dedicated mode for continuous task management without repeatedly typing `taskmaster` before each command.
* **Data Persistence:** All your tasks are automatically saved to a JSON file (`.tasks.json`) in your home directory, ensuring your task list is preserved across sessions.
* **Clear Completed Tasks:** Easily clean up your list by removing all tasks marked as completed.
* **Colored Output:** Task statuses and priorities are visually highlighted using terminal colors for quick identification.

---

## Installation & Setup

Before you begin, ensure you have Rust installed. If not, you can install it via `rustup.rs`.

1.  **Clone the Repository:**
    ```bash
    git clone [https://github.com/bogi42/taskmaster.git](https://github.com/YOUR_USERNAME/taskmaster.git)
    cd taskmaster
    ```

2.  **Build the Project:**
    Since `taskmaster` is structured as a [Rust workspace](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html), building is straightforward:
    ```bash
    cargo build --release
    ```
    This command will compile both the `tasks` library crate (containing the core logic) and the `taskmaster_cli` binary crate. The executable will be located at `target/release/taskmaster_cli`.

3.  **Add to Your PATH (Optional, Recommended):**
    For easy access, move the `taskmaster_cli` executable to a directory in your system's PATH (e.g., `/usr/local/bin` on Linux/macOS or a custom `bin` directory on Windows). You might want to rename it to just `taskmaster` for convenience.
    ```bash
    # On Linux/macOS (example)
    mv target/release/taskmaster_cli /usr/local/bin/taskmaster
    ```

---

## Usage

`taskmaster` can be used via direct commands or its interactive mode.

### Direct Commands

Run `taskmaster --help` for a full list of commands and their arguments.

```bash
# Add a new task
taskmaster add "Buy groceries"

# List all tasks
taskmaster list

# Complete a task by its 1-based index
taskmaster complete 1

# Change a task's description
taskmaster change 1 "Buy organic groceries"

# Increase a task's priority (Low -> Medium -> High)
taskmaster up 1

# Decrease a task's priority (High -> Medium -> Low)
taskmaster down 1

# Delete a task
taskmaster delete 2

# Clear all completed tasks
taskmaster clear
```
### Interactive Mode

For a continuous task management session, enter interactive mode:
```Bash
taskmaster interactive
```
In interactive mode, simply type commands without the taskmaster prefix. Type h or help to see a list of available commands within the interactive session.
```Bash
Starting interactive mode. Type 'h' or 'help' for commands.

Interactive Mode Commands:
  l / list                  - List all tasks
  a / add <desc>            - Add a new task
  c / complete <idx>        - Mark a task as completed
  up / + <idx>              - Increase a task's priority
  down / - <idx>            - Decrease a task's priority
  d / delete <idx>          - Delete a task
  ch / change <idx> <desc>  - Change a task's description
  clr / clear               - Clear all completed tasks
  h / help / ?              - Show this help message
  q / quit / x / exit       - Exit interactive mode

» add Implement interactive mode help
Added Task #1: Implement interactive mode help
» l
Your tasks:
1: ◆ [·] Implement interactive mode help
» q
Exiting interactive mode.
Tasks saved successfully.
```

## Data Storage

Your tasks are automatically saved to a JSON file named .tasks.json in your user's home directory (e.g., /home/youruser/.tasks.json on Linux, C:\Users\youruser\.tasks.json on Windows).
Contributing

## Contributions 
are welcome! If you have suggestions for improvements or encounter any bugs, please open an issue or submit a pull request.
