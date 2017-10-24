#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate clap;
extern crate serde_yaml;

use std::str;
use clap::{Arg, App, ArgGroup};

fn get_args() -> (String, String) {
    let mut task_name :String = String::from("");
    let mut task_description : String = String::from("");

    let matches = App::new("Simple Todo")
        .version("1.0.0")
        .arg(Arg::with_name("task")
            .short("t")
            .long("task")
            .help("The name of the task")
            .takes_value(true))
        .arg(Arg::with_name("description")
            .short("d")
            .long("description")
            .help("Description")
            .default_value("")
            .takes_value(true))
        .get_matches();
    match matches.occurrences_of("task") {
        0 => println!("Please specify a task to add"),
        1 => task_name = value_t_or_exit!(matches.value_of("task"), String),
        _ => println!("You can only use this argument once"),
    };
    match matches.occurrences_of("description") {
        0 => println!("test"),
        1 => task_description = value_t_or_exit!(matches.value_of("description"), String),
        _ => println!("You can only use this argument once"),
    };

    (task_name, task_description)
}

fn main() {
    let mut todo_list = todo::load();
    let (task_name, task_description) = get_args();
    todo::TodoList::add_task(&mut todo_list, task_name, task_description);
    todo_list.save(String::from("todo.yaml"));
}

pub mod todo {
    use std::fs::File;
    use serde_yaml;
    use std::io::Read;
    use std::io::Error;
    use std::io::Write;

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
    }

    impl TodoList {
        pub fn add_task(&mut self, task_name : String, description : String) {
            let task = Task {
                task_name,
                description,
                done: false,
            };

            self.task_list.push(task);
        }

        pub fn save(&self, filename : String) {
            let mut file = match File::create(&filename) {
                Err(why) => panic!("Could not create file {0}", &filename),
                Ok(file) => file,
            };
            file.write_all(serde_yaml::to_string(self).unwrap().as_bytes())
                .expect("File could not be written");
        }
    }

    impl Task {
        fn save(&self) -> String {
            serde_yaml::to_string(self).unwrap()
        }
    }
}

