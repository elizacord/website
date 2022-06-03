package main

import (
  "fmt"
  "net/http"
  "os"
)

func main() {
  buf, err := os.ReadFile("index.html")
  if err != nil {
    fmt.Println(err)
    return
  }

  http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
    switch r.URL.Path {
      case "/":
        _, err := w.Write(buf)
        if err != nil {
          fmt.Println(err)
        }
      case "/favicon.ico":
        http.NotFound(w, r)
      default:
        http.Redirect(w, r, "/", http.StatusFound)
    }
  })

  fmt.Println("Listening.")
  fmt.Println(http.ListenAndServeTLS(":443", "cert.pem", "key.pem", nil))
}
