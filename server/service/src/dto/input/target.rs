use regex::Regex;
use serde::Deserialize;
use std::sync::OnceLock;
use validator::{Validate, ValidationError};

fn domain_regex() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"^[a-zA-Z0-9]([a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(\.[a-zA-Z0-9]([a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$").unwrap())
}

fn validate_ipv4(ip: &str) -> Result<(), ValidationError> {
    if ip.parse::<std::net::Ipv4Addr>().is_ok() {
        Ok(())
    } else {
        Err(ValidationError::new("invalid_ipv4"))
    }
}

fn validate_ipv6(ip: &str) -> Result<(), ValidationError> {
    if ip.parse::<std::net::Ipv6Addr>().is_ok() {
        Ok(())
    } else {
        Err(ValidationError::new("invalid_ipv6"))
    }
}

fn validate_domain(domain: &str) -> Result<(), ValidationError> {
    if domain_regex().is_match(domain) {
        Ok(())
    } else {
        Err(ValidationError::new("invalid_domain"))
    }
}

/// 创建目标请求（管理员）
#[derive(Debug, Validate, Deserialize)]
pub struct CreateTargetRequest {
    #[validate(length(min = 1, max = 100, message = "名称不能为空且不能超过100字符"))]
    pub name: String,
    #[validate(length(max = 255))]
    #[validate(custom(function = "validate_domain", message = "无效的域名格式"))]
    pub domain: Option<String>,
    #[validate(length(max = 15))]
    #[validate(custom(function = "validate_ipv4", message = "无效的IPv4地址"))]
    pub ipv4: Option<String>,
    #[validate(length(max = 45))]
    #[validate(custom(function = "validate_ipv6", message = "无效的IPv6地址"))]
    pub ipv6: Option<String>,
}

/// 更新目标请求（管理员）
pub type UpdateTargetRequest = CreateTargetRequest;
