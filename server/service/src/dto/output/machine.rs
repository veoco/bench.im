use entity::machine::Model as MachineModel;
use serde::{Deserialize, Serialize};

/// 机器响应（公开，IP 已脱敏）
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MachineResponse {
    pub id: i32,
    pub name: String,
    pub ip: String,
    pub created: u64,
    pub updated: Option<u64>,
}

impl From<MachineModel> for MachineResponse {
    fn from(m: MachineModel) -> Self {
        let ipv4 = m.ip.contains(".");
        let ip = if ipv4 {
            let parts: Vec<&str> = m.ip.split(".").collect();
            format!("{}.{}.*.*", parts[0], parts[1])
        } else {
            let parts: Vec<&str> = m.ip.split(":").collect();
            let prefix = match parts.len() {
                n if n > 4 => parts[..(n - 4)].join(":"),
                n if n > 1 => parts[..(n - 1)].join(":"),
                _ => m.ip.to_string(),
            };
            format!("{}::*", prefix)
        };

        Self {
            id: m.id,
            name: m.name,
            ip,
            created: m.created.and_utc().timestamp() as u64,
            updated: m.updated.map(|dt| dt.and_utc().timestamp() as u64),
        }
    }
}

/// 机器列表项（精简版，用于侧边栏）
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MachineListItem {
    pub id: i32,
    pub name: String,
    pub updated: i64,
}

impl From<MachineModel> for MachineListItem {
    fn from(m: MachineModel) -> Self {
        Self {
            id: m.id,
            name: m.name,
            updated: m.updated.map(|dt| dt.and_utc().timestamp()).unwrap_or(0),
        }
    }
}

/// 机器详情（包含目标列表）
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MachineWithTargets {
    pub id: i32,
    pub name: String,
    pub ip: String,
    pub created: u64,
    pub targets: Vec<crate::dto::output::target::TargetResponse>,
}

impl From<MachineResponse> for MachineWithTargets {
    fn from(m: MachineResponse) -> Self {
        Self {
            id: m.id,
            name: m.name,
            ip: m.ip,
            created: m.created,
            targets: vec![],
        }
    }
}
