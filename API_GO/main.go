package main

import (
	"log"
	"main/src"
	"net/http"
	"time"

	"github.com/gorilla/mux"
)

func main() {
	//routes
	router := mux.NewRouter()
	src.SetupRoutesForTweets(router)

	port := "0.0.0.0:3040"

	server := &http.Server{
		Handler:      router,
		Addr:         port,
		WriteTimeout: 20 * time.Second,
		ReadTimeout:  20 * time.Second,
	}
	log.Printf("Server started at %s", port)
	log.Fatal(server.ListenAndServe())
}
