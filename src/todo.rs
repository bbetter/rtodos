use chrono::Local;

#[derive(Debug)]
pub struct Todo {
    pub text: String,
    pub created_at: i64
}

impl Todo {

    pub fn new(timestamp: i64, text: &str) -> Self {
        Self {
            text: text.to_string(),
            created_at: timestamp
        }
    }
    pub fn from_text(text: &str) -> Self{
        Self::new(Local::now().timestamp(),  text)
    }

    pub fn from_line(line: &str) -> Self {
        let split_values: Vec<&str> = line.split(' ').collect();
        let timestamp = split_values[0].parse::<i64>().expect("failed to parse timestamp");
        let text = split_values[1..].join(" ");
        Self::new(timestamp, &text)
    }
}

#[cfg(test)]
mod todo_tests {
    use crate::todo::Todo;
    use super::*;

    #[test]
    fn it_todo_created_from_line() {
        let todo = Todo::from_line("1151351 some very interesting test");
        assert_eq!(todo.created_at, 1151351);
        assert_eq!(todo.text, "some very interesting test");
    }
}