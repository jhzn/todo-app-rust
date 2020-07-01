FROM rust:1.44 as base

RUN apt update
RUN apt install -y protobuf-compiler




#assumes code is volume mounted
FROM base as develop

RUN apt install curl tar

RUN curl -L https://github.com/cortesi/modd/releases/download/v0.8/modd-0.8-linux64.tgz --output modd.tgz
RUN tar xf modd.tgz && cp modd-0.8-linux64/modd /usr/local/bin/

WORKDIR /app
ENTRYPOINT [ "modd" ]




FROM base as scratch-builder

WORKDIR /app

COPY backend .
COPY todo.proto .

RUN cargo install --path .
CMD ["/app/server"]




FROM scratch

COPY --from=scratch-builder /app/server /server
