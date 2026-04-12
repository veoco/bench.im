use askama::Template;

use super::MachineListItem;

/// 申请页面模板
#[derive(Template)]
#[template(path = "apply/index.html")]
pub struct ApplyTemplate {
    pub site_name: String,
    pub eligible: bool,
    pub ip: String,
    pub province: String,
    pub isp: String,
    pub reason: String,
    pub current_count: i32,
    pub max_count: i32,
    pub machines: Vec<MachineListItem>,
    pub current_machine_id: i32,
    pub enable_apply: bool,
    pub is_admin: bool,
}

/// 申请成功页面模板
#[derive(Template)]
#[template(path = "apply/success.html")]
pub struct ApplySuccessTemplate {
    pub site_name: String,
    pub machine_id: i32,
    pub name: String,
    pub key: String,
    pub command: String,
    pub machines: Vec<MachineListItem>,
    pub current_machine_id: i32,
    pub enable_apply: bool,
    pub is_admin: bool,
}

/// 申请功能关闭页面模板
#[derive(Template)]
#[template(path = "apply/disabled.html")]
pub struct ApplyDisabledTemplate {
    pub site_name: String,
    pub machines: Vec<MachineListItem>,
    pub current_machine_id: i32,
    pub enable_apply: bool,
    pub is_admin: bool,
}
