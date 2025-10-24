# nginx_wrapper_api
REST API written in Rust designed to manage NGINX safely and programmatically, including SSL support via Let's Encrypt.

<sub>ℹ️ This API currently only supports Linux systems where NGINX is managed by `systemd`</sub>  
<sub>⚠️ Make sure to host this behind HTTPS in production (usually behind a reverse proxy like NGINX or Caddy)</sub>

Build (production): `cargo run build`  
Development (local server): `cargo run dev`

Example usage: `curl -H "Authorization: Bearer password123" http://127.0.0.1:8080/test`

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