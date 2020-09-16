package users

import (
	"net/http"

	"forgery/api"
	"forgery/db"

	"github.com/wspowell/spiderweb/endpoint"
)

const (
	ResourcePath = "/v1/users/{user_guid}"
)

type lookupResponse struct {
	UserGuid string `json:"user_guid"`
	Username string `json:"username"`
}

type Lookup struct {
	UserGuid     string         `spiderweb:"path=user_guid"`
	UserStore    db.UserStore   `spiderweb:"resource=userstore"`
	ResponseBody lookupResponse `spiderweb:"response,mime=json"`
}

func (self *Lookup) Handle(ctx *endpoint.Context) (int, error) {
	if user, err := self.UserStore.GetUserByGuid(ctx, self.UserGuid); err != nil {
		return http.StatusInternalServerError, api.NewInternalServerError(api.InternalCodeUsersCreateDbFailure)
	} else if user == nil {
		return http.StatusNotFound, api.NewErrorWithCodes(api.ErrorCodeUserNotFound, api.InternalCodeUsersLookupNotFound, api.ErrorMessageUserNotFound)
	} else {
		self.ResponseBody.UserGuid = user.Guid
		self.ResponseBody.Username = user.Username
	}

	return http.StatusOK, nil
}
