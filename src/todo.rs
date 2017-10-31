use std::fs::File;
use serde_yaml;
use std::io::Read;
use std::io::Write;
use std::process::exit;
use time::now;

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

    pub fn save(&self, filename : String) {
        let mut file = match File::create(&filename) {
            Err(_) => {
                println!("Could not create file");
                exit(1);
            }
            Ok(file) => file,
        };
        file.write_all(serde_yaml::to_string(&self).unwrap().as_bytes())
            .expect("File could not be written");
    }
}
