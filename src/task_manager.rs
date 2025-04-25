use crate::task::{Task, TaskStatus};
use std::fs::{self, File};
use std::io::{ErrorKind, Result, Write};

const SAVE_FILE_PATH: &str = "tasks.json";
const DEFAULT_STATUS: TaskStatus = TaskStatus::Todo;

pub fn create_task(token_array: Vec<String>) {
	let mut task_vector = load_tasks();

	let new_task_description: String = get_description(token_array);

	let new_task_id = generate_id();

	let new_task = Task {
		id: new_task_id,
		description: new_task_description,
		status: DEFAULT_STATUS,
	};

	clear_console();
	println!("Task {} created with success", &new_task.description);

	task_vector.push(new_task);
	let _ = save_tasks(task_vector);
}

fn get_description(token_array: Vec<String>) -> String {
	let mut description: String = String::new();

	for token in token_array {
		if token.as_str() == "create" {
			continue;
		}

		description.push_str(token.as_str());
		description.push(' ');
	}
	description = description.replace("\"", "");
	description.pop();
	description
}

fn generate_id() -> u32 {
	let task_vector: Vec<Task> = load_tasks();
	let mut higher_id: u32 = 0;

	for task in task_vector.iter() {
		if task.id > higher_id {
			higher_id = task.id;
		}
	}

	higher_id + 1
}

pub fn list_tasks() {
	let task_vector = load_tasks();

	if task_vector.is_empty() {
		clear_console();
		println!("No tasks found");
		return;
	}

	for task in task_vector.iter() {
		println!("{:3} - {:40} {}", task.id, task.description, task.status_str());
	}
}

pub fn clear_console() {
	print!("\x1B[2J\x1B[1;1H");
}

pub fn delete_task(token_array: Vec<String>) {
	let last_item = token_array.len() - 1;
	let id: u32 = token_array[last_item].parse().unwrap();
	let mut task_vector: Vec<Task> = load_tasks();

	clear_console();

	if task_vector.iter().any(|task| task.id == id) {
		task_vector.retain(|task| task.id != id);
		println!("Task {} deleted with success", id);
	} else {
		println!("Task not found");
		return;
	}

	save_tasks(task_vector).unwrap()
}

pub fn mark_task(token_array: Vec<String>, status: TaskStatus) {
	let last_item = token_array.len() - 1;
	let id: u32 = token_array[last_item].parse().unwrap();
	let mut task_vector: Vec<Task> = load_tasks();

	clear_console();

	for task in task_vector.iter_mut() {
		if task.id == id {
			task.status = status.clone();
			print!("Task {} marked as {}", id, task.status_str());
			let _ = save_tasks(task_vector);
			return;
		}
	}

	println!("Task not found");
}


fn save_tasks(task_vector: Vec<Task>) -> Result<()> {
	let json_vector = serde_json::to_string_pretty(&task_vector).unwrap();

	let mut save_file = File::create("tasks.json").unwrap();
	save_file.write_all(json_vector.as_bytes()).unwrap();

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

	serde_json::from_str(saved_tasks.as_str()).unwrap_or_default()
}
