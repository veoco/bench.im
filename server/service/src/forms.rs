use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Validate, Deserialize)]
pub struct PingCreate {
    pub ipv6: bool,
    #[validate(range(max = 1000))]
    pub min: u16,
    #[validate(range(max = 1000))]
    pub avg: u16,
    #[validate(range(max = 20))]
    pub fail: u8,
}

#[derive(Debug, Validate, Deserialize)]
pub struct PingFilter {
    pub ipv6: Option<bool>,
}

#[derive(Debug, Validate, Deserialize)]
pub struct MachineCreateAdmin {
    #[validate(length(min = 1, max = 100, message = "名称不能为空且不能超过100字符"))]
    pub name: String,
    #[validate(length(min = 1, max = 255, message = "IP不能为空"))]
    pub ip: String,
    #[validate(length(min = 1, max = 255, message = "密钥不能为空"))]
    pub key: String,
}

#[derive(Debug, Validate, Deserialize)]
pub struct TargetCreateAdmin {
    #[validate(length(min = 1, max = 100, message = "名称不能为空且不能超过100字符"))]
    pub name: String,
    pub domain: Option<String>,
    pub ipv4: Option<String>,
    pub ipv6: Option<String>,
}
