use std::env;
use std::sync::Arc;
use std::time::Duration;

use getopts::Options;
use log::{debug, info};
use tokio::sync::Semaphore;
use tokio::time;

use bim::{BimClient, PingMode, resolve_domain};

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("m", "mid", "set test client machine id", "MACHINE_ID");
    opts.optopt("t", "token", "set the token", "TOKEN");
    opts.optopt("s", "server_url", "set the server URL", "SERVER_URL");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            print!("{}\n", f.to_string());
            print_usage(&program, opts);
            return;
        }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let mid = match matches.opt_str("m") {
        Some(mid) => {
            if let Ok(m) = mid.parse::<i32>() {
                m
            } else {
                print!("Invalid ID: {}\n", mid);
                return;
            }
        }
        None => {
            print_usage(&program, opts);
            return;
        }
    };

    let token = match matches.opt_str("t") {
        Some(t) => t,
        None => {
            print_usage(&program, opts);
            return;
        }
    };

    let server_url = match matches.opt_str("s") {
        Some(s) => s,
        None => {
            print_usage(&program, opts);
            return;
        }
    };

    env_logger::init();
    debug!("API Token: {token}");
    info!("Running Machine: {mid}");

    run(mid, token, server_url);
}

#[tokio::main]
async fn run(mid: i32, token: String, server_url: String) {
    // 启动时检测 ping 模式
    info!("Detecting ping mode...");
    let ping_mode = PingMode::detect().await;
    match &ping_mode {
        PingMode::Native { .. } => info!("Using native ping (surge-ping)"),
        PingMode::System => info!("Using system ping fallback"),
    }

    let mut interval = time::interval(Duration::from_secs(300));
    let semaphore = Arc::new(Semaphore::new(64));

    loop {
        info!("Waiting for next tick");
        interval.tick().await;

        let c = match BimClient::new(mid, token.clone(), server_url.clone()).await {
            Ok(c) => Arc::new(c),
            Err(e) => {
                info!("Connect failed: {e}");
                continue;
            }
        };

        let targets = match c.get_targets().await {
            Ok(t) => t,
            Err(e) => {
                info!("Get targets failed: {e}");
                continue;
            }
        };

        let count = targets.len();
        info!("Testing {count} targets");

        let mut tasks = vec![];

        for target in targets {
            let target_id = target.id;
            
            // 域名解析，获取 IPv4 和 IPv6 地址
            let (v4_addr, v6_addr) = if let Some(domain) = &target.domain {
                match resolve_domain(domain).await {
                    Ok((v4, v6)) => (v4, v6),
                    Err(e) => {
                        debug!("Failed to resolve domain {}: {}", domain, e);
                        (None, None)
                    }
                }
            } else {
                // 解析已有的 IP 字符串
                let v4 = target.ipv4.and_then(|s| s.parse().ok());
                let v6 = target.ipv6.and_then(|s| s.parse().ok());
                (v4, v6)
            };

            // 创建 IPv4 任务
            if let Some(addr) = v4_addr {
                let pm = ping_mode.clone();
                let sem = semaphore.clone();
                let cc = c.clone();
                let task = tokio::spawn(async move {
                    pm.ping(addr, false, sem, target_id, cc).await;
                });
                tasks.push(task);
            }

            // 创建 IPv6 任务
            if let Some(addr) = v6_addr {
                let pm = ping_mode.clone();
                let sem = semaphore.clone();
                let cc = c.clone();
                let task = tokio::spawn(async move {
                    pm.ping(addr, true, sem, target_id, cc).await;
                });
                tasks.push(task);
            }
        }

        let task_count = tasks.len();
        info!("Waiting for {task_count} tasks to finish");
        for t in tasks {
            let _ = t.await;
        }

        info!("Finished {task_count} tasks")
    }
}
