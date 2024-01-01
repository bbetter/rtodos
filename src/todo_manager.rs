use std::io::{Read, Write};
use crate::crud_by_pos::CrudByPos;
use crate::crud_by_pos_file::CrudByPosFile;

#[derive(Debug,PartialEq)]
pub enum TodoCommand {
    Add(String),
    Remove(usize),
    Update(usize, String),
    Show(usize),
    List
}

pub fn parse(args: Vec<&str>) -> TodoCommand {
    return match args[1].to_lowercase().as_str() {
        "add" => {
            TodoCommand::Add(args[2..].join(" "))
        },
        "list" => TodoCommand::List,
        "show" => {
            let position = args[2].parse::<usize>().expect("fail to parse out id");
            TodoCommand::Show(position)
        }
        "remove" => {
            let position = args[2].parse::<usize>().expect("fail to parse out id");
            TodoCommand::Remove(position)
        }
        "update" => {
            let position = args[2].parse::<usize>().expect("fail to parse out id");
            TodoCommand::Update(position, args[3..].join(" "))
        }
        _ => panic!("unknown command")
    }
}

pub fn process(command: &TodoCommand){

    match command {
        TodoCommand::Add(text) => {

            let todo = CrudByPosFile.add(text);
            println!("created_at === text");
            println!("-R {} === {}", todo.created_at, todo.text)
        }
        TodoCommand::Remove(position) => {
            let todo = CrudByPosFile.remove(*position);
            println!("pos === created_at");
            println!("-R {} === {}", position, todo.created_at)
        }
        TodoCommand::Update(position, text) => {
            let todo = CrudByPosFile.update(*position, text);
            println!("pos === new_text");
            println!("-U {} === {}", position, todo.text)
        }
        TodoCommand::List => {
            println!("pos === created_at === text");
            for (i, el) in CrudByPosFile.all().iter().enumerate() {
                println!("{} === {} === {}", i, el.created_at, el.text)
            }
        }
        TodoCommand::Show(position) => {
            println!("pos === created_at === text");
            let todo = CrudByPosFile.get(*position);
            println!("{} === {} === {}", position, todo.created_at, todo.text)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::todo_manager::{parse, TodoCommand};

    #[test]
    fn it_parse_add_correct() {
        let args = vec!["todos", "add", "some long text"];
        let command = parse(args);
        assert_eq!(command, TodoCommand::Add(String::from("some long text")))
    }

    #[test]
    fn it_parse_list() {
        let args = vec!["todos", "list"];
        let command = parse(args);
        assert_eq!(command, TodoCommand::List)
    }

    #[test]
    fn it_parse_show_correct() {
        let args = vec!["todos", "show", "0"];
        let command = parse(args);
        assert_eq!(command, TodoCommand::Show(0))
    }

    #[test]
    fn it_parse_update_correct() {
        let args = vec!["todos", "update", "0", "some long text"];
        let command = parse(args);
        assert_eq!(command, TodoCommand::Update(0, String::from("some long text")))
    }

    #[test]
    fn it_parse_remove_correct() {
        let args = vec!["todos", "remove", "0"];
        let command = parse(args);
        assert_eq!(command, TodoCommand::Remove(0))
    }
}