## Play project with golang and rust

### A Todo Task application.

The backend is written in rust and has SQLite as storage. There is a CLI and a GRPC server.
The frontend/web-GUI is written in golang and is a client of the GRPC server.

Visit the respective directories for additional info.

### Starting web application and development environment which has hotrelead

```sh
#this builds docker images and starts application
docker-compose up

#now go to http://localhost:1337
#and hack away on some code and see the hotreleading at work
```
