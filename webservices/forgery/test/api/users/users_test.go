package main

import (
	"net/http"
	"testing"

	"forgery/api/users"
	"forgery/test"

	"github.com/wspowell/spiderweb/spiderwebtest"
)

func Test_Users_Create(t *testing.T) {
	t.Parallel()

	server := test.NewServer()

	// Create a new user.
	spiderwebtest.TestRequest(t, server,
		spiderwebtest.GivenRequest(http.MethodPost, users.BasePath).
			WithRequestBody([]byte(`{"username":"me","password":"password"}`)).
			Expect(http.StatusCreated, []byte(`{"user_guid":"guid-12345"}`)))

	// Username conflict.
	spiderwebtest.TestRequest(t, server,
		spiderwebtest.GivenRequest(http.MethodPost, users.BasePath).
			WithRequestBody([]byte(`{"username":"me","password":"password"}`)).
			Expect(http.StatusConflict, []byte(`{"Code":"USERNAME_ALREADY_EXISTS","InternalCode":"FORGERY-0005","Message":"username already taken"}`)))
}

func Test_Users_Lookup(t *testing.T) {
	t.Parallel()

	server := test.NewServer()

	// Create a new user.
	spiderwebtest.TestRequest(t, server,
		spiderwebtest.GivenRequest(http.MethodPost, users.BasePath).
			WithRequestBody([]byte(`{"username":"you","password":"password"}`)).
			Expect(http.StatusCreated, []byte(`{"user_guid":"guid-12345"}`)))

	// Get user.
	spiderwebtest.TestRequest(t, server,
		spiderwebtest.GivenRequest(http.MethodGet, users.BasePath+"/guid-12345").
			Expect(http.StatusOK, []byte(`{"user_guid":"guid-12345","username":"you"}`)))

	// User not found.
	spiderwebtest.TestRequest(t, server,
		spiderwebtest.GivenRequest(http.MethodGet, users.BasePath+"/not_here").
			Expect(http.StatusNotFound, []byte(`{"Code":"USER_NOT_FOUND","InternalCode":"FORGERY-0004","Message":"user not found"}`)))
}
