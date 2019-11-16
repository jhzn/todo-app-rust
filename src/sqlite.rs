use crate::ToDoStore;
use crate::ToDoTask;
use rusqlite::Connection;
use rusqlite::NO_PARAMS;
use std::error::Error;

struct Sqlite {
	conn: rusqlite::Connection,
}

pub fn setup(db_file_path: String, reset_store: bool) -> Result<impl ToDoStore, Box<dyn Error>> {
	let store = Sqlite {
		conn: Connection::open(db_file_path).expect("unable to open sqlite database"),
	};

	if reset_store {
		match store.conn.execute("DROP TABLE IF EXISTS todo", NO_PARAMS) {
			Ok(_) => {
				store
					.conn
					.execute(
						"CREATE TABLE todo (
								id INTEGER PRIMARY KEY AUTOINCREMENT,
								task TEXT NOT NULL,
								finished BOOLEAN DEFAULT FALSE
							)",
						NO_PARAMS,
					)
					.unwrap();
			}
			Err(err) => return Err(err.into()),
		}
	}

	Ok(store)
}
/// Retrieves a single task if task_id is given else if task_id is omitted then all tasks are retrieved
///
// Function probably does to much, potential refactor, it is private so okey, leaving as is atm
fn get_tasks(todo_storer: &Sqlite, task_id: Option<i32>) -> Result<Vec<ToDoTask>, Box<dyn Error>> {
	let (sql_condition, sql_input_parameters) = match task_id.is_some() {
		true => (" WHERE id = ?1", vec![task_id.unwrap().to_string()]),
		false => ("", vec![]),
	};

	let mut stmt = todo_storer
		.conn
		.prepare(&(String::from("SELECT id, task, finished FROM todo") + sql_condition))
		.expect("Invalid sql query");

	let todo_tasks_result = stmt
		.query_map::<ToDoTask, _, _>(sql_input_parameters, |row| {
			let mut is_task_finished = false;
			//due to sqlite being dynamically typed, the type of a boolean is uncertain
			//ugly
			//TODO find out how to do this better with less boilerplate or just use an i8 as sqlite column type instead of boolean
			match row.get::<_, String>(2) {
				Ok(value) => {
					if value == "true" {
						is_task_finished = true;
					}
				}
				Err(err) => {
					//if this is false then the column is stored is stored as a INT
					if err != rusqlite::Error::InvalidColumnType(2, rusqlite::types::Type::Integer)
					{
						return Err(err.into());
					}

					match row.get::<_, i32>(2) {
						Ok(value) => {
							if value == 1 {
								is_task_finished = true
							}
						}
						Err(err) => return Err(err.into()),
					}
				}
			}
			Ok(ToDoTask {
				id: row.get(0)?,
				task: row.get(1)?,
				finished: is_task_finished,
			})
		})
		.expect("Unable to get task/tasks");

	//TODO make functional instead
	let mut todo_tasks: Vec<ToDoTask> = vec![];
	for task in todo_tasks_result {
		match task {
			Ok(t) => todo_tasks.push(t),
			Err(err) => return Err(err.into()),
		};
	}

	return Ok(todo_tasks);
}

impl ToDoStore for Sqlite {
	fn get(&self, id: i32) -> Result<ToDoTask, Box<dyn Error>> {
		let tasks = get_tasks(&self, Some(id));
		match tasks {
			Ok(tasks) => {
				if tasks.len() == 1 {
					return Ok(tasks[0].copy());
				}
				return Err("Invalid number of tasks found")?;
			}
			Err(err) => return Err(err.into()),
		}
	}

	fn get_all(&self) -> Result<Vec<ToDoTask>, Box<dyn Error>> {
		return get_tasks(&self, None);
	}

	fn add(&self, t: ToDoTask) -> Result<(), Box<dyn Error>> {
		match self.conn.execute(
			"INSERT INTO todo (task, finished) VALUES (?1, 0)",
			&[t.task],
		) {
			Ok(rows_affected) => {
				if rows_affected != 1 {
					Err("Wrong number of rows affected")?; //should be a panic but trying out mixing return error types
				}
				Ok(())
			}
			Err(err) => Err(err.into()),
		}
	}

	fn update(&self, t: ToDoTask) -> Result<(), Box<dyn Error>> {
		match self.conn.execute(
			"UPDATE todo SET
					task = ?1,
					finished = ?2
				WHERE id = ?3
				",
			&[t.task, t.finished.to_string(), t.id.unwrap().to_string()],
		) {
			Ok(rows_affected) => {
				if rows_affected != 1 {
					Err("Wrong number of rows affected")? //should be a panic but trying out mixing return error types
				}
				Ok(())
			}
			Err(err) => return Err(err.into()),
		}
	}
}
