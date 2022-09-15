#[warn(clippy::needless_pass_by_value)]
use dirs::config_dir;
use std::path::Path;

use clap::{Parser, Subcommand};

mod commands;
mod utils;

#[derive(Parser)]
#[clap(version, about, author, arg_required_else_help(true))]
struct TodoCli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// List all todo
    #[clap(visible_alias = "ls")]
    List,
    /// List all todo and done items
    #[clap(visible_alias = "lsa")]
    ListAll,
    /// List all done items
    #[clap(visible_alias = "lsd")]
    ListDone,
    /// Add item to the todo list
    #[clap(visible_alias = "a")]
    Add {
        /// Text of todo item to add
        text: String,
        /// Priority of the item to add (from 1 to 5)
        #[clap(default_value_t = 3, value_parser = clap::value_parser!(i32).range(1..=5))]
        priority: i32,
    },
    /// Insert item to todo list at position number
    #[clap(visible_alias = "in")]
    Insert {
        /// Position of todo item to insert
        num: i32,
        /// Text of todo item to insert
        text: String,
        /// Priority of the item to add (from 1 to 5)
        #[clap(default_value_t = 3, value_parser = clap::value_parser!(i32).range(1..=5))]
        priority: i32,
    },
    /// Delete item from todo list by number
    #[clap(visible_aliases = &["rm", "del"])]
    Remove {
        /// Position of todo item to remove
        num: i32,
    },
    /// Delete all items in todo list
    #[clap(visible_alias = "cl")]
    Clear,
    /// Delete all items in done list
    #[clap(visible_alias = "cld")]
    ClearDone,
    /// Replace an item with updated text
    #[clap(visible_alias = "re")]
    Replace {
        /// Position of todo item to replace
        num: i32,
        /// Text of todo item to replace
        text: String,
    },
    /// Mark an item as done
    #[clap(visible_alias = "d")]
    Done {
        /// Position of todo item to mark as done
        num: i32,
    },
    /// Unmark an item as done
    #[clap(visible_aliases = &["undo", "ud"])]
    Undone {
        /// Position of todo item to undo
        num: i32,
    },
}

fn main() {
    let todo_cli = TodoCli::parse();
    let todo_path = config_dir()
        .unwrap()
        .join(Path::new("rusty-todo"))
        .join(Path::new("todo.txt"));

    let done_path = config_dir()
        .unwrap()
        .join(Path::new("rusty-todo"))
        .join(Path::new("done.txt"));

    utils::create_dirs_from_file(todo_path.as_path());
    utils::create_dirs_from_file(done_path.as_path());

    match &todo_cli.command {
        Some(Commands::List) => {
            commands::list_todo(todo_path.as_path());
        }
        Some(Commands::ListAll) => {
            commands::list_todo(todo_path.as_path());
            println!("");
            commands::list_done(done_path.as_path());
        }
        Some(Commands::ListDone) => {
            commands::list_done(done_path.as_path());
        }
        Some(Commands::Add { text, priority }) => {
            commands::add_item(todo_path.as_path(), text, priority);
            commands::list_todo(todo_path.as_path());
        }
        Some(Commands::Insert {
            num,
            text,
            priority,
        }) => {
            if commands::insert_item(todo_path.as_path(), text, num, priority) {
                commands::list_todo(todo_path.as_path());
            }
        }
        Some(Commands::Remove { num }) => {
            if commands::remove_item(todo_path.as_path(), num) {
                commands::list_todo(todo_path.as_path());
            }
        }
        Some(Commands::Clear) => {
            commands::clear_list(todo_path.as_path());
        }
        Some(Commands::ClearDone) => {
            commands::clear_list(done_path.as_path());
        }
        Some(Commands::Replace { num, text }) => {
            if commands::replace_item(todo_path.as_path(), num, text) {
                commands::list_todo(todo_path.as_path());
            }
        }
        Some(Commands::Done { num }) => {
            if commands::done_item(todo_path.as_path(), done_path.as_path(), num) {
                commands::list_done(done_path.as_path());
            }
        }
        Some(Commands::Undone { num }) => {
            if commands::undone_item(todo_path.as_path(), done_path.as_path(), num) {
                commands::list_todo(todo_path.as_path());
            }
        }
        None => {}
    }
}
