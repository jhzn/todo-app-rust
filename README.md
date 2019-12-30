## Play project with golang and rust

### A Todo Task application.

The backend is written in rust and has SQLite as storage. There is a CLI and a GRPC server.
The frontend/web-GUI is written in golang and is a client of the GRPC server.

Visit the respective directories for additional info.

### Building application

```shell
#Assuming a working rust/golang/protoc dev environment, this script ought to work.
./build.sh
```

### Starting web application

```shell
#Shell 1
cd backend && \
cargo run --bin server

#Shell 2
cd frontend && \
go run cmd/main.go

#now go to http://localhost:1337
```
