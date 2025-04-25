
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Task {
        pub id: u32,
        pub description: String,
	pub status: TaskStatus,
}

#[derive(Serialize, Deserialize)]
#[derive(Clone)]
pub enum TaskStatus {
	Todo,
	InProgress,
	Done,
}

impl Task {
	pub fn status_str(&self) -> &str {
		match self.status {
			TaskStatus::Todo => "todo",
			TaskStatus::InProgress => "in progress",
			TaskStatus::Done => "done",
		}
	}
}
