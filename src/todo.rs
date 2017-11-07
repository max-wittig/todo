/*
 * Copyright (c) 2017 Max Wittig
 *
 * SPDX-License-Identifier:     MIT
 */

use std::fs::File;
use serde_yaml;
use std::io::Read;
use std::io::Write;
use std::process::exit;
use std::fmt;
use prettytable::Table;
use chrono::NaiveDateTime;
use chrono::Local;

pub fn load() -> TodoList {
    match File::open("todo.yaml") {
        Err(_) => {
            // File not found --> create new list
            new()
        },
        Ok(mut file) => {
            let mut file_content = String::new();
            file.read_to_string(&mut file_content).unwrap();
            serde_yaml::from_str(&file_content).unwrap()
        },
    }
}

pub fn new() -> TodoList {
    TodoList {
        task_list: Vec::new()
    }
}

#[derive(Serialize, Deserialize)]
pub struct TodoList {
    task_list : Vec<Task>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    task_name: String,
    description: String,
    done : bool,
    done_at : i64
}

impl Task {
    pub fn mark_done(&mut self) {
        self.done = true;
        self.done_at = Local::now().timestamp();
    }
}

impl TodoList {
    pub fn print(&self) {
        if self.task_list.is_empty() {
            println!("Task List is empty!")
        }
        else {
            let mut table = Table::new();
            table.add_row(row!["TASK NAME", "DESCRIPTION", "DONE", "DONE AT"]);
            for current_task in &self.task_list {
                let done_at_row = if current_task.done_at > 0 {
                    NaiveDateTime::from_timestamp(current_task.done_at, 0)
                        .format("%Y-%m-%d %H:%M:%S").to_string() } else { "-".to_string() };

                table.add_row(row![current_task.task_name, current_task.description,
                current_task.done, done_at_row]);
            }

            table.printstd();
        }
    }

    pub fn add_task(&mut self, task_name : &str, description : &str) {
        let task_name = task_name.to_string();
        let description = description.to_string();
        let task = Task {
            task_name,
            description,
            done : false,
            done_at : 0,
        };

        self.task_list.push(task);
    }

    pub fn mark_done(&mut self, task_name : &str) {
        for current_task in &mut self.task_list {
            if current_task.task_name == task_name {
                current_task.mark_done()
            }
        }
    }

    pub fn task_exists(&self, task_name : &str) -> bool {
        for current_task in &self.task_list {
            if current_task.task_name == task_name {
                return true
            }
        }
        false
    }

    fn to_yaml(&self) -> String {
        serde_yaml::to_string(&self).expect("Could not convert to yaml")
    }

    pub fn save(&self, filename : &str) {
        let mut file = match File::create(&filename) {
            Err(_) => {
                println!("Could not create file");
                exit(1)
            }
            Ok(file) => file,
        };
        file.write_all(self.to_yaml().as_bytes())
            .expect("File could not be written");
    }
}

impl fmt::Display for TodoList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{0}", self.to_yaml())
    }
}
