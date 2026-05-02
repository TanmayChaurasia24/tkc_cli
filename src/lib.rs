pub mod cli;
pub mod error;
pub mod output;
pub mod store;
pub mod todo;

use anyhow::Context;
use clap::Parser;

use cli::{Cli, Commands};
use error::TkcError;

pub fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let mut todos = store::load().context("failed to load todos from disk")?;

    match cli.command {
        Commands::Add { title, tag } => {
            let title = title.trim().to_string();
            if title.is_empty() {
                return Err(TkcError::Other("todo title cannot be empty".to_string()).into());
            }

            let id = store::next_id(&todos);

            let new_todo = todo::Todo::new(id, title, tag);

            output::print_added(&new_todo, cli.json);

            todos.push(new_todo);

            store::save(&todos).context("failed to save todos")?;
        }

        Commands::List { tag, done, pending } => {
            let filtered: Vec<&todo::Todo> = todos
                .iter()
                .filter(|t| t.matches_tag(&tag))
                .filter(|t| {
                    if done {
                        t.done
                    } else if pending {
                        !t.done
                    } else {
                        true
                    }
                })
                .collect();

            let owned: Vec<todo::Todo> = filtered.into_iter().cloned().collect();
            output::print_list(&owned, cli.json);
        }

        Commands::Done { id } => {
            let todo = todos
                .iter_mut()
                .find(|t| t.id == id)
                .ok_or(TkcError::TodoNotFound(id))?;

            todo.mark_done();

            let cloned = todo.clone();
            output::print_done(&cloned, cli.json);

            store::save(&todos).context("failed to save todos")?;
        }

        Commands::Remove { id } => {
            let pos = todos
                .iter()
                .position(|t| t.id == id)
                .ok_or(TkcError::TodoNotFound(id))?;

            let removed = todos.remove(pos);
            output::print_removed(&removed, cli.json);

            store::save(&todos).context("failed to save todos")?;
        }

        Commands::Clear => {
            let before = todos.len();

            todos.retain(|t| !t.done);

            let removed_count = before - todos.len();
            output::print_cleared(removed_count, cli.json);

            store::save(&todos).context("failed to save todos")?;
        }
    }

    Ok(())
}