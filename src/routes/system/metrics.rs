use actix_web::{ get, HttpResponse, Responder, web };
use std::{ process::Command, fs, collections::HashMap };
use async_std::task;
use sysinfo::System;

#[get("/memory")]
pub async fn mem_info() -> impl Responder {
    let output = Command::new("sudo")
        .arg("dmidecode")
        .arg("--type")
        .arg("17")
        .output()
        .expect("Failed to execute");

    if !output.status.success() {
        return HttpResponse::InternalServerError().body("Failed to fetch memory information");
    }

    let dmidecode_output = String::from_utf8_lossy(&output.stdout);

    HttpResponse::Ok().json(
        serde_json::json!({
        "message": "Memory information fetched successfully",
        "output": dmidecode_output
    })
    )
}

#[get("/memory/usage")]
pub async fn mem_usage() -> impl Responder {
    let mut sys = System::new();
    sys.refresh_memory();
    HttpResponse::Ok().json(
        serde_json::json!({
        "message": "Memory usage information fetched successfully",
        "usage": format!("{:.2}GiB/{:.2}GiB", (sys.used_memory() as f64) / (1024.0 * 1024.0 * 1024.0), (sys.total_memory() as f64) / (1024.0 * 1024.0 * 1024.0))
    })
    )
}

#[get("/cpu")]
pub async fn cpu_info() -> impl Responder {
    let content = match fs::read_to_string("/proc/cpuinfo") {
        Ok(c) => c,
        Err(_) => {
            return HttpResponse::InternalServerError().json(
                serde_json::json!({"message": "Failed to execute"})
            );
        }
    };

    let mut physical: HashMap<String, serde_json::Value> = HashMap::new();
    let mut cpu_block: HashMap<&str, &str> = HashMap::new();

    let mut sys = System::new();
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

#[get("/cpu/usage")]
pub async fn cpu_usage() -> impl Responder {
    let mut sys = System::new();
    sys.refresh_cpu_usage();
    task::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL).await;
    sys.refresh_cpu_usage();
    let usage: Vec<f32> = sys
        .cpus()
        .iter()
        .map(|cpu| cpu.cpu_usage())
        .collect();

    HttpResponse::Ok().json(
        serde_json::json!({
        "message": "CPU usage information fetched successfully",
        "usage": usage
    })
    )
}

pub fn cfg_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web
            ::scope("/metrics")
            .service(cpu_info)
            .service(cpu_usage)
            .service(mem_info)
            .service(mem_usage)
    );
}
