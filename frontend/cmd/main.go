//go:generate ../scripts/generate.sh

package main

import (
	"context"
	"flag"
	"fmt"
	"log"
	"time"

	"github.com/jhzn/todo_app/m/v2/internal/app"
	pb "github.com/jhzn/todo_app/m/v2/pkg/proto"
	"google.golang.org/grpc"
)

func main() {
	var grpcClientPort, httpServerPort int
	flag.IntVar(&grpcClientPort, "grpc-client-port", 9000, "The GPRC client port which this application should connect to")
	flag.IntVar(&httpServerPort, "http-server-port", 1337, "The HTTP server port where this application is served")
	flag.Parse()

	log.Printf("Attempting to connect to grpc server on port %d", grpcClientPort)
	ctx, cancelFunc := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancelFunc()
	conn, err := grpc.DialContext(ctx, fmt.Sprintf(":%d", grpcClientPort), grpc.WithInsecure(), grpc.WithBlock())
	if err != nil {
		log.Fatalf("fail to dial: %v", err)
	}
	defer conn.Close()
	client := pb.NewTodoClient(conn)
	server := app.Server{Store: client}

	err = server.Serve(uint16(httpServerPort))
	if err != nil {
		panic(err)
	}
}
