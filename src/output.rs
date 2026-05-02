use colored::Colorize;
use crate::todo::Todo;

// when json mode is off
pub fn print_todo(todo: &Todo) {
    let checkbox = if todo.done {
        "✓".green().bold().to_string()
    } else {
        "○".yellow().to_string()
    };

    let title = if todo.done {
        todo.title.dimmed().strikethrough().to_string()
    } else {
        todo.title.white().bold().to_string()
    };

    let tag = match &todo.tag {
        Some(t) => format!("[{}]", t).cyan().to_string(),
        None => String::new(),
    };

    println!(
        "  {}  {}  {:<40} {}  {}",
        todo.id.to_string().dimmed(),
        checkbox,
        title,
        tag,
        todo.created_at.dimmed()
    );
}

// when json mode is on
pub fn print_list(todos: &[Todo], json_mode: bool) {
    if json_mode {
        println!("{}", serde_json::to_string_pretty(todos).unwrap());
        return;
    }

    if todos.is_empty() {
        println!("{}", " No todos found.".dimmed());
        return;
    }

    println!(
        "\n  {}  {}  {:<40} {}",
        "id".dimmed(),
        " ".dimmed(),
        "title".dimmed(),
        "created".dimmed()
    );
    println!("  {}", "─".repeat(70).dimmed());

    for todo in todos {
        print_todo(todo);
    }

    let total = todos.len();
    let done = todos.iter().filter(|t| t.done).count();
    let pending = total - done;

    println!("  {}", "─".repeat(70).dimmed());
    println!(
        "  {} total  •  {} pending  •  {} done\n",
        total.to_string().white(),
        pending.to_string().yellow(),
        done.to_string().green()
    );
}

pub fn print_added(todo: &Todo, json_mode: bool) {
    if json_mode {
        println!("{}", serde_json::to_string_pretty(todo).unwrap());
        return;
    } 

    println!(
        "  {} Added: \"{}\" (id: {})",
        "✓".green().bold(),
        todo.title.white().bold(),
        todo.id.to_string().dimmed()
    );
}

pub fn print_done(todo: &Todo, json_mode: bool) {
    if json_mode {
        println!("{}", serde_json::to_string_pretty(todo).unwrap());
        return;
    }
 
    println!(
        "  {} Marked done: \"{}\"",
        "✓".green().bold(),
        todo.title.dimmed().strikethrough()
    );
}

pub fn print_removed(todo: &Todo, json_mode: bool) {
    if json_mode {
        println!("{}", serde_json::to_string_pretty(todo).unwrap());
        return;
    }
 
    println!(
        "  {} Removed: \"{}\"",
        "✗".red().bold(),
        todo.title.dimmed()
    );
}

pub fn print_cleared(count: usize, json_mode: bool) {
    if json_mode {
        println!("{{\"cleared\": {}}}", count);
        return;
    }
 
    if count == 0 {
        println!("  {}", "No completed todos to clear.".dimmed());
    } else {
        println!(
            "  {} Cleared {} completed todo{}.",
            "✓".green().bold(),
            count.to_string().yellow(),
            if count == 1 { "" } else { "s" }
        );
    }
}
 
pub fn print_error(message: &str) {
    eprintln!("  {} {}", "error:".red().bold(), message);
}