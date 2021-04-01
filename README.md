# whale-warden

This software keeps your docker containers up to date.

It does NOT do that by "watching out" for any updates in a specific interval, instead it hosts an extremely lightweight HTTP server providing custom webhooks, which you can use with your CI/CD solution (e.g. with Docker Hub).

## Focus

- simplicity (easy to use and only a small program)
- speed (build with Rust and Actix Web)
- no overhead (do not look out for updates in an interval)
  - additionally, the webhook model leads to changes being applied more quickly
- a small docker image

## State

This project is still in early development!

## Usage

### On the host system

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

### In your continuous integration environment

If you are using the default settings, create webhook to the following URL: <br>
`http://youraddress:9090/webhook/update?token=YOURSECRETTOKEN`
