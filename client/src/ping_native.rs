use std::net::IpAddr;
use std::sync::Arc;
use std::time::Duration;

use rand::random;
use surge_ping::{Client, IcmpPacket, PingIdentifier, PingSequence};
use tokio::sync::Semaphore;
use log::debug;

use crate::BimClient;
use crate::models::PingData;

pub async fn ping_native(
    v4_client: &Arc<Client>,
    v6_client: &Arc<Client>,
    target_ip: IpAddr,
    ipv6: bool,
    _semaphore: Arc<Semaphore>,
    target_id: i32,
    cc: Arc<BimClient>,
) -> Option<PingData> {
    let client = if ipv6 { v6_client } else { v4_client };
    let ident = PingIdentifier(random());
    let mut pinger = client.pinger(target_ip, ident).await;
    pinger.timeout(Duration::from_secs(2));

    let mut times = Vec::new();
    
    for seq in 0..20 {
        match pinger.ping(PingSequence(seq), &[]).await {
            Ok((packet, duration)) => {
                // 验证返回的包类型是否匹配请求的 IP 类型
                let is_ipv6 = matches!(packet, IcmpPacket::V6(_));
                if is_ipv6 == ipv6 {
                    times.push(duration.as_millis() as u16);
                }
            }
            Err(e) => {
                debug!("Ping {} seq {} failed: {}", target_ip, seq, e);
            }
        }
        
        // 间隔 100ms，避免发送过快
        if seq < 19 {
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }

    if times.is_empty() {
        debug!("All pings to {} failed", target_ip);
        return None;
    }

    let ping_min = times.iter().min().copied().unwrap_or(0);
    let ping_avg = times.iter().sum::<u16>() / times.len() as u16;
    let ping_fail = 20 - times.len() as u8;

    let data = PingData {
        ipv6,
        min: ping_min,
        avg: ping_avg,
        fail: ping_fail,
    };

    cc.post_target_data(target_id, data.clone()).await;
    
    Some(data)
}
