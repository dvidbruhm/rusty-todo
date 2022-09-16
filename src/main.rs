#[warn(clippy::needless_pass_by_value)]
use dirs::config_dir;
use std::path::Path;
use utils::check_priority;

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
        #[clap(value_parser)]
        priority: Option<i32>,
    },
    /// Insert item to todo list at position number
    #[clap(visible_alias = "in")]
    Insert {
        /// Position of todo item to insert
        num: i32,
        /// Text of todo item to insert
        text: String,
        /// Priority of the item to insert (from 1 to 5)
        #[clap(value_parser)]
        priority: Option<i32>,
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
    /// Changes the priority of an item
    #[clap(visible_alias = "p")]
    Priority {
        /// Position of item to change priority
        num: i32,
        /// New priority (from 1 to 5)
        priority: Option<i32>,
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
            let p = &check_priority(priority, 1, 5, 3);
            if p.is_none() {
                return;
            }
            commands::add_item(todo_path.as_path(), text, p);
            commands::list_todo(todo_path.as_path());
        }
        Some(Commands::Insert {
            num,
            text,
            priority,
        }) => {
            let p = &check_priority(priority, 1, 5, 3);
            if p.is_none() {
                return;
            }
            if commands::insert_item(todo_path.as_path(), text, num, p) {
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
        Some(Commands::Priority { num, priority }) => {
            let p = &check_priority(priority, 1, 5, 3);
            if p.is_none() {
                return;
            }
            commands::change_priority(todo_path.as_path(), num, p);
            commands::list_todo(todo_path.as_path());
        }
        None => {}
    }
}
