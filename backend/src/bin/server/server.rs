extern crate grpc;
extern crate protobuf;

use std::sync::Mutex;
use todo_app::grpc::todo;
use todo_app::grpc::todo_grpc;

pub struct TodoServer {
	//Need a mutex here because the underlying sqlite connection does not support multithreaded access
	//TODO hide this detail away because this only needed for this specific database backend
	pub storage: Box<Mutex<dyn todo_app::types::TodoTaskStore + Send>>,
}

impl todo_grpc::Todo for TodoServer {
	fn get_task(
		&self,
		_: grpc::RequestOptions,
		p: todo::GetTaskRequest,
	) -> grpc::SingleResponse<todo::GetTaskResponse> {
		let store = self.storage.lock().expect("unable to aquire mutex lock");
		let stored_task = store.get(p.TaskID as i32).unwrap();
		let mut response = todo::GetTaskResponse::new();
		response.set_Task(conv_app_todo_task(stored_task));
		grpc::SingleResponse::completed(response)
	}

	fn list_tasks(
		&self,
		_: grpc::RequestOptions,
		_: todo::ListTasksRequest,
	) -> grpc::SingleResponse<todo::ListTasksResponse> {
		let store = self.storage.lock().expect("unable to aquire mutex lock");
		let stored_todo_tasks = store.list().expect("Unable to retrieve todo tasks");

		let mut response_tasks = protobuf::RepeatedField::<todo::TodoTaskEntity>::new();
		for t in stored_todo_tasks.iter() {
			response_tasks.push(conv_app_todo_task(t.clone()))
		}
		let mut response = todo::ListTasksResponse::new();
		response.set_Tasks(response_tasks);
		grpc::SingleResponse::completed(response)
	}

	fn add_task(
		&self,
		_: grpc::RequestOptions,
		p: todo::AddTaskRequest,
	) -> grpc::SingleResponse<todo::AddTaskResponse> {
		let store = self.storage.lock().expect("unable to aquire mutex lock");
		let stored_task = store
			.add(conv_grpc_todo_task(p.get_Task().clone()))
			.expect("unable to save todo task");
		let mut response = todo::AddTaskResponse::new();
		response.set_Task(conv_app_todo_task(stored_task.clone()));
		grpc::SingleResponse::completed(response)
	}

	fn update_task(
		&self,
		_: grpc::RequestOptions,
		_: todo::AddTaskRequest,
	) -> grpc::SingleResponse<todo::UpdateTaskResponse> {
		todo!()
	}
}

fn conv_app_todo_task(t: todo_app::types::TodoTask) -> todo::TodoTaskEntity {
	let id: i32 = match t.id {
		Some(id) => id,
		None => panic!("todo task id is empty"),
	};
	let mut inner_todo_task = todo::TodoTask::new();
	inner_todo_task.set_Name(t.task);
	inner_todo_task.set_IsFininshed(t.finished);

	let mut conv_task = todo::TodoTaskEntity::new();
	conv_task.set_ID(id as i64);
	conv_task.set_Task(inner_todo_task);
	conv_task
}

fn conv_grpc_todo_task(t: todo::TodoTask) -> todo_app::types::TodoTask {
	let task = todo_app::types::TodoTask {
		id: None,
		task: String::from(t.get_Name().clone()),
		finished: t.get_IsFininshed(),
	};
	task
}
