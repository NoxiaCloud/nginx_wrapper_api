use actix_web::{ get, post, HttpResponse, Responder, web };
use serde_json::Value;
use std::{ fs, process::Command, collections::HashMap };

#[get("/interfaces")]
pub async fn interfaces() -> impl Responder {
    let output = match Command::new("ip").arg("addr").arg("show").output() {
        Ok(out) => out,
        Err(e) => {
            return HttpResponse::InternalServerError().json(
                serde_json::json!({
                    "message": "Failed to execute",
                    "error": e.to_string()
                })
            );
        }
    };

    if !output.status.success() {
        return HttpResponse::InternalServerError().json(
            serde_json::json!({
                "message": "Command failed",
                "stderr": String::from_utf8_lossy(&output.stderr)
            })
        );
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut interfaces = vec![];
    let mut current_iface = serde_json::Map::new();

    for line in stdout.lines() {
        if let Some(colon_pos) = line.find(':') {
            let iface_part = &line[..colon_pos].trim();
            if iface_part.chars().next().unwrap_or('0').is_digit(10) {
                if !current_iface.is_empty() {
                    interfaces.push(Value::Object(current_iface.clone()));
                    current_iface.clear();
                }
                let iface_name = line[colon_pos + 1..]
                    .split_whitespace()
                    .next()
                    .unwrap_or("")
                    .to_string();
                current_iface.insert("name".to_string(), Value::String(iface_name));
            }
        }

        if line.contains("inet ") {
            let parts: Vec<&str> = line.trim().split_whitespace().collect();
            if parts.len() >= 2 {
                let addr = parts[1].to_string();
                current_iface.insert("ipv4".to_string(), Value::String(addr));
            }
        }
        if line.contains("inet6 ") {
            let parts: Vec<&str> = line.trim().split_whitespace().collect();
            if parts.len() >= 2 {
                let addr = parts[1].to_string();
                current_iface.insert("ipv6".to_string(), Value::String(addr));
            }
        }

        if line.contains("link/") {
            let parts: Vec<&str> = line.trim().split_whitespace().collect();
            if parts.len() >= 2 {
                let mac = parts[1].to_string();
                current_iface.insert("mac".to_string(), Value::String(mac));
            }
        }
    }

    if !current_iface.is_empty() {
        interfaces.push(Value::Object(current_iface));
    }

    HttpResponse::Ok().json(
        serde_json::json!({
            "message": "Interfaces fetched successfully",
            "interfaces": interfaces
        })
    )
}

#[post("/traceroute")]
pub async fn traceroute(opts: Option<web::Json<Value>>) -> impl Responder {
    let mut cmd = Command::new("traceroute");

    if let Some(opts) = opts {
        if let Some(args) = opts.get("args").and_then(|v| v.as_array()) {
            for arg in args {
                if let Some(arg_str) = arg.as_str() {
                    cmd.arg(arg_str);
                }
            }
        }
    }

    let output = match cmd.output() {
        Ok(out) => out,
        Err(e) => {
            return HttpResponse::InternalServerError().json(
                serde_json::json!({
            "message": "Failed to execute",
            "error": e.to_string()
        })
            );
        }
    };

    if !output.status.success() {
        return HttpResponse::InternalServerError().json(
            serde_json::json!({
            "message": "Command failed",
            "stderr": String::from_utf8_lossy(&output.stderr)
        })
        );
    }

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();

    HttpResponse::Ok().json(
        serde_json::json!({
        "message": "Traceroute completed successfully",
        "output": stdout
    })
    )
}

#[post("/speedtest")]
pub async fn speedtest(opts: Option<web::Json<Value>>) -> impl Responder {
    let mut cmd = Command::new("speedtest");

    if let Some(opts) = opts {
        if let Some(args) = opts.get("args").and_then(|v| v.as_array()) {
            for arg in args {
                if let Some(arg_str) = arg.as_str() {
                    cmd.arg(arg_str);
                }
            }
        }
    }

    let output = match cmd.output() {
        Ok(out) => out,
        Err(e) => {
            return HttpResponse::InternalServerError().json(
                serde_json::json!({
            "message": "Failed to execute",
            "error": e.to_string()
        })
            );
        }
    };

    if !output.status.success() {
        return HttpResponse::InternalServerError().json(
            serde_json::json!({
            "message": "Command failed",
            "stderr": String::from_utf8_lossy(&output.stderr)
        })
        );
    }

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();

    HttpResponse::Ok().json(
        serde_json::json!({
        "message": "Speedtest completed successfully",
        "output": stdout
    })
    )
}

#[get("/stats")]
pub async fn stats() -> impl Responder {
    let content = match fs::read_to_string("/proc/net/dev") {
        Ok(c) => c,
        Err(e) => {
            return HttpResponse::InternalServerError().json(
                serde_json::json!({"message": "Failed to execute", "error": e.to_string()})
            );
        }
    };

    let mut output: HashMap<String, HashMap<String, u64>> = HashMap::new();

    for line in content.lines().skip(2) {
        if let Some((iface, data)) = line.split_once(':') {
            let fields: Vec<&str> = data.split_whitespace().collect();
            if fields.len() >= 16 {
                let mut iface_stats = HashMap::new();
                iface_stats.insert("receive_bytes".to_string(), fields[0].parse().unwrap_or(0));
                iface_stats.insert("receive_packets".to_string(), fields[1].parse().unwrap_or(0));
                iface_stats.insert("transmit_bytes".to_string(), fields[8].parse().unwrap_or(0));
                iface_stats.insert("transmit_packets".to_string(), fields[9].parse().unwrap_or(0));
                output.insert(iface.trim().to_string(), iface_stats);
            }
        }
    }

    HttpResponse::Ok().json(
        serde_json::json!({
        "message": "Network stats fetched successfully",
        "output": output
    })
    )
}

pub fn cfg_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web
            ::scope("/network")
            .service(stats)
            .service(speedtest)
            .service(traceroute)
            .service(interfaces)
    );
}
