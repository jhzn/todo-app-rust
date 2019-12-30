extern crate grpc;
use std::sync::Mutex;
use std::thread;
use todo_app::grpc::todo_grpc;

extern crate clap;
use clap::{App, Arg};
mod server;

fn main() {
	let matches = App::new("Todo-Task GRPC server")
		.version("1.0")
		.author("Johan Håkansson")
		.about("A GRPC server which serves as all servers should do")
		.arg(
			Arg::with_name("port")
				.long("port")
				.default_value("9000")
				.takes_value(true)
				.help("The port the GRPC server should run on"),
		)
		.arg(
			Arg::with_name("reset-store")
				.long("reset-store")
				.help("Whether the SQLite database should be emptied and created anew"),
		)
		.get_matches();

	//program args
	let port = clap::value_t!(matches.value_of("port"), u16).expect("Port is not a valid number");
	let should_reset_store = matches.is_present("reset-store");

	//setup database
	let todo_store = todo_app::sqlite::setup("todo.db".to_string(), should_reset_store)
		.expect("failed to setup SQLite database");

	//Setup GRPC server implementation
	let todo_server = server::TodoServer {
		storage: Box::new(Mutex::new(todo_store)),
	};

	//Inject implementation and start server
	let mut server = grpc::ServerBuilder::new_plain();
	server.http.set_port(port);
	server.add_service(todo_grpc::TodoServer::new_service_def(todo_server));
	let _server = server.build().expect("server");

	println!("server started on port {}", port);

	loop {
		thread::park();
	}
}
