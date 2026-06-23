package main

import (
	"net/http"

	"agents-app/backend/internal/server"
)

func main() {
	if err := http.ListenAndServe(":8080", server.NewRouter()); err != nil {
		panic(err)
	}
}
