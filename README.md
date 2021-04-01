# whale-warden

This software keeps you docker containers up to date.

It does NOT do that by "watching out" for any updates with a specific interval, instead it hosts an extremely lightweight HTTP server providing you custom webhooks, which you can use with your CI/CD solution (e.g. with Docker Hub).

## State

This project is still in early development!

## Usage

```yaml
services:
  # your custom service here

  # and then add whale-warden
  whale-warden:
    image: bluefireoly/whale-warden:latest
    environment:
      WEBHOOK_PATH: "/website" # default is "/default"
      WEBHOOK_PORT: 9091 # default is "9090"
    ports:
      - "9090:9090"
```
