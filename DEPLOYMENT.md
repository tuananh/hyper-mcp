Deployment
==========

## Docker

Assume you have Docker installed.

Pull the image

```sh
docker pull ghcr.io/tuananh/hyper-mcp:latest
```

Create a sample config file like this, assume at `/home/ubuntu/config.yml`

```json
{
  "plugins": [
    {
      "name": "time",
      "path": "oci://ghcr.io/tuananh/time-plugin:latest"
    },
    {
      "name": "qr-code",
      "path": "oci://ghcr.io/tuananh/qrcode-plugin:latest"
    }
  ]
}
```

Run the container

```sh
docker run -d \
    --name hyper-mcp \
    -p 3001:3001 \
    -v /home/ubuntu/config.yml:/app/config.yml \
    ghcr.io/tuananh/hyper-mcp \
    --transport sse \
    --bind-address 0.0.0.0:3001 \
    --config-file /app/config.yaml
```

Note that we need to bind to `--bind-address 0.0.0.0:3001` in order to access from the host.

## GCP Cloud Run

To be updated

## Cloudflare Workers

Not possible yet but it's in my TODO list.