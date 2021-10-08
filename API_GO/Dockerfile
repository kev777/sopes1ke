FROM golang:alpine as builder
WORKDIR /app
COPY go.mod go.sum ./
RUN go mod download 
COPY . .
RUN CGO_ENABLED=0 GOOS=linux go build -a -installsuffix cgo -o main .
FROM alpine:latest
RUN apk --no-cache add tzdata
WORKDIR /root/
COPY --from=builder /app/main .
COPY --from=builder /app/.env .   
EXPOSE 3040
CMD ["./main"]