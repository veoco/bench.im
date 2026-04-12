use serde::Deserialize;
use validator::{Validate, ValidationError};

fn validate_ip(ip: &str) -> Result<(), ValidationError> {
    if ip.parse::<std::net::Ipv4Addr>().is_ok() || ip.parse::<std::net::Ipv6Addr>().is_ok() {
        Ok(())
    } else {
        Err(ValidationError::new("invalid_ip"))
    }
}

/// 创建机器请求（管理员）
#[derive(Debug, Validate, Deserialize)]
pub struct CreateMachineRequest {
    #[validate(length(min = 1, max = 100, message = "名称不能为空且不能超过100字符"))]
    pub name: String,
    #[validate(length(min = 1, max = 255, message = "IP不能为空"))]
    #[validate(custom(function = "validate_ip", message = "必须是有效的IPv4或IPv6地址"))]
    pub ip: String,
    #[validate(length(min = 1, max = 255, message = "密钥不能为空"))]
    pub key: String,
}

/// 更新机器请求（管理员）
pub type UpdateMachineRequest = CreateMachineRequest;
