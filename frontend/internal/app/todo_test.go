package app

import (
	"context"
	"log"
	"testing"
	"time"

	pb "github.com/jhzn/todo_app/pkg/proto"
	"google.golang.org/grpc"
)

func TestLoadTestServer(t *testing.T) {

	ctx := context.Background()

	conn, err := grpc.Dial(":9000", grpc.WithInsecure(), grpc.WithBlock())
	if err != nil {
		log.Fatalf("fail to dial: %v", err)
	}
	defer conn.Close()
	client := pb.NewTodoClient(conn)

	tasksChannel := make(chan pb.TodoTaskEntity)
	programStartTime := time.Now()

	//Limit amount of workers
	sem := make(chan bool, 500)
	go func() {
		for i := 0; i <= 100000; i++ {
			go func() {
				tasks, err := client.ListTasks(ctx, &pb.ListTasksRequest{})
				if err != nil {
					panic(err)
				}

				for _, t := range tasks.Tasks {
					tasksChannel <- *t
				}
				<-sem
			}()
			sem <- true
		}

		//Make sure the last goroutines are finished
		for i := 0; i < cap(sem); i++ {
			sem <- true
		}

		close(tasksChannel)
	}()

	//collecting the data which is being produced concurrently
	collectedTasks := []pb.TodoTaskEntity{}
	for task := range tasksChannel {
		collectedTasks = append(collectedTasks, task)
	}
	t.Logf("Load test is done. Time taken = %v", time.Since(programStartTime))
}
