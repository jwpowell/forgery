package api

import (
	"net/http"

	"github.com/wspowell/spiderweb/endpoint"
)

type NoopAuth struct{}

func (self NoopAuth) Auth(ctx *endpoint.Context, headers map[string][]byte) (int, error) {
	return http.StatusOK, nil
}
