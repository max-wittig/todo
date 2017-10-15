#[macro_use]
extern crate clap;

use std::str;
use clap::{Arg, App, ArgGroup};

fn get_args() {
    let mut task_name :String = String::from("sfsdf");
    let mut task_description : String;

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

    println!("{}", task_name);
}

fn main() {
    let mut task_list = Vec::new();
    get_args();
    let task_name = String::from("Test");
    let task_description = String::from("Test description");
    todo::add_task(&mut task_list, task_name, task_description);
    println!("{:#?}", task_list)
}

pub mod todo {
    pub fn add_task(task_list: &mut Vec<Task>,
                task_name : String, description : String) {
        let task = Task {
            task_name,
            description,
            done: false,
        };

        task_list.push(task);
    }

    #[derive(Debug)]
    pub struct Task {
        task_name: String,
        description: String,
        done : bool,
    }

    impl Task {
        fn to_json(&self) -> String {
            String::from("whatever")
        }
    }
}

