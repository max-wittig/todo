/*
 * Copyright (c) 2017 Max Wittig
 *
 * SPDX-License-Identifier:     MIT
 */

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate prettytable;
extern crate serde_yaml;
extern crate clap;
extern crate time;

mod todo;

use std::str;
use clap::{Arg, App};
use std::process::exit;


fn main() {
    let mut todo_list = todo::load();
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
        .arg(Arg::with_name("print")
            .short("p")
            .long("print")
            .help("Just print tasks")
            .takes_value(false))
        .get_matches();

    let task_name = matches.value_of("task");
    let task_description = matches.value_of("description");
    let mark_done;
    if matches.occurrences_of("mark_done") > 0 {
        mark_done = true;
    }
    else {
        mark_done = false;
    }
    if matches.occurrences_of("print") > 0 {
        todo_list.print();
        exit(0);
    }

    if let Some(task_name) = task_name {
        let task_name = task_name.to_string();
        let description;
        if let Some(task_description) = task_description {
            description = task_description.to_string();
        } else {
            description = String::new();
        }

        if todo_list.task_exists(&task_name) {
            if mark_done {
                todo_list.mark_done(task_name);
            }
        } else {
            todo::TodoList::add_task(&mut todo_list, task_name,
                                     description, mark_done);
            todo_list.print();
        }
        todo_list.save(String::from("todo.yaml"));
    } else {
        println!("Please set a task name!");
        exit(1);
    }
}