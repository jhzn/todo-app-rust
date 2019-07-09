use rusqlite::{Connection};
use rusqlite::NO_PARAMS;

fn main(){
	let conn = Connection::open("todo.db").unwrap();

	conn.execute(
		"drop table if exists todo",
		NO_PARAMS,
	).unwrap();

	conn.execute(
		"create table todo (
			 id integer primary key autoincrement,
			 task text not null,
			 finished boolean default false
		 )",
		NO_PARAMS,
	).unwrap();

	let _new_task = Task{
		id:None,
		task: String::from("Buy milk"),
		finished:false,
	};

	add(_new_task, &conn).unwrap();
	let mut task_res = get(1, &conn).expect("Unable to get task from DB1");
	println!("TASK1 = {:?}", task_res);

	task_res.finished = true;

	update(task_res, &conn).unwrap();

	let task_res2 = get(1, &conn).expect("Unable to get task from DB2");
	println!("TASK2 = {:?}", task_res2);
}

#[derive(Debug)]
struct Task {
	id: Option<i32>,
	task: String,
	finished: bool,
}

// trait ToDoStorage {
// 	fn get(id: i32, conn: rusqlite::Connection) -> std::result::Result<Task, rusqlite::Error>;
// 	fn add(self, conn: rusqlite::Connection);
// 	fn update(self, conn: rusqlite::Connection);
// }


fn get(id: i32, conn: &rusqlite::Connection) -> std::result::Result<Task, Box<std::error::Error>> {
	match conn.query_row::<Task, _, _>(
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
			//due to sqlite storing this a string or int we need to do this.
			//ugly
			//TODO find out how to do this better with a more generic solution
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

fn add(t: Task, conn: &rusqlite::Connection) -> Result<(), Box<std::error::Error>> {
	match conn.execute(
		"INSERT INTO todo
		(task, finished)
		VALUES
		(?1, 0)
		",
		&[t.task],
	){
	 	Ok(rows_affected) => {
			 if rows_affected != 1 {
				 Err("Nothing was inserted")?;
			 }
			Ok(())
		 },
        Err(err) => Err(err.into()),
	}
}

fn update(t: Task, conn: &rusqlite::Connection) -> Result<(), Box<std::error::Error>>{
	match conn.execute(
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
				Err("Wrong number of affected rows")?
			}
			Ok(())
		},
		Err(err) => Err(err.into())
	}
}