use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "tkc",
    version = "0.1.0",
    about = "A fast terminal todo_manager for developers",
    long_about = "tkc lets you manage todos without leaving your terminal. \n todos are saved to ~/.tkc/todo.json"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(long, global = true)]
    pub json: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    Add {
        title: String,

        #[arg(short,long)]
        tag: Option<String>,
    },
    List {
        #[arg(short,long)]
        tag: Option<String>,

        #[arg(long)]
        done: bool,

        #[arg(long)]
        pending: bool,
    },
    Done {
        id: u32
    },
    Remove {
        id: u32,
    },
    Clear,
}