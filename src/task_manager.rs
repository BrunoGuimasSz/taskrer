use crate::task::{Task, TaskStatus};
use std::fs::{self, File};
use std::io::{ErrorKind, Result, Write};

const SAVE_FILE_PATH: &str = "tasks.json";
const DEFAULT_STATUS: TaskStatus = TaskStatus::Todo;

pub fn create_task(token_array: Vec<String>) {
    let mut new_task_description: String = String::new();
    for token in token_array {
        if token.as_str() == "create" {
            continue;
        }

        new_task_description.push_str(token.as_str());
        new_task_description.push(' ');
    }
    new_task_description = new_task_description.replace("\"", "");
    new_task_description.pop();

    let new_task_id = generate_id();

    let new_task = Task {
        id: new_task_id,
        description: new_task_description,
        status: DEFAULT_STATUS,
    };

    save_tasks(new_task);
}

fn generate_id() -> u32 {
    let task_vector: Vec<Task> = load_tasks();
    let mut max_id: u32 = 0;

    for task in task_vector.iter() {
        if task.id > max_id {
            max_id = task.id;
        }
    }

    max_id + 1
}

pub fn list_tasks() {
    let task_vector = load_tasks();

    for task in task_vector.iter() {
        println!("{} {}", task.id, task.description);
    }
}

fn save_tasks(task_to_save: Task) -> Result<()> {
    let mut task_vector: Vec<Task> = load_tasks();

    task_vector.push(task_to_save);

    let json_vector = serde_json::to_string_pretty(&task_vector).unwrap();

    let mut save_file = File::create("tasks.json")?;
    save_file.write_all(json_vector.as_bytes())?;

    Ok(())
}

fn load_tasks() -> Vec<Task> {
    let save_file = File::open(SAVE_FILE_PATH);

    let _save_file = match save_file {
        Ok(file_opened) => file_opened,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(SAVE_FILE_PATH) {
                Ok(file_created) => file_created,
                Err(error) => panic!("Error while creating save file: {}", error),
            },
            _other => panic!("Error while opening save file: {}", error),
        },
    };

    let saved_tasks = match fs::read_to_string(SAVE_FILE_PATH) {
        Ok(file_opened) => file_opened,
        Err(error) => panic!("Error while reading save file {}", error),
    };

    match serde_json::from_str(saved_tasks.as_str()) {
        Ok(file_read) => file_read,
        Err(_error) => Vec::new(),
    }
}
