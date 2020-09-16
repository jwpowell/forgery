package main

import (
	"net/http"
	"time"

	"forgery/api"
	"forgery/api/login"
	"forgery/api/users"
	"forgery/db"

	"github.com/wspowell/spiderweb"
	"github.com/wspowell/spiderweb/endpoint"
	"github.com/wspowell/spiderweb/logging"
)

const (
	host = "localhost"
	port = 8080

	logLevel = logging.LevelDebug
)

func main() {
	userStore := db.NewInMemoryUserStore()

	serverConfig := spiderweb.NewServerConfig(host, port, endpoint.Config{
		Auther:            api.NoopAuth{},
		ErrorHandler:      api.ErrorJsonWithCodeResponse{},
		LogConfig:         logging.NewConfig(logLevel, map[string]interface{}{}),
		MimeTypeHandlers:  map[string]endpoint.MimeTypeHandler{},
		RequestValidator:  api.NoopRequestValidator{},
		ResponseValidator: api.NoopResponseValidator{},
		Resources: map[string]endpoint.ResourceFunc{
			"userstore": GetUserStore(userStore),
		},
		Timeout: 30 * time.Second,
	})

	serverConfig.Handle(http.MethodPost, login.Path, &login.Login{})
	serverConfig.Handle(http.MethodPost, users.BasePath, &users.Create{})
	serverConfig.Handle(http.MethodGet, users.ResourcePath, &users.Lookup{})

	server := spiderweb.NewServer(serverConfig)

	server.Listen()
}

func GetUserStore(userStore db.UserStore) endpoint.ResourceFunc {
	return func() interface{} {
		return userStore
	}
}
