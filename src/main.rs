use std::str;

fn main() {
    let mut task_list = Vec::new();
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

        }
    }
}

