use entity::ping::Model as PingModel;
use serde::{Deserialize, Serialize};

/// Ping 记录响应
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PingResponse {
    pub timestamp: i64,
    pub min: i32,
    pub avg: i32,
    pub fail: i32,
}

impl From<PingModel> for PingResponse {
    fn from(p: PingModel) -> Self {
        Self {
            timestamp: p.created.and_utc().timestamp(),
            min: p.min,
            avg: p.avg,
            fail: p.fail,
        }
    }
}

impl PingResponse {
    /// 转换为图表数据格式
    pub fn to_data(&self) -> PingData {
        (self.timestamp, self.min, self.avg, self.fail)
    }
}

/// 简化版 ping 数据（用于图表）
pub type PingData = (i64, i32, i32, i32);
