# nginx_wrapper_api
REST API written in Rust designed to manage NGINX safely and programmatically, including SSL support via Let's Encrypt.

<sub>ℹ️ This API currently only supports Linux systems where NGINX is managed by `systemd`</sub>  
<sub>⚠️ Make sure to host this behind HTTPS in production (usually behind a reverse proxy like NGINX or Caddy)</sub>

Build (production): `cargo run build`  
Development (local server): `cargo run dev`

Example usage: `curl -H "Authorization: Bearer <API_KEY>" http://127.0.0.1:8080/service/status`

Dependencies:
```
actix-web = "4.11.0"
actix-web-httpauth = "0.8.2"
dotenvy = "0.15.7"
num_cpus = "1.17.0"
serde_json = "1.0"
```

Example configuration (.env):

```
API_KEY="Your secret API key"
HOST="127.0.0.1"
PORT="8080"
WORKERS=""
```
<sub>⚠️ Never expose your real API key</sub>

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
