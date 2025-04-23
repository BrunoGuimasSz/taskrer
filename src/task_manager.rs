use crate::task::{Task, TaskStatus};
use std::io::{Result, Write};
use std::fs::{self, File};

pub fn create_task(token_array: Vec<String>) {
	const DEFAULT_STATUS: TaskStatus = TaskStatus::Todo;
	let new_task_description: String = token_array[1].clone();
	let new_task_id = generate_id();

	let new_task = Task {
		id: new_task_id,
		description: new_task_description,
		status: DEFAULT_STATUS,
	};

	save_tasks(new_task);
}

fn generate_id() -> u32 {
	1
}

pub fn list_tasks() {
	let task_vector = load_tasks();

	for task in task_vector.iter() {
		println!("{} {}", task.id, task.description);
	}

}

fn save_tasks(task_to_save: Task) -> Result<()> {
	let mut task_vector: Vec<Task> = Vec::new();

	task_vector.push(task_to_save);

	let json_vector = serde_json::to_string_pretty(&task_vector).unwrap();

	let mut save_file= File::create("tasks.json")?;
	save_file.write_all(json_vector.as_bytes())?;

	Ok(())
}

fn load_tasks() -> Vec<Task> {
	let save_file = fs::read_to_string("tasks.json").expect("Error while reading the file");

	let task_vector: Vec<Task> = serde_json::from_str(&save_file).expect("Error while deserializing");

	task_vector
}

