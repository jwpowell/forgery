package users

import (
	"net/http"

	"forgery/api"
	"forgery/db"

	"github.com/wspowell/spiderweb/endpoint"
)

const (
	BasePath = "/v1/users"
)

type createRequest struct {
	Username string `json:"username"`
	Password string `json:"password"`
}

type createResponse struct {
	UserGuid string `json:"user_guid"`
}

type Create struct {
	UserStore    db.UserStore   `spiderweb:"resource=userstore"`
	RequestBody  createRequest  `spiderweb:"request,mime=json"`
	ResponseBody createResponse `spiderweb:"response,mime=json"`
}

func (self *Create) Handle(ctx *endpoint.Context) (int, error) {
	if user, err := self.UserStore.GetUserByUsername(ctx, self.RequestBody.Username); err != nil {
		return http.StatusInternalServerError, api.NewInternalServerError(api.InternalCodeUsersCreateDbFailure)
	} else if user != nil {
		return http.StatusConflict, api.NewErrorWithCodes(api.ErrorCodeUsernameExists, api.InternalCodeUsersCreateAlreadyExists, api.ErrorMessageUsernameExists)
	}

	newUser := db.User{
		Username: self.RequestBody.Username,
	}

	userGuid, err := self.UserStore.CreateUser(ctx, newUser, self.RequestBody.Password)
	if err != nil {
		return http.StatusInternalServerError, api.NewInternalServerError(api.InternalCodeUsersCreateDbFailure)
	}

	self.ResponseBody.UserGuid = userGuid

	return http.StatusCreated, nil
}
