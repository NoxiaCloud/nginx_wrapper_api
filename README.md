# node-agent

REST API written in Rust designed to manage server operations for Noxia, including container deployment, NGINX configuration, SSL certificate handling, and other system-level tasks.

### Environment Variables

| Variable   | Description              |
| ---------- | ------------------------ |
| API_KEY    | Your secret API key      |
| HOST       | Server host              |
| PORT       | Server port              |
| WORKERS    | Number of worker threads |
| CF_ZONE_ID | Cloudflare Zone ID       |
| CF_API_KEY | Cloudflare API Key       |