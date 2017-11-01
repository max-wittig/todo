use std::fs::File;
use serde_yaml;
use std::io::Read;
use std::io::Write;
use std::process::exit;
use time::now;
use std::fmt;
use prettytable::Table;

pub fn load() -> TodoList {
    match File::open("todo.yaml") {
        Err(_) => {
            println!("File not found. Creating new todo list...");
            new()
        },
        Ok(mut file) => {
            let mut file_content = String::new();
            file.read_to_string(&mut file_content).unwrap();
            serde_yaml::from_str(&mut file_content).unwrap()
        },
    }
}

pub fn new() -> TodoList {
    TodoList {
        task_list: Vec::new()
    }
}

#[derive(Debug, Serialize, Deserialize)]
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
        self.done_at = now().to_timespec().sec;
    }
}

impl TodoList {
    pub fn print(&self) {
        let mut table = Table::new();
        table.add_row(row!["Task Name", "Description", "Done", "Done_at"]);
        for current_task in self.task_list.iter() {
            table.add_row(row![current_task.task_name, current_task.description,
                current_task.done, current_task.done_at]);
        }

        table.printstd();
    }

    pub fn add_task(&mut self, task_name : String, description : String, done : bool) {
        let task = Task {
            task_name,
            description,
            done,
            done_at : 0,
        };

        self.task_list.push(task);
    }

    pub fn mark_done(&mut self, task_name : String) {
        for current_task in self.task_list.iter_mut() {
            if current_task.task_name == task_name {
                current_task.mark_done()
            }
        }
    }

    pub fn task_exists(&self, task_name : &String) -> bool {
        for current_task in self.task_list.iter() {
            if current_task.task_name == task_name.as_str() {
                return true
            }
        }
        false
    }

    fn to_yaml(&self) -> String {
        serde_yaml::to_string(&self).unwrap()
    }

    pub fn save(&self, filename : String) {
        let mut file = match File::create(&filename) {
            Err(_) => {
                println!("Could not create file");
                exit(1);
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
