use std::error::Error;
use std::io;

fn main(){
	let todo_store = storage::TodoStorer::setup("todo.db".to_string(), true).unwrap();

	loop {
		let valid_args = "list, get [id], add, update";

		println!("\nEnter command({}): ", valid_args);
		println!("----------------------------------");
		let mut x = String::new();
		io::stdin().read_line(&mut x).expect("Error reading input");

		let args: Vec<&str> = x.split_whitespace().collect();
		if args.len() == 0 {
			println!("Missing program argument\nValid args are ({})", valid_args);
			return ();
		}

		let get_enum_from_program_arg = |program_arg: &str| -> Result<ToDoActions, Box<Error>> {
			match program_arg.trim().to_lowercase().as_ref() {
				"get" => Ok(ToDoActions::Get),
				"list" => Ok(ToDoActions::List),
				"add" => Ok(ToDoActions::Add),
				"update" => Ok(ToDoActions::Update),
				_ => Err("Invalid program argument")?,
			}
		};

		let program_action = get_enum_from_program_arg(&args[0]).unwrap();
		match program_action {
			ToDoActions::Get => {
				println!("\nSelected get task mode!\n");
				if args.len() != 2 {
					println!("Missing task id");
					continue;
				}
				let task_id: i32 = args[1].parse().expect("Invalid id given");
				let stored_task = todo_store.get(task_id).expect("Unable to get task");
				stored_task.pretty_print_to_console();
			},
			ToDoActions::List => {
				println!("\nSelected list tasks mode!\n");
				println!("Currently stored tasks are:");
				let stored_todo_tasks = todo_store.get_all().expect("Unable to retrieve todo tasks");
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

				let _new_task = ToDoTask{
					id:None,
					task: String::from(task_name),
					finished:false,
				};
				todo_store.add(_new_task).expect("Unable to add task");
			},
			ToDoActions::Update => {

				let currently_stored_tasks = &todo_store.get_all().expect("Unable to retrieve tasks when trying to update");
				println!("\nCurrently stored Tasks:");
				for task in currently_stored_tasks {
					task.copy().pretty_print_to_console();
				}

				println!("\nSelected task updating mode!\nEnter id of task which you want to modifiy");
				let mut x = String::new();
				io::stdin().read_line(&mut x).expect("Error reading input");
				let task_id: i32 = x.trim().parse().expect("Unable to parse task id");

				println!("Enter new task name(leaving empty will change nothing):");
				let mut x = String::new();
				io::stdin().read_line(&mut x).expect("Error reading input");
				let potentially_new_task_name: &str = x.trim();

				let mut task_name = potentially_new_task_name;
				if potentially_new_task_name.is_empty() {
					let mut found_task = false;
					for task in currently_stored_tasks {
						if task.id.unwrap() == task_id {
							found_task = true;
							task_name =  &task.task;
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

				let task = ToDoTask{
					id: Some(task_id),
					task: task_name.to_string(),
					finished: is_task_finished,
				};
				todo_store.update(task).expect("Unable to update task");
			},
		};
	}
}

#[derive(Debug, Clone)]
pub struct ToDoTask {
	id: Option<i32>, //Option because this is set by the database
	task: String,
	finished: bool,
}
impl ToDoTask {
	fn pretty_print_to_console(self){
		let task_id = match self.id.is_some() {
			true => self.id.unwrap().to_string(),
			false => String::from(""),
		};
		println!(" - ID: {}, Task: {}, Finished: {:?}", task_id, self.task, self.finished)
	}
	//cant derive copy trait, implementing it here instead
	fn copy (&self) -> Self {
		return ToDoTask{
			id: self.id,
			task: self.task.clone(),
			finished: self.finished,
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

trait ToDoStore {
	//TODO make setup nicer, input arg is implicitly hard coupled to sqlite
	fn setup(db_file_path: String, reset_store: bool) -> Result<Box<Self>, Box<Error>>;
	//fn run_tx() -> Box<Error>;
	fn get(&self, id: i32) -> Result<ToDoTask, Box<Error>>;
	fn get_all(&self) -> Result<Vec<ToDoTask>, Box<Error>>;
	fn add(&self, task: ToDoTask) -> Result<(), Box<Error>>;
	fn update(&self, task: ToDoTask) -> Result<(), Box<Error>>;
}

pub mod storage{

	use std::error::Error;
	use crate::ToDoTask;
	use crate::ToDoStore;
	use rusqlite::{Connection};
	use rusqlite::NO_PARAMS;

	pub struct TodoStorer{
		conn: rusqlite::Connection
	}
	/// Retrieves a single task if task_id is given else if task_id is omitted then all tasks are retrieved
	///
	// Function probably does to much, potential refactor, it is private so okey, leaving as is atm
	fn get_tasks(todo_storer: &TodoStorer, task_id: Option<i32>) -> Result<Vec<ToDoTask>, Box<Error>> {

		let (sql_condition, sql_input_parameters) = match task_id.is_some() {
			true => {
				(" WHERE id = ?1", vec![task_id.unwrap().to_string()])
			},
			false => {
				("", vec![])
			}
		};

		let mut stmt = todo_storer.conn.prepare(
			&(String::from(
			"SELECT
				id,
				task,
				finished
			FROM todo") + sql_condition)).expect("Invalid sql query");

		let todo_tasks_result = stmt.query_map::<ToDoTask, _, _>(
			sql_input_parameters,
			|row|{

			let mut is_task_finished = false;
			//due to sqlite being dynamically typed, the type of a boolean is uncertain
			//ugly
			//TODO find out how to do this better with less boilerplate or just use an i8 as sqlite column type instead of boolean
			match row.get::<_, String>(2)
			{
				Ok(value) => {
					if value == "true" {
						is_task_finished = true;
					}
				}
				Err(err) => {
					//if this is false then the column is stored is stored as a INT
					if err != rusqlite::Error::InvalidColumnType(2, rusqlite::types::Type::Integer) {
						return Err(err.into())
					}

					match row.get::<_, i32>(2) {
						Ok(value) => {
							if value == 1 {
								is_task_finished = true
							}
						},
						Err(err) => return Err(err.into())
					}
				}
			}
			Ok(
				ToDoTask{
					id: row.get(0)?,
					task: row.get(1)?,
					finished: is_task_finished,
				}
			)
		}).expect("Unable to get task/tasks");

		//TODO make functional instead
		let mut todo_tasks: Vec<ToDoTask> = vec![];
		for task in todo_tasks_result {
			match task {
				Ok(t) => todo_tasks.push(t),
				Err(err) => return Err(err.into())
			};
		}

		return Ok(todo_tasks)
	}

	impl ToDoStore for TodoStorer {

		fn setup(db_file_path: String, reset_store: bool) -> Result<Box<Self>, Box<Error>>{

			let storer = TodoStorer{
				conn: Connection::open(db_file_path).expect("unable to open sqlite database")
			};

			if reset_store {
				match storer.conn.execute(
					"DROP TABLE IF EXISTS todo",
					NO_PARAMS,
				) {
					Ok(_) => {
						storer.conn.execute(
							"CREATE TABLE todo (
								id INTEGER PRIMARY KEY AUTOINCREMENT,
								task TEXT NOT NULL,
								finished BOOLEAN DEFAULT FALSE
							)",
							NO_PARAMS,
						).unwrap();
					},
					Err(err) => return Err(err.into())
				}
			}

			Ok(Box::new(storer))
		}

		fn get(&self, id: i32) -> Result<ToDoTask, Box<Error>> {
			let tasks = get_tasks(&self, Some(id));
			match tasks {
				Ok(tasks) => {
					if tasks.len() == 1 {
						return Ok(tasks[0].copy());
					}
					return Err("Invalid number of tasks found")?
				},
				Err(err) => return Err(err.into())
			}
		}
		fn get_all(&self) -> Result<Vec<ToDoTask>, Box<Error>> {
			return get_tasks(&self, None);
		}

		fn add(&self, t: ToDoTask) -> Result<(), Box<Error>> {
			match self.conn.execute(
				"INSERT INTO todo
				(task, finished)
				VALUES
				(?1, 0)
				",
				&[t.task],
			){
				Ok(rows_affected) => {
					if rows_affected != 1 {
						Err("Wrong number of rows affected")?; //should be a panic but trying out mixing return error types
					}
					Ok(())
				},
				Err(err) => Err(err.into()),
			}
		}

		fn update(&self, t: ToDoTask) -> Result<(), Box<Error>>{
			match self.conn.execute(
				"UPDATE todo SET
					task = ?1,
					finished = ?2
				WHERE id = ?3
				",
				&[
					t.task,
					t.finished.to_string(),
					t.id.unwrap().to_string()
				],
			){
				Ok(rows_affected) => {
					if rows_affected != 1 {
						Err("Wrong number of rows affected")? //should be a panic but trying out mixing return error types
					}
					Ok(())
				},
				Err(err) => {
					return Err(err.into())
				}
			}
		}
	}
}
