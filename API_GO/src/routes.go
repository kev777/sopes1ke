package src

import (
	//"main/bd"
	"encoding/json"
	"github.com/gorilla/mux"
	"net/http"
	"fmt"
)

func SetupRoutesForTweets(router *mux.Router) {

	router.HandleFunc("/", func(res http.ResponseWriter, req *http.Request) {
		fmt.Fprint(res, "Api en Go")
	})

	
	router.HandleFunc("/carga", func(w http.ResponseWriter,r *http.Request){
		var tweet Tweet
		err := json.NewDecoder(r.Body).Decode(&tweet)
		if err != nil {
			respondWithError(err, w)
		} else {
			err := createTweet(tweet)
			if err != nil {
				respondWithError(err, w)
			} else {
				respondWithSuccess("Se ha publicado un nuevo tweet", w)
			}
		}
	}).Methods(http.MethodPost)

}

// Helper functions for respond with 200 or 500 code
func respondWithError(err error, w http.ResponseWriter) {
	w.WriteHeader(http.StatusInternalServerError)
	json.NewEncoder(w).Encode(err.Error())
}

func respondWithSuccess(data interface{}, w http.ResponseWriter) {

	w.WriteHeader(http.StatusOK)
	json.NewEncoder(w).Encode(data)
}
