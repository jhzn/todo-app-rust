use crate::types;
use rusqlite::Connection;
use rusqlite::NO_PARAMS;
use std::convert::TryInto;
use std::error::Error;

pub struct Sqlite {
	conn: rusqlite::Connection,
}

pub fn setup(db_file_path: String, reset_store: bool) -> Result<Sqlite, Box<dyn Error>> {
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

// Retrieves a single task if task_id is given else if task_id is omitted then all tasks are retrieved
// Function probably does to much, potential refactor, it is private so okey, leaving as is atm
fn get_tasks(
	todo_storer: &Sqlite,
	task_id: Option<i32>,
) -> Result<Vec<types::TodoTask>, Box<dyn Error>> {
	//if task_id is set then we use a filter otherwise fetch with no filter
	let (sql_condition, sql_input_parameters) = match task_id {
		Some(id) => (" WHERE id = ?1", vec![id]),
		None => ("", vec![]),
	};

	let mut stmt = todo_storer
		.conn
		.prepare(&(String::from("SELECT id, task, finished FROM todo") + sql_condition))
		.expect("Invalid sql query");

	let todo_tasks_result = stmt
		.query_map::<types::TodoTask, _, _>(sql_input_parameters, |row| {
			let mut is_task_finished = false;
			//due to sqlite being dynamically typed, the rust type of a boolean column is uncertain
			//ugly
			//TODO find out how to do this better with less boilerplate or just use an i8 as sqlite column type instead of boolean
			match row.get::<_, String>(2) {
				Ok(value) => {
					if value == "true" {
						is_task_finished = true;
					}
				}
				Err(err) => {
					//if this is true, something is terribly wrong
					if err != rusqlite::Error::InvalidColumnType(2, rusqlite::types::Type::Integer)
					{
						return Err(err.into());
					}
					//otherwise value is stored as an INT

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
			Ok(types::TodoTask {
				id: row.get(0)?,
				task: row.get(1)?,
				finished: is_task_finished,
			})
		})
		.expect("Unable to get task/tasks");

	//TODO make functional instead
	let mut todo_tasks: Vec<types::TodoTask> = vec![];
	for task in todo_tasks_result {
		match task {
			Ok(t) => todo_tasks.push(t),
			Err(err) => return Err(err.into()),
		};
	}

	return Ok(todo_tasks);
}

impl types::TodoTaskStore for Sqlite {
	fn get(&self, id: i32) -> Result<types::TodoTask, Box<dyn Error>> {
		let tasks = get_tasks(&self, Some(id));
		match tasks {
			Ok(tasks) => {
				return Ok(tasks.get(0).expect("Invalid number of tasks found").clone());
			}
			Err(err) => return Err(err.into()),
		}
	}

	fn list(&self) -> Result<Vec<types::TodoTask>, Box<dyn Error>> {
		return get_tasks(&self, None);
	}

	fn add(&self, t: types::TodoTask) -> Result<types::TodoTask, Box<dyn Error>> {
		match self.conn.execute(
			"INSERT INTO todo (task, finished) VALUES (?1, 0)",
			&[t.task],
		) {
			Ok(rows_affected) => {
				if rows_affected != 1 {
					panic!("Wrong number of rows affected");
				}
				let id = self.conn.last_insert_rowid();
				let stored_task = self
					.get(
						(id as i64)
							.try_into()
							.expect("failed to convert task id of i32 to i64"),
					)
					.expect("could not get recently stored task");
				Ok(stored_task)
			}
			Err(err) => Err(err.into()),
		}
	}

	fn update(&self, t: types::TodoTask) -> Result<(), Box<dyn Error>> {
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
					panic!("Wrong number of rows affected");
				}
				Ok(())
			}
			Err(err) => return Err(err.into()),
		}
	}
}
