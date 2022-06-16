package main

import (
  "bytes"
  "html/template"
  "io"
  "log"
  "net/http"
)

var indexBuf, privacyBuf bytes.Buffer

func init() {
  tmpl := template.Must(template.ParseGlob("views/*.html"))

  mustExecuteTemplate(tmpl, &indexBuf, "index.html")
  mustExecuteTemplate(tmpl, &privacyBuf, "privacy.html")
}

func mustExecuteTemplate(tmpl *template.Template, w io.Writer, name string) {
  err := tmpl.ExecuteTemplate(w, name, nil)
  if err != nil {
    panic(err)
  }
}

func writeResponse(w http.ResponseWriter, buf *bytes.Buffer) {
  _, err := w.Write(buf.Bytes())
  if err != nil {
    log.Println(err)
  }
}

func main() {
  http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
    switch r.URL.Path {
      case "/":
        writeResponse(w, &indexBuf)
      case "/privacy":
        writeResponse(w, &privacyBuf)
      case "/favicon.ico":
        http.NotFound(w, r)
      default:
        http.Redirect(w, r, "/", http.StatusFound)
    }
  })

  log.Println("Listening.")
  log.Fatalln(http.ListenAndServeTLS(":443", "cert.pem", "key.pem", nil))
}
