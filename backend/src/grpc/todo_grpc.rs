// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]


// interface

pub trait Todo {
    fn get_task(&self, o: ::grpc::RequestOptions, p: super::todo::GetTaskRequest) -> ::grpc::SingleResponse<super::todo::GetTaskResponse>;

    fn list_tasks(&self, o: ::grpc::RequestOptions, p: super::todo::ListTasksRequest) -> ::grpc::SingleResponse<super::todo::ListTasksResponse>;

    fn add_task(&self, o: ::grpc::RequestOptions, p: super::todo::AddTaskRequest) -> ::grpc::SingleResponse<super::todo::AddTaskResponse>;

    fn update_task(&self, o: ::grpc::RequestOptions, p: super::todo::AddTaskRequest) -> ::grpc::SingleResponse<super::todo::UpdateTaskResponse>;
}

// client

pub struct TodoClient {
    grpc_client: ::std::sync::Arc<::grpc::Client>,
    method_GetTask: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::todo::GetTaskRequest, super::todo::GetTaskResponse>>,
    method_ListTasks: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::todo::ListTasksRequest, super::todo::ListTasksResponse>>,
    method_AddTask: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::todo::AddTaskRequest, super::todo::AddTaskResponse>>,
    method_UpdateTask: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::todo::AddTaskRequest, super::todo::UpdateTaskResponse>>,
}

impl ::grpc::ClientStub for TodoClient {
    fn with_client(grpc_client: ::std::sync::Arc<::grpc::Client>) -> Self {
        TodoClient {
            grpc_client: grpc_client,
            method_GetTask: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/todo.Todo/GetTask".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_ListTasks: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/todo.Todo/ListTasks".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_AddTask: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/todo.Todo/AddTask".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_UpdateTask: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/todo.Todo/UpdateTask".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }
}

impl Todo for TodoClient {
    fn get_task(&self, o: ::grpc::RequestOptions, p: super::todo::GetTaskRequest) -> ::grpc::SingleResponse<super::todo::GetTaskResponse> {
        self.grpc_client.call_unary(o, p, self.method_GetTask.clone())
    }

    fn list_tasks(&self, o: ::grpc::RequestOptions, p: super::todo::ListTasksRequest) -> ::grpc::SingleResponse<super::todo::ListTasksResponse> {
        self.grpc_client.call_unary(o, p, self.method_ListTasks.clone())
    }

    fn add_task(&self, o: ::grpc::RequestOptions, p: super::todo::AddTaskRequest) -> ::grpc::SingleResponse<super::todo::AddTaskResponse> {
        self.grpc_client.call_unary(o, p, self.method_AddTask.clone())
    }

    fn update_task(&self, o: ::grpc::RequestOptions, p: super::todo::AddTaskRequest) -> ::grpc::SingleResponse<super::todo::UpdateTaskResponse> {
        self.grpc_client.call_unary(o, p, self.method_UpdateTask.clone())
    }
}

// server

pub struct TodoServer;


impl TodoServer {
    pub fn new_service_def<H : Todo + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/todo.Todo",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/todo.Todo/GetTask".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.get_task(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/todo.Todo/ListTasks".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.list_tasks(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/todo.Todo/AddTask".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.add_task(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/todo.Todo/UpdateTask".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.update_task(o, p))
                    },
                ),
            ],
        )
    }
}
