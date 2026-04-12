use entity::target::Model as TargetModel;
use serde::{Deserialize, Serialize};

/// 目标（完整数据）
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Target {
    pub id: i32,
    pub name: String,
    pub domain: Option<String>,
    pub ipv4: Option<String>,
    pub ipv6: Option<String>,
    pub created: i64,
    pub updated: Option<i64>,
}

impl From<Target> for TargetPublic {
    fn from(t: Target) -> Self {
        Self {
            id: t.id,
            name: t.name,
            created: t.created,
            updated: t.updated,
        }
    }
}

impl From<TargetModel> for Target {
    fn from(t: TargetModel) -> Self {
        Self {
            id: t.id,
            name: t.name,
            domain: t.domain,
            ipv4: t.ipv4,
            ipv6: t.ipv6,
            created: t.created.and_utc().timestamp(),
            updated: t.updated.map(|dt| dt.and_utc().timestamp()),
        }
    }
}

impl From<TargetModel> for TargetPublic {
    fn from(t: TargetModel) -> Self {
        Target::from(t).into()
    }
}

/// 目标（公开版本，仅基础信息）
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TargetPublic {
    pub id: i32,
    pub name: String,
    pub created: i64,
    pub updated: Option<i64>,
}
