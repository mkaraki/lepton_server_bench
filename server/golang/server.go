package main

import (
	"fmt"
	"log"
	"net/http"
	"os"
	"github.com/mkaraki/lepton_server_bench/server/golang/lepton_jpeg"
)

func httpHandler(w http.ResponseWriter, r *http.Request) {
	image := r.URL.Query().Get("i")

	fp, err := os.Open("images/" + image)
	if err != nil {
		panic(err)
	}
	defer fp.Close()

	w.Header().Set("Content-Type", "image/jpeg")
	lepton_jpeg.DecodeLepton(w, fp)
}

func main() {
	http.HandleFunc("/", httpHandler)

	fmt.Println("Starting server")
	err := http.ListenAndServe(":8080", nil)
	if err != nil {
		log.Fatal(err)
	}
}