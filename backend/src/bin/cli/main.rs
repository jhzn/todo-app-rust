use std::error::Error;
use std::io;
use todo_app::types::*;
extern crate clap;
use clap::{App, Arg};

//These are the possible commandline arguments
#[derive(Debug)]
enum TodoAction {
	Add,
	Update,
	Get,
	List,
	Exit,
}
fn main() {
	let matches = App::new("Todo-Task CLI application")
		.version("1.0")
		.author("Johan Håkansson")
		.about("A CLI application to create todo tasks")
		.arg(
			Arg::with_name("reset-storage")
				.long("reset-storage")
				.help("Whether the SQLite database should be emptied and created anew"),
		)
		.get_matches();

	let should_reset_storage = matches.is_present("reset-storage");
	let todo_store = todo_app::sqlite::setup("todo.db".to_string(), should_reset_storage).unwrap();

	loop {
		let possible_cmdline_args = "list, get [id], add, update, exit";

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

		let get_enum_from_program_arg = |program_arg: &str| -> Result<TodoAction, Box<dyn Error>> {
			match program_arg.trim().to_lowercase().as_ref() {
				"get" => Ok(TodoAction::Get),
				"list" => Ok(TodoAction::List),
				"add" => Ok(TodoAction::Add),
				"update" => Ok(TodoAction::Update),
				"exit" => Ok(TodoAction::Exit),
				_ => Err("Invalid program argument")?,
			}
		};

		match get_enum_from_program_arg(&args[0]).unwrap() {
			TodoAction::Exit => {
				println!("Ok then. See you later.");
				break;
			}
			TodoAction::Get => {
				println!("\nSelected get task mode!\n");
				if args.len() != 2 {
					println!("Missing task id");
					continue;
				}
				let task_id: i32 = args[1].parse().expect("Invalid id given");
				let stored_task = todo_store.get(task_id).expect("Unable to get task");
				stored_task.pretty_print_to_stdout();
			}
			TodoAction::List => {
				println!("\nSelected list tasks mode!\n");
				println!("Currently stored tasks are:");
				let stored_todo_tasks = todo_store.list().expect("Unable to retrieve todo tasks");
				if stored_todo_tasks.is_empty() {
					println!("No tasks are stored!");
				} else {
					for task in stored_todo_tasks {
						task.pretty_print_to_stdout();
					}
				}
			}
			TodoAction::Add => {
				println!("\nSelected task creation mode!\nEnter task name:");
				let mut x = String::new();
				io::stdin().read_line(&mut x).expect("Error reading input");
				let task_name: &str = x.trim();

				let _new_task = TodoTask {
					id: None,
					task: String::from(task_name),
					finished: false,
				};
				let stored_task = todo_store.add(_new_task).expect("Unable to add task");
				println!("Saved task:");
				stored_task.pretty_print_to_stdout();
			}
			TodoAction::Update => {
				let currently_stored_tasks = &todo_store
					.list()
					.expect("Unable to retrieve tasks when trying to update");
				println!("\nCurrently stored Tasks:");
				for task in currently_stored_tasks {
					task.clone().pretty_print_to_stdout();
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

				let task = TodoTask {
					id: Some(task_id),
					task: task_name.to_string(),
					finished: is_task_finished,
				};
				todo_store.update(task).expect("Unable to update task");
			}
		};
	}
}
