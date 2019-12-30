package app

import (
	"fmt"
	"log"
	"net/http"
	"text/template"

	"github.com/gorilla/schema"
	pb "github.com/jhzn/todo_app/m/v2/pkg/proto"
)

//TODO improve error handling

type Server struct {
	Store     pb.TodoClient
	templates *template.Template
}

type route struct {
	URLPath      string
	TemplateName string
}

func (s Server) Serve(port uint16) error {

	listRoute := route{
		URLPath:      "/list",
		TemplateName: "list.tmpl",
	}
	addRoute := route{
		URLPath:      "/add",
		TemplateName: "add.tmpl",
	}

	type navbarItem struct {
		LinkPath string
		LinkText string
	}
	tpl, err := template.New("").Funcs(
		template.FuncMap{
			//these are used to create the navbar
			"navbarItems": func() []navbarItem {
				return []navbarItem{
					navbarItem{
						LinkPath: addRoute.URLPath,
						LinkText: "Add a task",
					},
					navbarItem{
						LinkPath: listRoute.URLPath,
						LinkText: "List all tasks",
					},
				}
			},
		},
	).ParseGlob("web/templates/*.tmpl") //Note: we're using glob here, all templates are parsed, this is what makes every template which imports another template work
	if err != nil {
		panic(err)
	}
	s.templates = tpl

	//doing it this way moves the resolving of these routes to the time of startup of the application instead of having them resolved at runtime
	//it also avoids having them specified elsewhere, ie the source of truth is right here
	http.HandleFunc(
		listRoute.URLPath,
		s.handleList(listRoute.TemplateName),
	)
	http.HandleFunc(addRoute.URLPath,
		s.handleAdd(
			addRoute,
			listRoute,
		),
	)
	log.Printf("Starting web server on port %d", port)

	return http.ListenAndServe(fmt.Sprintf(":%d", port), nil)
}

func (s Server) handleList(templateName string) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		tasks, err := s.Store.ListTasks(r.Context(), &pb.ListTasksRequest{})
		if err != nil {
			panic(err)
		}

		conved_tasks := []pb.TodoTaskEntity{}
		for _, t := range tasks.Tasks {
			conved_tasks = append(conved_tasks, *t)
		}

		viewData := struct {
			Tasks []pb.TodoTaskEntity
		}{
			Tasks: conved_tasks,
		}
		err = s.templates.ExecuteTemplate(w, templateName, viewData)
		if err != nil {
			panic(err)
		}
	}
}

func (s Server) handleAdd(addRoute, listRoute route) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		switch r.Method {
		case http.MethodGet:
			s.viewAddTaskForm(addRoute, w, r)
			return
		case http.MethodPost:
			s.addTask(listRoute, w, r)
			return
		//TODO
		//case http.MethodPut:
		//return
		default:
			w.Write([]byte("invalid http method"))
		}
	}
}

func (s Server) addTask(routeToRedirectToOnSuccess route, w http.ResponseWriter, r *http.Request) {
	err := r.ParseForm()
	if err != nil {
		log.Printf("%v", err)
	}

	var addRequest struct {
		Task     string
		Finished bool
	}
	err = schema.NewDecoder().Decode(&addRequest, r.PostForm)
	if err != nil {
		log.Printf("%v", err)
	}

	_, err = s.Store.AddTask(r.Context(), &pb.AddTaskRequest{
		Task: &pb.TodoTask{
			Name:        addRequest.Task,
			IsFininshed: addRequest.Finished,
		},
	})
	if err != nil {
		log.Printf("%v", err)
	}
	http.Redirect(w, r, routeToRedirectToOnSuccess.URLPath, 301)
}

func (s Server) viewAddTaskForm(addRoute route, w http.ResponseWriter, r *http.Request) {
	err := s.templates.ExecuteTemplate(w, addRoute.TemplateName, struct{ HTTPFormActionURL string }{HTTPFormActionURL: addRoute.URLPath})
	if err != nil {
		panic(err)
	}
}
