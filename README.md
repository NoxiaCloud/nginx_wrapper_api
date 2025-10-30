# NGINX Wrapper API

## 🚀 Overview
#### This is a lightweight and secure REST API that allows you to manage and monitor your [NGINX](https://nginx.org/) service safely and programmatically.  
#### It integrates cleanly into Linux environments that use `systemd` and includes [**Let's Encrypt**](https://letsencrypt.org) integration for automated SSL management.

> ⚠️ **Production Recommendation:** Always use HTTPS — preferably behind a reverse proxy such as [**NGINX**](https://nginx.org/) or [**Caddy**](https://caddyserver.com/).

---

## 🧱 Features

- ✅ Start, stop, reload, and restart NGINX via REST endpoints 
- ⚙️ Systemd integration for reliable process control  
- 🌍 SSL automation with [**Certbot**](https://certbot.eff.org/) / [**Let's Encrypt**](https://letsencrypt.org)

---

## ⚙️ Prerequisities


Before you begin, make sure your environment includes:

- A **Linux** system with `systemd`  
- **NGINX** installed and managed by `systemctl`  
- **Certbot** (for SSL certificate automation)  
- **Rust toolchain** (via [`rustup`](https://rustup.rs))  

---

## 🏗️ Build & Run

### Production build

```bash
cargo build --release
```

### Development (local server)

```bash
cargo run
```

---

## 🧩 Configuration

Create a `.env` file in the project root:

| Variable | Description |
|-----------|-------------|
| `API_KEY` | Secret API key used for authentication |
| `HOST` | Host address for the API server |
| `PORT` | Port to run the API server on |
| `WORKERS` | Number of worker threads to spawn |

> ⚠️ **IMPORTANT:** Never expose your real `API_KEY` in a public environment.

Example `.env` file:

```env
API_KEY=supersecretkey123
HOST=127.0.0.1
PORT=8080
WORKERS=4
```

---

## 📡 API Reference

### `/service/start` — **[POST]**
This starts the NGINX service.

**Successful Response:**
```json
{ "message": "Successfully started NGINX" }
```

**Failure Response:**
```json
{ "message": "Failed to start NGINX", "error": "<details>" }
```

---

### `/service/stop` — **[POST]**
This stops the NGINX service.

**Successful Response:**
```json
{ "message": "Successfully stopped NGINX" }
```

**Failure Response:**
```json
{ "message": "Failed to stop NGINX", "error": "<details>" }
```

---

### `/service/reload` — **[POST]**
This reloads the NGINX configuration.

**Successful Response:**
```json
{ "message": "Successfully reloaded NGINX" }
```

**Failure Response:**
```json
{ "message": "Failed to reload NGINX", "error": "<details>" }
```

---

### `/service/restart` — **[POST]**
This restarts the NGINX service.

**Successful Response:**
```json
{ "message": "Successfully restarted NGINX" }
```

**Failure Response:**
```json
{ "message": "Failed to restart NGINX", "error": "<details>" }
```

---

### `/service/status` — **[GET]**
This fetches the current NGINX service status.

**Successful Response:**
```json
{ "message": "NGINX status fetched successfully", "status": "<output>" }
```

**Failure Response:**
```json
{ "message": "Failed to fetch NGINX status", "error": "<details>" }
```

---

## 🔐 Example Usage

Example API call using `curl`:

```bash
curl -H "Authorization: Bearer <API_KEY>"      http://127.0.0.1:8080/service/status
```

---

## 📑 License

Licensed under the **MIT License** — see [LICENSE](LICENSE) for details.