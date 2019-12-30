#!/bin/sh

if ! type "protoc" > /dev/null; then
	echo "protoc compiler not found in path. You must install it. Exiting..." && exit 1
fi

cd backend && \
cargo build && \
cd - && \
cd frontend && \
go generate ./...

if [ $? -eq 0 ]; then
	echo "Succesfully generated code"
else
	echo "Unsuccesfully generated code" && exit 1
fi
