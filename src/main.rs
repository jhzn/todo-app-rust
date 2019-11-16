use std::error::Error;
use std::io;
mod sqlite;

fn main() {
	let todo_store = sqlite::setup("todo.db".to_string(), true).unwrap();

	loop {
		let possible_cmdline_args = "list, get [id], add, update";

		println!("\nEnter command({}): ", possible_cmdline_args);
		println!("----------------------------------");
		let mut x = String::new();
		io::stdin().read_line(&mut x).expect("Error reading input");

		let args: Vec<&str> = x.split_whitespace().collect();
		if args.len() == 0 {
			println!(
				"Missing program argument\nValid args are ({})",
				possible_cmdline_args
			);
			return ();
		}

		let get_enum_from_program_arg = |program_arg: &str| -> Result<ToDoActions, Box<dyn Error>> {
			match program_arg.trim().to_lowercase().as_ref() {
				"get" => Ok(ToDoActions::Get),
				"list" => Ok(ToDoActions::List),
				"add" => Ok(ToDoActions::Add),
				"update" => Ok(ToDoActions::Update),
				_ => Err("Invalid program argument")?,
			}
		};

		match get_enum_from_program_arg(&args[0]).unwrap() {
			ToDoActions::Get => {
				println!("\nSelected get task mode!\n");
				if args.len() != 2 {
					println!("Missing task id");
					continue;
				}
				let task_id: i32 = args[1].parse().expect("Invalid id given");
				let stored_task = todo_store.get(task_id).expect("Unable to get task");
				stored_task.pretty_print_to_console();
			}
			ToDoActions::List => {
				println!("\nSelected list tasks mode!\n");
				println!("Currently stored tasks are:");
				let stored_todo_tasks =
					todo_store.get_all().expect("Unable to retrieve todo tasks");
				if stored_todo_tasks.is_empty() {
					println!("No tasks are stored!");
				} else {
					for task in stored_todo_tasks {
						task.pretty_print_to_console();
					}
				}
			}
			ToDoActions::Add => {
				println!("\nSelected task creation mode!\nEnter task name:");
				let mut x = String::new();
				io::stdin().read_line(&mut x).expect("Error reading input");
				let task_name: &str = x.trim();

				let _new_task = ToDoTask {
					id: None,
					task: String::from(task_name),
					finished: false,
				};
				todo_store.add(_new_task).expect("Unable to add task");
			}
			ToDoActions::Update => {
				let currently_stored_tasks = &todo_store
					.get_all()
					.expect("Unable to retrieve tasks when trying to update");
				println!("\nCurrently stored Tasks:");
				for task in currently_stored_tasks {
					task.copy().pretty_print_to_console();
				}

				println!(
					"\nSelected task updating mode!\nEnter id of task which you want to modifiy"
				);
				let mut x = String::new();
				io::stdin().read_line(&mut x).expect("Error reading input");
				let task_id: i32 = x.trim().parse().expect("Unable to parse task id");

				println!("Enter new task name(leaving empty will change nothing):");
				let mut x = String::new();
				io::stdin().read_line(&mut x).expect("Error reading input");
				let potentially_new_task_name: &str = x.trim();

				let mut task_name = potentially_new_task_name;
				if potentially_new_task_name.is_empty() {
					//if the user left the task name empty, we dont change it, get what the name was previously
					let mut found_task = false;
					for task in currently_stored_tasks {
						if task.id.unwrap() == task_id {
							found_task = true;
							task_name = &task.task;
						}
					}
					if !found_task {
						panic!("Could not find a task with that ID")
					}
				}

				println!("Is task finished? Y/N:");
				let mut x = String::new();
				io::stdin().read_line(&mut x).expect("Error reading input");

				let is_task_finished: bool = match x.trim().to_lowercase().as_ref() {
					"y" => true,
					"n" => false,
					_ => {
						panic!("Unable to convert input to boolean");
					}
				};

				let task = ToDoTask {
					id: Some(task_id),
					task: task_name.to_string(),
					finished: is_task_finished,
				};
				todo_store.update(task).expect("Unable to update task");
			}
		};
	}
}

#[derive(Debug)]
enum ToDoActions {
	Add,
	Update,
	Get,
	List,
}
#[derive(Debug, Clone)]
pub struct ToDoTask {
	id: Option<i32>, //Option because this is set by the database
	task: String,
	finished: bool,
}
impl ToDoTask {
	fn pretty_print_to_console(self) {
		let task_id = match self.id.is_some() {
			true => self.id.unwrap().to_string(),
			false => String::from(""),
		};
		println!(
			" - ID: {}, Task: {}, Finished: {:?}",
			task_id, self.task, self.finished
		)
	}
	//cant derive copy trait, implementing it here instead
	fn copy(&self) -> Self {
		return ToDoTask {
			id: self.id,
			task: self.task.clone(),
			finished: self.finished,
		};
	}
}

pub trait ToDoStore {
	fn get(&self, id: i32) -> Result<ToDoTask, Box<dyn Error>>;
	fn get_all(&self) -> Result<Vec<ToDoTask>, Box<dyn Error>>;
	fn add(&self, task: ToDoTask) -> Result<(), Box<dyn Error>>;
	fn update(&self, task: ToDoTask) -> Result<(), Box<dyn Error>>;
	//TODO
	//fn remove(&self, task: ToDoTask) ->  Result<(), Box<dyn Error>>;
}
