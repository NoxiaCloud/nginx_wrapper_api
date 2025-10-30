use actix_web::{ post, get, HttpResponse, Responder, web };
use serde_json::Value;
use std::{ process::Command, fs, collections::HashMap };
use tokio::time::sleep;
use sysinfo::System;

#[get("/system/metrics/cpu")]
pub async fn cpu_info() -> impl Responder {
    let content = match fs::read_to_string("/proc/cpuinfo") {
        Ok(c) => c,
        Err(_) => {
            return HttpResponse::InternalServerError().json(
                serde_json::json!({"message": "Failed to read /proc/cpuinfo"})
            );
        }
    };

    let mut physical: HashMap<String, serde_json::Value> = HashMap::new();
    let mut cpu_block: HashMap<&str, &str> = HashMap::new();

    let mut sys = System::new_all();
    sys.refresh_cpu_all();

    for line in content.lines().chain([""].iter().cloned()) {
        if line.trim().is_empty() {
            if let Some(phys_id) = cpu_block.get("physical id") {
                physical
                    .entry(phys_id.to_string())
                    .or_insert_with(
                        ||
                            serde_json::json!({
                                "model": cpu_block.get("model name").unwrap_or(&"Unknown"),
                                "cpu_count": sys.cpus().len(),
                                "vendor_id": cpu_block.get("vendor_id").unwrap_or(&"Unknown"),
                                "frequency_mhz": cpu_block.get("cpu MHz").unwrap_or(&"0").parse::<f64>().unwrap_or(0.0)
                            })
                    );
            }
            cpu_block.clear();
        } else if let Some((k, v)) = line.split_once(':') {
            cpu_block.insert(k.trim(), v.trim());
        }
    }

    HttpResponse::Ok().json(
        serde_json::json!({
        "message": "CPU information fetched successfully",
        "physical_cpu_count": physical.len(),
        "cpus": physical.values().cloned().collect::<Vec<_>>()
    })
    )
}

#[get("/system/metrics/cpu/usage")]
pub async fn cpu_usage() -> impl Responder {
    let mut sys = System::new_all();
    sys.refresh_cpu_usage();
    sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL).await;
    sys.refresh_cpu_usage();
    let usage: Vec<f32> = sys
        .cpus()
        .iter()
        .map(|cpu| cpu.cpu_usage())
        .collect();

    HttpResponse::Ok().json(
        serde_json::json!({
        "message": "CPU usage fetched successfully",
        "usage": usage
    })
    )
}

#[post("/system/metrics/speedtest")]
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
            "message": "Failed to execute speedtest",
            "error": e.to_string()
        })
            );
        }
    };

    if !output.status.success() {
        return HttpResponse::InternalServerError().json(
            serde_json::json!({
            "message": "Speedtest command failed",
            "stderr": String::from_utf8_lossy(&output.stderr)
        })
        );
    }

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();

    HttpResponse::Ok().json(
        serde_json::json!({
        "message": "Speedtest completed",
        "output": stdout
    })
    )
}

pub fn cfg_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(speedtest).service(cpu_info).service(cpu_usage);
}
