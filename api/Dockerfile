FROM golang:latest AS build
# builder
WORKDIR /app

COPY go.mod go.sum ./

RUN go mod download 

COPY . .

RUN go build -o api 

EXPOSE 8080

# start PocketBase
CMD [ "./api", "serve", "--http=0.0.0.0:8080" ]
