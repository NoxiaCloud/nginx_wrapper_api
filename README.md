# node-agent

A lightweight REST API built in Rust designed to manage server operations for [Noxia](https://noxia.cloud)

## ⚙️ Features

- [NGINX](https://nginx.org) configuration management
- [Docker](https://docker.io) container deployment
- Automatic SSL certificates via [Let’s Encrypt](https://letsencrypt.org/)
- Subdomain creation via [Cloudflare](https://cloudflare.com/) REST API
- GitHub integration; deploy repositories to containers

### 🧩 Environment Variables

| Variable   | Description                                    |
| ---------- | ---------------------------------------------- |
| API_KEY    | Your secret API key                            |
| HOST       | Server host                                    |
| PORT       | Server port                                    |
| WORKERS    | Number of worker threads                       |
| LOG_DIR    | Directory for storing logs; defaults to ./logs |
| CF_ZONE_ID | Cloudflare Zone ID                             |
| CF_API_KEY | Cloudflare API Key                             |
