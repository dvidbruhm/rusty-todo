use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::{BufRead, BufReader};
use std::path::Path;

use chrono::Datelike;
use colored::{ColoredString, Colorize};

struct Line {
    num: i32,
    text: String,
}

pub fn list_todo(path: &Path) {
    let lines = read_file_as_lines(path);

    let num_lines = lines.len();
    if num_lines == 0 {
        println!("{}", "There are no items in your todo list. :)".green());
        return;
    }

    let mut lines_struct = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        lines_struct.push(Line {
            num: (i + 1) as i32,
            text: line.to_owned(),
        })
    }

    lines_struct.sort_by(|a, b| line_to_priority(&a.text).cmp(&line_to_priority(&b.text)));

    println!("{}", "Todo list : ".blue().bold());
    println!("{}", "-----------".blue().dimmed());

    for line in lines_struct.iter() {
        let p = &line.text[..4];
        let t = &line.text[4..];
        println!(
            "[{}] -> {} {}",
            line.num.to_string().magenta().bold(),
            priority_to_colored_str(p).bold(),
            t.blue()
        );
    }
}

pub fn list_done(path: &Path) {
    let lines = read_file_as_lines(path);

    let num_lines = lines.len();
    if num_lines == 0 {
        println!("{}", "There are no items in your done list. :(".green());
        return;
    }

    println!("{}", "Done list : ".green().bold());
    println!("{}", "-----------".green().dimmed());
    for (i, line) in lines.iter().enumerate() {
        println!(
            "[{}] -> {}",
            (i + 1).to_string().magenta().bold(),
            line.green()
        );
    }
}

pub fn add_item(path: &Path, text: &str, priority: &i32) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(path)
        .expect("Unable to open file");

    if priority == &-1 {
        writeln!(file, "{}", text).expect("Unable to write to file");
    } else {
        let priority_str = priority_to_str(priority);
        writeln!(file, "{} {}", priority_str, text).expect("Unable to write to file");
    }
}

pub fn insert_item(path: &Path, text: &str, item_num: &i32, priority: &i32) -> bool {
    let mut lines = read_file_as_lines(path);

    let num_lines = lines.len() as i32;

    let item_ok = check_item_num(&num_lines, item_num);
    if !item_ok {
        return false;
    }

    lines.insert(
        *item_num as usize - 1,
        format!("{} {}", priority_to_str(priority), text).to_string(),
    );
    write_lines_to_file(path, lines);
    true
}

pub fn remove_item(path: &Path, item_num: &i32) -> bool {
    let mut lines = read_file_as_lines(path);

    let num_lines = lines.len() as i32;

    let item_ok = check_item_num(&num_lines, item_num);
    if !item_ok {
        return false;
    }

    lines.remove((*item_num as usize) - 1);
    write_lines_to_file(path, lines);
    true
}

pub fn clear_list(path: &Path) {
    write_lines_to_file(path, Vec::new());
}

pub fn replace_item(path: &Path, item_num: &i32, text: &str) -> bool {
    let removed = remove_item(path, item_num);
    if !removed {
        return false;
    }
    insert_item(path, text, item_num, &3);
    true
}

pub fn done_item(todo_path: &Path, done_path: &Path, item_num: &i32) -> bool {
    let lines = read_file_as_lines(todo_path);
    let num_lines = lines.len() as i32;

    let item_ok = check_item_num(&num_lines, item_num);
    if !item_ok {
        return false;
    }

    let done_line = &lines[*item_num as usize - 1];
    let current_time = chrono::offset::Local::now().date();
    let mut time = format!(
        "[{:0>4}-{:0>2}-{:0>2}] - ",
        current_time.year(),
        current_time.month(),
        current_time.day()
    );
    time.push_str(done_line);
    add_item(done_path, &time, &-1);
    remove_item(todo_path, item_num);
    true
}

pub fn undone_item(todo_path: &Path, done_path: &Path, item_num: &i32) -> bool {
    let lines = read_file_as_lines(done_path);
    let num_lines = lines.len() as i32;

    let item_ok = check_item_num(&num_lines, item_num);
    if !item_ok {
        return false;
    }

    let undone_line = &lines[*item_num as usize - 1][15..];
    add_item(todo_path, undone_line, &-1);
    remove_item(done_path, item_num);
    true
}

fn check_item_num(num_lines: &i32, item_num: &i32) -> bool {
    if !(1..num_lines + 1).contains(item_num) {
        println!(
            "Your list contains {} items, the item number has to be between {} and {}.",
            num_lines.to_string().yellow().underline(),
            "1".yellow().underline(),
            num_lines.to_string().yellow().underline()
        );
        return false;
    }
    true
}

fn read_file_as_lines(path: &Path) -> Vec<String> {
    let file = File::open(path).expect("Unable to open file");
    let buffered_file = BufReader::new(file);
    buffered_file.lines().map(|l| l.unwrap()).collect()
}

fn write_lines_to_file(path: &Path, lines: Vec<String>) {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(path)
        .expect("Unable to open file");

    for line in lines.iter() {
        writeln!(file, "{}", line).expect("Unable to write to file");
    }
}

fn priority_to_str(priority: &i32) -> &str {
    match priority {
        1 => return "(!!)",
        2 => return " (!)",
        3 => return "    ",
        4 => return " (-)",
        5 => return "(--)",
        _ => panic!(
            "Priority should be between {} and {}",
            "1".yellow(),
            "5".yellow()
        ),
    }
}

fn priority_to_colored_str(priority: &str) -> ColoredString {
    match priority {
        "(!!)" => return "(!!)".yellow(),
        " (!)" => return " (!)".red(),
        "    " => return "    ".white(),
        " (-)" => return " (-)".cyan(),
        "(--)" => return "(--)".blue(),
        _ => panic!(
            "Priority should be between {} and {}",
            "1".yellow(),
            "5".yellow()
        ),
    }
}

fn line_to_priority(line: &str) -> i32 {
    let s = &line[..4];
    match s {
        "(!!)" => return 1,
        " (!)" => return 2,
        "    " => return 3,
        " (-)" => return 4,
        "(--)" => return 5,
        _ => panic!("Bad priority string."),
    }
}
