package login

import (
	"net/http"

	"github.com/wspowell/spiderweb/endpoint"
)

const (
	Path = "/v1/login"
)

type Login struct {
}

func (self *Login) Handle(ctx *endpoint.Context) (int, error) {
	return http.StatusOK, nil
}
