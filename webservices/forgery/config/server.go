package config

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

func NewServerConfig(host string, port int, logLevel logging.Level) *spiderweb.ServerConfig {
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

	return serverConfig
}

func GetUserStore(userStore db.UserStore) endpoint.ResourceFunc {
	return func() interface{} {
		return userStore
	}
}
