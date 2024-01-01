use std::env;
use crate::todo_manager::{parse, process};

mod todo;
mod todo_manager;
mod crud_by_pos_file;
mod crud_by_pos;

fn main() {
    let string_args: Vec<String> = env::args()
        .collect();

    let str_args = string_args.iter()
        .map(|x| x.as_str())
        .collect();

    let command = parse(str_args);

    process(&command);
}