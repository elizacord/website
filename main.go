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
    if r.URL.Path != "/" {
      http.Redirect(w, r, "/", http.StatusFound)
      return
    }

    _, err := w.Write(buf)
    if err != nil {
      fmt.Println(err)
    }
  })

  fmt.Println("Listening.")
  fmt.Println(http.ListenAndServeTLS(":443", "cert.pem", "key.pem", nil))
}
