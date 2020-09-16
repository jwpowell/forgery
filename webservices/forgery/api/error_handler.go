package api

import (
	"encoding/json"
	"net/http"

	"github.com/wspowell/spiderweb/endpoint"
	"github.com/wspowell/spiderweb/errors"
)

type ErrorWithCodes struct {
	Code         string
	InternalCode string
	Message      string
}

func NewInternalServerError(internalCode string) error {
	return NewErrorWithCodes(ErrorCodeInternalServiceError, internalCode, ErrorMessageInternalServerError)
}

func NewErrorWithCodes(code string, internalCode string, message string) error {
	return errors.Wrap(internalCode, ErrorWithCodes{
		Code:         code,
		InternalCode: internalCode,
		Message:      message,
	})
}

func (self ErrorWithCodes) Error() string {
	return self.Message
}

var _ endpoint.ErrorHandler = (*ErrorJsonWithCodeResponse)(nil)

type ErrorJsonWithCodeResponse struct {
	Code         string `json:"code"`
	InternalCode string `json:"internal_code"`
	Message      string `json:"message"`
}

func (self ErrorJsonWithCodeResponse) MimeType() string {
	return endpoint.MimeTypeJson
}

func (self ErrorJsonWithCodeResponse) HandleError(ctx *endpoint.Context, httpStatus int, err error) (int, []byte) {
	var errorBytes []byte
	var responseErr error

	if endpoint.HasFrameworkError(err) {
		if errors.Is(err, endpoint.ErrorRequestTimeout) {
			return httpStatus, nil
		} else if httpStatus == http.StatusBadRequest {
			errorBytes, responseErr = json.Marshal(ErrorWithCodes{
				Code:         ErrorCodeBadRequest,
				InternalCode: InternalCodeBadRequest,
				Message:      ErrorMessageBadRequest,
			})
		} else {
			errorBytes, responseErr = json.Marshal(ErrorWithCodes{
				Code:         ErrorCodeInternalServiceError,
				InternalCode: InternalCodeFrameworkError,
				Message:      ErrorMessageInternalServerError,
			})
		}
	} else {
		var apiErr ErrorWithCodes
		if errors.As(err, &apiErr) {
			errorBytes, responseErr = json.Marshal(apiErr)
		} else {
			// Catch anything not using ErrorWithCodes.
			errorBytes, responseErr = json.Marshal(ErrorJsonWithCodeResponse{
				Code:         ErrorCodeInternalServiceError,
				InternalCode: errors.InternalCode(err),
				Message:      err.Error(),
			})
		}
	}

	if responseErr != nil {
		// Provide a valid default for responding.
		httpStatus = http.StatusInternalServerError
		errorBytes = []byte(`{"code":"` + ErrorCodeInternalServiceError + `","internal_code":"` + InternalCodeErrorMarshalFailure + `","message":"` + ErrorMessageInternalServerError + `"}`)
	}

	return httpStatus, errorBytes
}
