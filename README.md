# whale-warden

This software keeps you docker containers up to date.

It does NOT do that by "watching out" for any updates with a specific interval, instead it hosts an extremely lightweight HTTP server providing you custom webhooks, which you can use with your CI/CD solution (e.g. with Docker Hub).

## State

This project is still in early development!

## Usage

Example `docker-compose.yml`:

```yaml
services:
  whale-warden:
    image: bluefireoly/whale-warden:latest
    environment:
      WEBHOOK_TOKEN: "YOURSECRETTOKEN" # change this, as the default ist "unset"
    ports:
      - "9090:9090"
    volumes:
      - /var/run/docker.sock:/var/run/docker.sock
```

Additional environment variables:
- `WEBHOOK_PATH` (**default** = "/webhook/update")
    - the path for the webhook
- `WEBHOOK_PORT` (**default** = "9090")
    - the port of the HTTP server
