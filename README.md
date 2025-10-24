# nginx_wrapper_api
REST API written in Rust designed to manage NGINX safely and programmatically, including SSL support via Let's Encrypt.

<sub>ℹ️ This project currently only supports Linux systems where NGINX is managed by `systemd`</sub>  
<sub>⚠️ Make sure to use HTTPS in production (usually behind a reverse proxy like NGINX or Caddy)</sub>

Build (production): `cargo build --release`  
Development (local server): `cargo run`

Example usage: `curl -H "Authorization: Bearer <API_KEY>" http://127.0.0.1:8080/service/status`

Example configuration (.env):

| Variable  | Description             |
|-----------|-------------------------|
| API_KEY   | Your secret API key     |
| HOST      | Server host             |
| PORT      | Server port             |
| WORKERS   | Number of worker threads|

<sub>⚠️ Never expose your real API key</sub>

### Prerequisites:
- systemd-enabled Linux system
- NGINX
- Certbot

### API Documentation
#### Service:

/service/start `[POST]`
- Start NGINX
- Success: `{"message": "Successfully started NGINX"}`
- Error: `{"message": "Failed to start NGINX", "error": "<error>"}`

/service/stop `[POST]`
- Stop NGINX
- Success: `{"message": "Successfully stopped NGINX"}`
- Error: `{"message": "Failed to stop NGINX", "error": "<error>"}`

/service/reload `[POST]`
- Reload NGINX
- Success: `{"message": "Successfully reloaded NGINX"}`
- Error: `{"message": "Failed to reload NGINX", "error": "<error>"}`

/service/restart `[POST]`
- Restart NGINX
- Success: `{"message": "Successfully restarted NGINX"}`
- Error: `{"message": "Failed to restart NGINX", "error": "<error>"}`

/service/status `[GET]`
- Fetch NGINX status
- Success: `{"message": "NGINX status fetched successfully", "status": "<output>"}`
- Error: `{"message": "Failed to fetch NGINX status", "error": "<error>"}`
