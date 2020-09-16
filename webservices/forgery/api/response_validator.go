package api

import (
	"net/http"

	"github.com/wspowell/spiderweb/endpoint"
)

type NoopResponseValidator struct{}

func (self NoopResponseValidator) ValidateResponse(ctx *endpoint.Context, httpStatus int, responseBody []byte) (int, error) {
	return http.StatusOK, nil
}
