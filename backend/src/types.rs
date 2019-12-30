use std::error::Error;

#[derive(Debug, Clone)]
pub struct TodoTask {
	pub id: Option<i32>, //Option because this is set by the database
	pub task: String,
	pub finished: bool,
}

impl TodoTask {
	pub fn pretty_print_to_stdout(self) {
		let task_id = match self.id.is_some() {
			true => self.id.unwrap().to_string(),
			false => String::from(""),
		};
		println!(
			" - ID: {}, Task: {}, Finished: {:?}",
			task_id, self.task, self.finished
		)
	}
}

pub trait TodoTaskStore {
	fn get(&self, id: i32) -> Result<TodoTask, Box<dyn Error>>;
	fn list(&self) -> Result<Vec<TodoTask>, Box<dyn Error>>;
	fn add(&self, task: TodoTask) -> Result<TodoTask, Box<dyn Error>>;
	fn update(&self, task: TodoTask) -> Result<(), Box<dyn Error>>;
	//TODO
	//fn remove(&self, task: ToDoTask) ->  Result<(), Box<dyn Error>>;
}
