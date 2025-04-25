mod task;
mod task_manager;

use std::io::{stdin, stdout, Result, Write};

fn main() -> Result<()> {
	loop {
		print!("Taskrer -> ");
		let _ = stdout().flush();

		let mut input: String = String::new();

		stdin()
			.read_line(&mut input)
			.expect("Error while reading line");

		let cleaned_input = input.trim().to_string();

		if cleaned_input == "exit" || cleaned_input == "quit" {
			break;
		}

		command_handler(&input);
	}

	Ok(())
}

fn command_handler(input: &str) {
	let token_array: Vec<String> = input
		.split_whitespace()
		.map(|token| token.to_string())
		.collect();

	match token_array[0].as_str() {
		"create" => task_manager::create_task(token_array),
		"list" => task_manager::list_tasks(),
		"delete" => task_manager::delete_task(token_array),
		"mark-done" => task_manager::mark_task(token_array, task::TaskStatus::Done),
		"mark-in-progress" => task_manager::mark_task(token_array, task::TaskStatus::InProgress),
		"mark-todo" => task_manager::mark_task(token_array, task::TaskStatus::Todo),
		_ => println!("Command not found: {}", token_array[0]),
	}
}
