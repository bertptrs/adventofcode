BINARY := aoc2026

GOFLAGS =

.PHONY: all clean test $(BINARY)

all: $(BINARY)

$(BINARY): 
	go build $(GOFLAGS) -o $@ main.go

test: $(BINARY)
	go test $(GOFLAGS) ./...
