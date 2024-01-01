use crate::todo::Todo;
use crate::crud_by_pos::CrudByPos;

use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use chrono::Local;

const RELATIVE_DATA_FILE: &str = "todos.dat";

pub struct CrudByPosFile;

impl CrudByPos<Todo> for CrudByPosFile {
    fn add(&self, text: &str) -> Todo {
        let mut f = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(RELATIVE_DATA_FILE)
            .expect("Failed to open file");


        let todo = Todo::from_text(text);
        writeln!(&mut f, "{} {}", todo.created_at, todo.text)
            .expect("failed to write to file");
        todo
    }

    fn update(&self, position: usize, text: &str) -> Todo {
        let mut contents = String::new();
        {
            let mut file = OpenOptions::new()
                .read(true)
                .open(RELATIVE_DATA_FILE)
                .expect("failed to open file for rewrite");
            file.read_to_string(&mut contents)
                .expect("failed to read contents");
        }
        let new_lines: Vec<_> = contents.lines().enumerate()
            .map(|(index, line)| {
                if index != (position - 1) {
                    Some(line.to_string())
                } else {
                    let split_values: Vec<&str> = line.split(' ').collect();
                    let timestamp = split_values[0].parse::<i64>().expect("failed to parse timestamp");

                    let new_line = format!("{} {}", timestamp, text);
                    Some(new_line)
                }
            })
            .filter_map(|x| x)
            .collect();


        let mut output_file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(RELATIVE_DATA_FILE)
            .expect("failed to open file for rewrite");

        for line in new_lines {
            writeln!(output_file, "{}", line)
                .expect("")
        }
        let timestmap = Local::now().timestamp();

        return Todo::from_line(
            format!("{} {}", timestmap, text)
                .as_str()
        )
    }

    fn remove(&self, position: usize) -> Todo {
        let mut contents = String::new();
        {
            let mut file = OpenOptions::new()
                .read(true)
                .open(RELATIVE_DATA_FILE)
                .expect("failed to open file for rewrite");
            file.read_to_string(&mut contents)
                .expect("failed to read contents");

            //scope end to close file
        }
        let mut removed_line: &str = "";
        let new_lines: Vec<_> = contents.lines().enumerate()
            .filter_map(|(index, line)| {
                if index != (position - 1) { // Adjust for 0-based indexing
                    removed_line = line;
                    Some(line)
                } else {
                    None
                }
            })
            .collect();


        let mut output_file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(RELATIVE_DATA_FILE)
            .expect("failed to open file for rewrite");

        for line in new_lines {
            writeln!(output_file, "{}", line)
                .expect("")
        }

        return Todo::from_line(removed_line)
    }

    fn all(&self) -> Vec<Todo> {
        let mut contents = String::new();
        File::open(RELATIVE_DATA_FILE)
            .expect("failed to open file")
            .read_to_string(&mut contents)
            .expect("failed to read contents");

        let todos = contents.lines().map(|line| {
            Todo::from_line(line)
        }).collect();

        return todos;
    }

    fn get(&self, position: usize) -> Todo {
        let mut contents = String::new();
        File::open(RELATIVE_DATA_FILE)
            .expect("failed to open file")
            .read_to_string(&mut contents)
            .expect("failed to read contents");

        let lines: Vec<_> = contents.lines().collect();
        return Todo::from_line(lines.get(position - 1).expect(""))
    }
}

#[cfg(test)]
mod tests {
    use std::fs::{File, metadata, remove_file};
    use crate::crud_by_pos::CrudByPos;
    use crate::crud_by_pos_file::{CrudByPosFile, RELATIVE_DATA_FILE};

    use std::sync::Once;

    static INIT: Once = Once::new();

    pub fn initialize() {
        INIT.call_once(|| {
            // initialization code here
            if let result = remove_file(RELATIVE_DATA_FILE) {
                println!("file deleted")
            }
        });
    }

    #[test]
    fn it_works_when_add_update_remove_show() {
        initialize();
        CrudByPosFile.add("test");
        CrudByPosFile.add("test22");
        if let metadata = metadata(RELATIVE_DATA_FILE) {
            let mut todos = CrudByPosFile.all();
            assert_eq!(todos.len(), 2);
            assert_eq!(todos[0].text, "test");
            assert_eq!(todos[1].text, "test22");

            CrudByPosFile.update(1, "test11");
            todos = CrudByPosFile.all();
            assert_eq!(todos.len(), 2);
            assert_eq!(todos[0].text, "test11");

            CrudByPosFile.remove(1);

            todos = CrudByPosFile.all();
            assert_eq!(todos.len(), 1);
            assert_eq!(todos[0].text, "test22");

            CrudByPosFile.add("test33");
            let todo = CrudByPosFile.get(1);
            assert_eq!(todo.text, "test22");
        }
    }
}