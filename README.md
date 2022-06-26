# matrix-dnsbot

DNS resolver in matrix. Just invite `@dnsbot:matrix.org` in your 1:1 or group chat.

## Features

- Fully supports encryption
- Support bot-bridges
- Async

## Use with Docker

Simple as

```bash
docker run -d --name dnsbot --restart always \
  -v db:/db \
  -e HOMESERVER=https://matrix.org/ \
  -e USERNAME=dnsbot \
  -e PASSWORD=p4ssW0rD_ \
  --dns 1.1.1.1 \
  --cpus 0.5 -m 100m \
  cofob/dnsbot
```
