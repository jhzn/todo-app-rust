fn main(){
	let todo_store = storage::TodoStorer::setup("todo.db".to_string()).unwrap();

	let _new_task = ToDoTask{
		id:None,
		task: String::from("Buy milk"),
		finished:false,
	};
	todo_store.add(_new_task).unwrap();
	let mut stored_task = todo_store.get(1).expect("Unable to get task after creating it");
	println!("Stored task = {:?}", stored_task);

	stored_task.finished = true;
	todo_store.update(stored_task).expect("Unable to update task");

	let updated_task = todo_store.get(1).expect("Unable to get task from after updating it");
	println!("Updated task = {:?}", updated_task);
}

#[derive(Debug)]
pub struct ToDoTask {
	id: Option<i32>, //Option because this is set by the database
	task: String,
	finished: bool,
}
trait ToDoStore {
	//TODO make setup nicer, input arg is implicitly hard coupled to sqlite
	fn setup(db_file_path: String) -> std::result::Result<Box<Self>, Box<std::error::Error>>;
	fn get(&self, id: i32) -> std::result::Result<ToDoTask, Box<std::error::Error>>;
	fn add(&self, task: ToDoTask) -> std::result::Result<(), Box<std::error::Error>>;
	fn update(&self, task: ToDoTask) -> std::result::Result<(), Box<std::error::Error>>;
}

pub mod storage{

	use crate::ToDoTask;
	use crate::ToDoStore;
	use rusqlite::{Connection};
	use rusqlite::NO_PARAMS;

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

		fn get(&self, id: i32) -> std::result::Result<ToDoTask, Box<std::error::Error>> {
			match self.conn.query_row::<ToDoTask, _, _>(
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
				}
				) {
					Ok(task) => Ok(task),
					Err(err) => Err(err.into())
				}
		}

		fn add(&self, t: ToDoTask) -> Result<(), Box<std::error::Error>> {
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

		fn update(&self, t: ToDoTask) -> Result<(), Box<std::error::Error>>{
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
				Err(err) => Err(err.into())
			}
		}
	}
}
