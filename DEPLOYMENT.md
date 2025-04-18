Deployment
==========

## Docker

Assume you have Docker installed.

Pull the image

```sh
docker pull ghcr.io/tuananh/hyper-mcp:latest
```

Run the container

```sh
docker run -d \
    --name hyper-mcp \
    -p 3001:3001 \
    -v /home/ubuntu/config.yml:/app/config.yml \
    ghcr.io/tuananh/hyper-mcp \
    --config-file /app/config.yaml
```

## GCP Cloud Run

To be updated

## Cloudflare Workers

Not possible yet but it's in my TODO list.