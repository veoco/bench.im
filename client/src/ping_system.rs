use std::net::IpAddr;
use std::sync::Arc;

use regex::Regex;
use tokio::process::Command;
use tokio::sync::Semaphore;
use log::debug;

use crate::BimClient;
use crate::models::PingData;

pub async fn ping_system(
    target_ip: IpAddr,
    ipv6: bool,
    semaphore: Arc<Semaphore>,
    target_id: i32,
    cc: Arc<BimClient>,
) -> Option<PingData> {
    let permit = match semaphore.acquire().await {
        Ok(p) => p,
        Err(_) => {
            debug!("System ping: failed to acquire semaphore");
            return None;
        }
    };

    // Windows 使用 -n，Unix/Linux/macOS 使用 -c
    let count_arg = if cfg!(target_os = "windows") { "-n" } else { "-c" };
    
    let output = Command::new("ping")
        .arg(count_arg)
        .arg("20")
        // 直接使用 IP 地址，不使用 -4/-6 参数
        // macOS 和 Windows 的 ping 根据 IP 类型自动选择协议
        .arg(target_ip.to_string())
        .output()
        .await;

    drop(permit);

    let output = match output {
        Ok(o) => o,
        Err(e) => {
            debug!("System ping command failed: {}", e);
            return None;
        }
    };

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut ping_times = Vec::new();
    let mut ping_success = 0;

    let time_regex = Regex::new(r"=([\d.]+)\s*ms").unwrap();
    
    for line in stdout.lines() {
        if let Some(caps) = time_regex.captures(line) {
            if let Ok(time) = caps[1].parse::<f32>() {
                ping_times.push(time as u16);
                ping_success += 1;
            }
        }
    }

    if ping_success == 0 {
        debug!("System ping: no successful pings to {}", target_ip);
        return None;
    }

    let ping_min = ping_times.iter().min().copied().unwrap_or(0);
    let ping_avg = ping_times.iter().sum::<u16>() / ping_success as u16;
    let ping_fail = 20 - ping_success;

    let data = PingData {
        ipv6,
        min: ping_min,
        avg: ping_avg,
        fail: ping_fail as u8,
    };

    cc.post_target_data(target_id, data.clone()).await;
    
    Some(data)
}
