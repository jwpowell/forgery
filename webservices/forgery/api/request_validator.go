package api

import (
	"net/http"

	"github.com/wspowell/spiderweb/endpoint"
)

type NoopRequestValidator struct{}

func (self NoopRequestValidator) ValidateRequest(ctx *endpoint.Context, requestBody []byte) (int, error) {
	return http.StatusOK, nil
}
