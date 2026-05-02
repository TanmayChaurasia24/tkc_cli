# tkc

A fast, lightweight, and simple terminal todo manager for developers. `tkc` lets you manage your todos without ever leaving your terminal. Your todos are automatically saved locally to `~/.tkc/todos.json`.

## Installation

You can install `tkc` directly from crates.io using Cargo:

```bash
cargo install tkc
```

*(Ensure your `~/.cargo/bin` is in your system's PATH).*

## Usage

`tkc` provides a set of intuitive commands for managing your tasks.

### Adding Tasks
Add a new task using the `add` command. You can also assign a tag for better organization.

```bash
# Add a simple task
tkc add "fix the login bug"

# Add a task with a tag
tkc add "write documentation" --tag work
```

### Listing Tasks
View your tasks using the `list` command. It supports filtering by tag or by status.

```bash
# List all tasks
tkc list

# List tasks with a specific tag
tkc list --tag work

# List only completed tasks
tkc list --done

# List only pending tasks
tkc list --pending
```

### Managing Tasks
Mark tasks as done, remove them individually, or clear all completed tasks.

```bash
# Mark a task as done (using its ID)
tkc done 1

# Delete a specific task
tkc remove 2

# Remove all completed tasks from the list
tkc clear
```

### JSON Output Integration
Every command supports the `--json` global flag, making `tkc` incredibly easy to integrate into scripts or other tools.

```bash
tkc --json list
tkc --json add "test script task"
```

## Storage

Your todos are stored in a simple JSON format at `~/.tkc/todos.json`. You can easily back up this file or sync it across devices.

## License

MIT License. See [LICENSE](LICENSE) for more details.