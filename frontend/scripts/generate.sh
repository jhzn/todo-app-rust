#!/bin/sh

PROJECT_ROOT=".."
OUT_DIR="$PROJECT_ROOT/frontend/pkg/proto/"

rm -rf "$OUT_DIR" && \
mkdir -p "$OUT_DIR" && \
protoc -I "$PROJECT_ROOT" --go_out=plugins=grpc:"$OUT_DIR" "$PROJECT_ROOT/todo.proto"

