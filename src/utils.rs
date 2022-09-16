use std::fs::{create_dir_all, File};
use std::path::Path;

use colored::Colorize;

pub fn create_dirs_from_file(path: &Path) {
    if path.exists() {
        return;
    }
    let parent_dir = path.parent().unwrap();
    create_dir_all(parent_dir).unwrap();
    File::create(path).unwrap();
}

pub fn check_priority(priority: &Option<i32>, min: i32, max: i32, default: i32) -> Option<i32> {
    match priority {
        Some(p) => match p {
            1..=5 => Some(p.to_owned()),
            _ => {
                println!(
                    "Priority must be between {} and {}.",
                    min.to_string().yellow().underline(),
                    max.to_string().yellow().underline()
                );
                None
            }
        },
        None => Some(default),
    }
}
