use entity::machine::Model as MachineModel;
use serde::{Deserialize, Serialize};

/// 机器（完整数据）
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Machine {
    pub id: i32,
    pub name: String,
    pub ip: String,
    pub key: String,
    pub created: i64,
    pub updated: Option<i64>,
}

impl Machine {
    fn mask_ip(ip: &str) -> String {
        if ip.contains('.') {
            let parts: Vec<&str> = ip.split('.').collect();
            format!("{}.{}.*.*", parts[0], parts[1])
        } else {
            let parts: Vec<&str> = ip.split(':').collect();
            let prefix = match parts.len() {
                n if n > 4 => parts[..(n - 4)].join(":"),
                n if n > 1 => parts[..(n - 1)].join(":"),
                _ => ip.to_string(),
            };
            format!("{}::*", prefix)
        }
    }
}

impl From<Machine> for MaskedMachine {
    fn from(m: Machine) -> Self {
        Self {
            id: m.id,
            name: m.name,
            ip: Machine::mask_ip(&m.ip),
            created: m.created,
            updated: m.updated,
        }
    }
}

impl From<Machine> for MachineListItem {
    fn from(m: Machine) -> Self {
        Self {
            id: m.id,
            name: m.name,
            updated: m.updated.unwrap_or(0),
        }
    }
}

impl From<MachineModel> for Machine {
    fn from(m: MachineModel) -> Self {
        Self {
            id: m.id,
            name: m.name,
            ip: m.ip,
            key: m.key,
            created: m.created.and_utc().timestamp(),
            updated: m.updated.map(|dt| dt.and_utc().timestamp()),
        }
    }
}

impl From<MachineModel> for MaskedMachine {
    fn from(m: MachineModel) -> Self {
        Machine::from(m).into()
    }
}

impl From<MachineModel> for MachineListItem {
    fn from(m: MachineModel) -> Self {
        Machine::from(m).into()
    }
}

/// 机器（脱敏版本，公开 API 使用）
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MaskedMachine {
    pub id: i32,
    pub name: String,
    pub ip: String,
    pub created: i64,
    pub updated: Option<i64>,
}

/// 机器列表项（精简版，用于侧边栏）
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MachineListItem {
    pub id: i32,
    pub name: String,
    pub updated: i64,
}

/// 机器详情（包含目标列表）
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MachineDetail {
    #[serde(flatten)]
    pub machine: MaskedMachine,
    pub targets: Vec<super::target::Target>,
}

impl MachineDetail {
    pub fn new(machine: &Machine, targets: Vec<super::target::Target>) -> Self {
        Self {
            machine: machine.clone().into(),
            targets,
        }
    }
}

/// 客户端认证信息
#[derive(Clone, Debug)]
pub struct ClientAuthInfo {
    pub id: i32,
    pub name: String,
}
