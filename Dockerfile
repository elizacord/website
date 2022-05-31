FROM golang:1.18
WORKDIR /usr/src/eliza-website
COPY . .
RUN go build main.go
EXPOSE 443
CMD ./main