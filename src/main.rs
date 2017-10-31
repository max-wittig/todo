#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;
extern crate clap;
extern crate time;

mod todo;

use std::str;
use clap::{Arg, App};
use std::process::exit;

fn get_args() -> (String, String, bool) {
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
            .multiple(true)
            .default_value("")
            .takes_value(true))
        .arg(Arg::with_name("mark_done")
            .short("md")
            .long("mark_done")
            .help("Mark task as done")
            .takes_value(false))
        .get_matches();

    let task_name = matches.value_of("task");
    let task_description = matches.value_of("description");
    let mark_done;
    match matches.occurrences_of("mark_done") {
        1 => mark_done = true,
        _ => mark_done = false
    }

    if task_name.is_some() {
        let description;
        if task_description.is_some() {
            description = task_description.unwrap().to_string();
        }
        else {
            description = String::new();
        }

        (task_name.unwrap().to_string(), description, mark_done)
    }
    else {
        println!("Please set a task name!");
        exit(1);
    }

}

fn main() {
    let mut todo_list = todo::load();
    let (task_name, task_description, mark_done) = get_args();
    if todo_list.task_exists(&task_name) {
        if mark_done {
            todo_list.mark_done(task_name);
        }
    }
    else {
        todo::TodoList::add_task(&mut todo_list, task_name, task_description, mark_done);
    }

    todo_list.save(String::from("todo.yaml"));
}