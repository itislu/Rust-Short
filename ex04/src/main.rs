/*
allowed symbols:
    std::{print, println}
    std::vec::Vec::{new, push, remove, clear, len, is_empty}
    std::string::String::as_str
    str::{to_string, parse, len, is_empty, trim, strip_prefix}
    ftkit::{read_line, read_number}
    std::result::Result
*/

enum Command {
    Todo(String), // Command: "TODO"
    Done(usize),  // Command: "DONE"
    Purge,        // Command: "PURGE"
    Quit,         // Command: "QUIT"
}

impl Command {
    fn prompt() -> Self {
        loop {
            let line = ftkit::read_line();
            if let Some(todo) = line.strip_prefix("TODO ") {
                let todo = todo.trim();
                if !todo.is_empty() {
                    return Command::Todo(todo.to_string());
                }
            } else if let Some(num) = line.strip_prefix("DONE ") {
                if let Ok(num) = num.trim().parse::<usize>() {
                    return Command::Done(num);
                }
            } else if !line.is_empty() {
                match (line.strip_prefix("PURGE"), line.strip_prefix("QUIT")) {
                    (Some(cmd), None) if cmd.trim().is_empty() => return Command::Purge,
                    (None, Some(cmd)) if cmd.trim().is_empty() => return Command::Quit,
                    _ => {}
                }
            } else {
                return Command::Quit
            }
        }
    }
}

struct TodoList {
    todos: Vec<String>,
    dones: Vec<String>,
}

impl TodoList {
    fn new() -> Self {
        TodoList {
            todos: Vec::new(),
            dones: Vec::new(),
        }
    }

    fn display(&self) {
        println!();
        if self.todos.len() > 0 || self.dones.len() > 0 {
            for (i, todo) in self.todos.iter().enumerate() {
                println!("    {} [ ] {}", i, todo);
            }
            for done in self.dones.iter() {
                println!("      [x] {}", done);
            }
            println!();
        }
    }

    fn add(&mut self, todo: String) {
        self.todos.push(todo);
    }

    fn done(&mut self, index: usize) {
        if index < self.todos.len() {
            self.dones.push(self.todos[index].to_string());
            self.todos.remove(index);
        }
    }

    fn purge(&mut self) {
        self.dones.clear();
    }
}

fn main() {
    let mut todo_list = TodoList::new();

    loop {
        todo_list.display();
        match Command::prompt() {
            Command::Todo(todo) => todo_list.add(todo),
            Command::Done(index) => todo_list.done(index),
            Command::Purge => todo_list.purge(),
            Command::Quit => break,
        }
    }
}
