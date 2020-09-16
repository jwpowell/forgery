package db

import (
	"github.com/wspowell/spiderweb/local"
	"github.com/wspowell/spiderweb/profiling"
)

type User struct {
	Guid     string
	Username string
}

type UserStore interface {
	CreateUser(ctx local.Context, user User, password string) (string, error)
	DeleteUser(ctx local.Context, userGuid string) error
	GetUserByGuid(ctx local.Context, userGuid string) (*User, error)
	GetUserByUsername(ctx local.Context, username string) (*User, error)
	ValidateCredentials(ctx local.Context, username string, password string) (bool, error)
}

var _ UserStore = (*InMemoryUserStore)(nil)

type InMemoryUserStore struct {
	users     map[string]User
	passwords map[string]string
}

func NewInMemoryUserStore() *InMemoryUserStore {
	return &InMemoryUserStore{
		users:     map[string]User{},
		passwords: map[string]string{},
	}
}

func (self *InMemoryUserStore) CreateUser(ctx local.Context, user User, password string) (string, error) {
	defer profiling.Profile(ctx, "db.CreateUser").Finish()

	user.Guid = "guid-12345" // FIXME: Make this a random guid.
	self.users[user.Guid] = user
	self.passwords[user.Username] = password

	return user.Guid, nil
}

func (self *InMemoryUserStore) DeleteUser(ctx local.Context, userGuid string) error {
	defer profiling.Profile(ctx, "db.DeleteUser").Finish()

	if user, err := self.GetUserByGuid(ctx, userGuid); err != nil {
		return err
	} else if user != nil {
		delete(self.users, userGuid)
		delete(self.passwords, user.Username)
	}
	return nil
}

func (self *InMemoryUserStore) GetUserByGuid(ctx local.Context, userGuid string) (*User, error) {
	defer profiling.Profile(ctx, "db.GetUserByGuid").Finish()

	if user, exists := self.users[userGuid]; exists {
		return &user, nil
	}
	return nil, nil
}

func (self *InMemoryUserStore) GetUserByUsername(ctx local.Context, username string) (*User, error) {
	defer profiling.Profile(ctx, "db.GetUserByUsername").Finish()

	for _, user := range self.users {
		if user.Username == username {
			return &user, nil
		}
	}

	return nil, nil
}

func (self *InMemoryUserStore) ValidateCredentials(ctx local.Context, username string, password string) (bool, error) {
	defer profiling.Profile(ctx, "db.ValidateCredentials").Finish()

	if actualPassword, exists := self.passwords[username]; exists {
		if actualPassword == password {
			return true, nil
		}
	}
	return false, nil
}
