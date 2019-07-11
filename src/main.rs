

fn main(){

	let _new_task = Task{
		id:None,
		task: String::from("Buy milk"),
		finished:false,
	};

	let todo_storer = storage::TodoStorer::setup("todo.db".to_string()).unwrap();

	todo_storer.add(_new_task).unwrap();
	let mut task_res = todo_storer.get(1).expect("Unable to get task from DB1");
	println!("TASK1 = {:?}", task_res);

	task_res.finished = true;

	todo_storer.update(task_res).unwrap();


	let task_res2 = todo_storer.get(1).expect("Unable to get task from DB2");
	println!("TASK2 = {:?}", task_res2);
}

#[derive(Debug)]
pub struct Task {
	id: Option<i32>,
	task: String,
	finished: bool,
}
trait ToDoStore {
	fn setup(db_file_path: String) -> std::result::Result<Box<Self>, Box<std::error::Error>>;
	fn get(&self, id: i32) -> std::result::Result<Task, Box<std::error::Error>>;
	fn add(&self, task: Task) -> std::result::Result<(), Box<std::error::Error>>;
	fn update(&self, task: Task) -> std::result::Result<(), Box<std::error::Error>>;
}


pub mod storage{

	use crate::Task;
	use crate::ToDoStore;
	use rusqlite::{Connection};
	use rusqlite::NO_PARAMS;

	//#[derive(Copy)]
	pub struct TodoStorer{
		conn: rusqlite::Connection
	}

	impl ToDoStore for TodoStorer {

		fn setup(db_file_path: String) -> std::result::Result<Box<Self>, Box<std::error::Error>>{

			let storer = TodoStorer{
				conn: Connection::open(db_file_path).expect("unable to open sqlite database")
			};

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
					Ok(Box::new(storer))
				},
				Err(err) => Err(err.into())
			}
		}

		fn get(&self, id: i32) -> std::result::Result<Task, Box<std::error::Error>> {
			match self.conn.query_row::<Task, _, _>(
				"SELECT
					id,
					task,
					finished
				FROM todo
				WHERE id = ?1
				",
				&[id.to_string()],
				|row|{

					let mut is_task_finished = false;
					//due to sqlite being dynamically typed, the type of a boolean is uncertain
					//ugly
					//TODO find out how to do this better with a more generic solution or just use an i8 as sqlite column type
					match row.get::<_, String>(2)
					{
						Ok(value) => {
							if value == "true" {
								is_task_finished = true;
							}
						}
						Err(err) => {
							//check error type
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
						Task{
							id: row.get(0)?,
							task: row.get(1)?,
							finished: is_task_finished,
						}
					)
				}
				) {
					Ok(task) => Ok(task),
					Err(err) => Err(err.into())
				}
		}

		fn add(&self, t: Task) -> Result<(), Box<std::error::Error>> {
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
						Err("Wrong number of rows affected")?;
					}
					Ok(())
				},
				Err(err) => Err(err.into()),
			}
		}

		fn update(&self, t: Task) -> Result<(), Box<std::error::Error>>{
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
						Err("Wrong number of rows affected")?
					}
					Ok(())
				},
				Err(err) => Err(err.into())
			}
		}
	}
}
