extern crate grpc;
extern crate protobuf;

use std::sync::Mutex;
use todo_app::grpc::todo as proto;
use todo_app::grpc::todo_grpc;
use todo_app::types as app_type;

pub struct TodoServer {
	//Need a mutex here because the underlying sqlite connection does not support multithreaded access
	//TODO hide this detail away because this only needed for this specific database backend
	pub storage: Box<Mutex<dyn app_type::TodoTaskStore + Send>>,
}

impl todo_grpc::Todo for TodoServer {
	fn get_task(
		&self,
		_: grpc::RequestOptions,
		p: proto::GetTaskRequest,
	) -> grpc::SingleResponse<proto::GetTaskResponse> {
		let store = self.storage.lock().expect("unable to aquire mutex lock");
		let stored_task = store.get(p.TaskID as i32).unwrap();
		let mut response = proto::GetTaskResponse::new();
		response.set_Task(conv_app_todo_task(stored_task));
		grpc::SingleResponse::completed(response)
	}

	fn list_tasks(
		&self,
		_: grpc::RequestOptions,
		_: proto::ListTasksRequest,
	) -> grpc::SingleResponse<proto::ListTasksResponse> {
		let store = self.storage.lock().expect("unable to aquire mutex lock");
		let stored_todo_tasks = store.list().expect("Unable to retrieve todo tasks");

		let mut response_tasks = protobuf::RepeatedField::<proto::TodoTaskEntity>::new();
		for t in stored_todo_tasks.iter() {
			response_tasks.push(conv_app_todo_task(t.clone()))
		}
		let mut response = proto::ListTasksResponse::new();
		response.set_Tasks(response_tasks);
		grpc::SingleResponse::completed(response)
	}

	fn add_task(
		&self,
		_: grpc::RequestOptions,
		p: proto::AddTaskRequest,
	) -> grpc::SingleResponse<proto::AddTaskResponse> {
		let store = self.storage.lock().expect("unable to aquire mutex lock");
		let stored_task = store
			.add(conv_grpc_todo_task(p.get_Task().clone()))
			.expect("unable to save todo task");
		let mut response = proto::AddTaskResponse::new();
		response.set_Task(conv_app_todo_task(stored_task.clone()));
		grpc::SingleResponse::completed(response)
	}

	fn update_task(
		&self,
		_: grpc::RequestOptions,
		_: proto::AddTaskRequest,
	) -> grpc::SingleResponse<proto::UpdateTaskResponse> {
		todo!()
	}
}

fn conv_app_todo_task(t: app_type::TodoTask) -> proto::TodoTaskEntity {
	let id: i32 = match t.id {
		Some(id) => id,
		None => panic!("todo task id is empty"),
	};
	let mut inner_todo_task = proto::TodoTask::new();
	inner_todo_task.set_Name(t.task);
	inner_todo_task.set_IsFininshed(t.finished);

	let mut conv_task = proto::TodoTaskEntity::new();
	conv_task.set_ID(id as i64);
	conv_task.set_Task(inner_todo_task);
	conv_task
}

fn conv_grpc_todo_task(t: proto::TodoTask) -> app_type::TodoTask {
	let task = app_type::TodoTask {
		id: None,
		task: String::from(t.get_Name().clone()),
		finished: t.get_IsFininshed(),
	};
	task
}
