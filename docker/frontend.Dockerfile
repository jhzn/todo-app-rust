FROM golang:1.14-alpine as base

ENV PROTOC_VERSION=3.11.4

RUN apk add git curl protoc

RUN go get -u github.com/golang/protobuf/protoc-gen-go





#This build stage assumes that you'll volume mounts during runtime
FROM base as develop

WORKDIR /app
RUN env GO111MODULE=on go get github.com/cortesi/modd/cmd/modd

ENTRYPOINT [ "modd" ]



FROM base as scratch-builder

WORKDIR /app
COPY ./frontend/go.mod ./frontend/go.sum ./
RUN go mod download

COPY frontend .
COPY todo.proto .

RUN go generate ./...
RUN go build -o /todo_app ./cmd/*.go




FROM scratch
COPY --from=builder /todo_app /todo_app
ENTRYPOINT [ "/todo_app" ]
