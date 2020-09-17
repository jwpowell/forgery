package main

import (
	"forgery/config"

	"github.com/wspowell/spiderweb"
	"github.com/wspowell/spiderweb/logging"
)

const (
	host = "localhost"
	port = 8080

	logLevel = logging.LevelDebug
)

func main() {
	spiderweb.NewServer(config.NewServerConfig(host, port, logLevel)).Listen()
}
