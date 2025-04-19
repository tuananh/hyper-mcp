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

### Prerequisites
- Google Cloud SDK installed
- Terraform installed
- A GCP project with Cloud Run and Secret Manager APIs enabled

### Configuration

1. Create a `terraform.tfvars` file with your configuration in `iac` folder:

```hcl
name       = "hyper-mcp"
project_id = "your-project-id"
region     = "asia-southeast1"  # or your preferred region
```

2. Create a config file in Secret Manager:

The config file will be automatically created and managed by Terraform. Here's an example of what it contains:

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

3. Deploy using Terraform:

```sh
cd iac
terraform init
terraform plan
terraform apply
```

The service will be deployed with:
- Port 3001 exposed
- Config file mounted at `/app/config.json`
- Public access enabled
- SSE transport mode
- Bound to 0.0.0.0:3001

### Accessing the Service

After deployment, you can get the service URL using:

```sh
terraform output url
```

The service will be accessible at the provided URL.

## Cloudflare Workers

Not possible yet but it's in my TODO list.