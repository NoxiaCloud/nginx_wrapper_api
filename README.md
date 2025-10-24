# nginx_wrapper_api
REST API written in Rust designed to manage NGINX safely and programmatically, including SSL support via Let's Encrypt.

<small>Make sure to host this behind HTTPS in production (usually behind a reverse proxy like NGINX or Caddy)</small>

Build (production): `cargo run build`  
Development (local server): `cargo run dev`

Usage example: `curl -H "Authorization: Bearer password123" http://127.0.0.1:8080/test`

Dependencies:
```
actix-web = "4.11.0"
actix-web-httpauth = "0.8.2"
dotenvy = "0.15.7"
```

Example configuration (.env):

```
API_KEY="Your secret API key"
HOST="127.0.0.1"
PORT="8080"
```
<small>⚠️ Never expose your real API key</small>