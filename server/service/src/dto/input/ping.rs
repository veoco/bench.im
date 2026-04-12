use serde::Deserialize;
use validator::Validate;

fn validate_ping(ping: &CreatePingRequest) -> Result<(), validator::ValidationError> {
    if ping.min > ping.avg {
        return Err(validator::ValidationError::new("min_cannot_exceed_avg"));
    }
    Ok(())
}

/// 创建 Ping 记录请求（客户端）
#[derive(Debug, Validate, Deserialize)]
#[validate(schema(function = "validate_ping"))]
pub struct CreatePingRequest {
    pub ipv6: bool,
    #[validate(range(min = 1, max = 1000))]
    pub min: u16,
    #[validate(range(min = 1, max = 1000))]
    pub avg: u16,
    #[validate(range(max = 20))]
    pub fail: u8,
}

/// Ping 查询过滤条件
#[derive(Debug, Deserialize)]
pub struct PingFilter {
    pub ipv6: Option<bool>,
}
