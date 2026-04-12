use entity::target::Model as TargetModel;
use serde::{Deserialize, Serialize};

/// 目标响应
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TargetResponse {
    pub id: i32,
    pub name: String,
    pub created: u64,
    pub updated: Option<u64>,
}

impl From<TargetModel> for TargetResponse {
    fn from(t: TargetModel) -> Self {
        Self {
            id: t.id,
            name: t.name,
            created: t.created.and_utc().timestamp() as u64,
            updated: t.updated.map(|dt| dt.and_utc().timestamp() as u64),
        }
    }
}

/// 目标详情响应（管理员）
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TargetDetailResponse {
    pub id: i32,
    pub name: String,
    pub domain: Option<String>,
    pub ipv4: Option<String>,
    pub ipv6: Option<String>,
    pub created: u64,
    pub updated: Option<u64>,
}

impl From<TargetModel> for TargetDetailResponse {
    fn from(t: TargetModel) -> Self {
        Self {
            id: t.id,
            name: t.name,
            domain: t.domain,
            ipv4: t.ipv4,
            ipv6: t.ipv6,
            created: t.created.and_utc().timestamp() as u64,
            updated: t.updated.map(|dt| dt.and_utc().timestamp() as u64),
        }
    }
}
