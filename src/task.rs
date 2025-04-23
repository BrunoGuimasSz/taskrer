use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Task {
        pub id: u32,
        pub description: String,
	pub status: TaskStatus,
}

#[derive(Serialize, Deserialize)]
pub enum TaskStatus {
	Todo,
	InProgress,
	Done,
}
