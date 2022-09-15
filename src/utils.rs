use std::fs::{create_dir_all, File};
use std::path::Path;

pub fn create_dirs_from_file(path: &Path) {
    if path.exists() {
        return;
    }
    let parent_dir = path.parent().unwrap();
    create_dir_all(parent_dir).unwrap();
    File::create(path).unwrap();
}
