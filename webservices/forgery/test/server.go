package test

import (
	"forgery/config"

	"github.com/wspowell/spiderweb"
	"github.com/wspowell/spiderweb/logging"
)

func NewServer() spiderweb.Server {
	serverConfig := config.NewServerConfig("localhost", 8080, logging.LevelFatal)
	return spiderweb.NewServer(serverConfig)
}
