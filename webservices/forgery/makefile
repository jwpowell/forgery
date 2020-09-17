CURRENT_DIR = $(shell pwd)

build:
	go build -o bin/forgeryweb main.go

test: build 
	GOBIN=$(CURRENT_DIR)/bin go get gotest.tools/gotestsum
	-$(CURRENT_DIR)/bin/gotestsum --format dots -- -count=1 -parallel 8 -race -v ./...