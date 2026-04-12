use askama::Template;

use crate::templates::{AdminMachine, AdminTarget, Machine, MachineListItem, Target};

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub site_name: String,
    pub targets: Vec<Target>,
    pub machines: Vec<MachineListItem>,
    pub current_machine_id: i32,
    pub enable_apply: bool,
    pub is_admin: bool,
}

#[derive(Template)]
#[template(path = "machine.html")]
pub struct MachineTemplate {
    pub site_name: String,
    pub machine: Machine,
    pub targets: Vec<Target>,
    pub machines: Vec<MachineListItem>,
    pub current_machine_id: i32,
    pub enable_apply: bool,
    pub is_admin: bool,
}

#[derive(Template)]
#[template(path = "target.html")]
pub struct TargetTemplate {
    pub site_name: String,
    pub target: Target,
    pub machines: Vec<MachineListItem>, // 用于侧边栏机器列表
    pub target_machines: Vec<Machine>,  // 用于目标页面的机器列表（显示图表）
    pub current_machine_id: i32,
    pub enable_apply: bool,
    pub is_admin: bool,
}

#[derive(Template)]
#[template(path = "admin/login.html")]
pub struct AdminLoginTemplate {
    pub site_name: String,
    pub machines: Vec<MachineListItem>,
    pub current_machine_id: i32,
    pub enable_apply: bool,
    pub is_admin: bool,
}

#[derive(Template)]
#[template(path = "admin/index.html")]
pub struct AdminIndexTemplate {
    pub site_name: String,
    pub machines: Vec<MachineListItem>, // 用于侧边栏
    pub current_machine_id: i32,
    pub admin_machines: Vec<AdminMachine>, // 用于管理列表
    pub admin_targets: Vec<AdminTarget>,   // 用于管理列表
    pub enable_apply: bool,
    pub is_admin: bool,
}

#[derive(Template)]
#[template(path = "admin/edit_machine.html")]
pub struct EditMachineTemplate {
    pub site_name: String,
    pub is_edit: bool,
    pub id: i32,
    pub name: String,
    pub ip: String,
    pub key: String,
    pub machines: Vec<MachineListItem>,
    pub current_machine_id: i32,
    pub enable_apply: bool,
    pub is_admin: bool,
}

#[derive(Template)]
#[template(path = "admin/edit_target.html")]
pub struct EditTargetTemplate {
    pub site_name: String,
    pub is_edit: bool,
    pub id: i32,
    pub name: String,
    pub domain: String,
    pub ipv4: String,
    pub ipv6: String,
    pub machines: Vec<MachineListItem>,
    pub current_machine_id: i32,
    pub enable_apply: bool,
    pub is_admin: bool,
}

#[derive(Template)]
#[template(path = "admin/delete.html")]
pub struct DeleteTemplate {
    pub site_name: String,
    pub item_type: String,
    pub name: String,
    pub ip: String,
    pub domain: String,
    pub ipv4: String,
    pub ipv6: String,
    pub machines: Vec<MachineListItem>,
    pub current_machine_id: i32,
    pub enable_apply: bool,
    pub is_admin: bool,
}

impl DeleteTemplate {
    /// 创建机器删除模板
    pub fn for_machine(
        site_name: String,
        name: String,
        ip: String,
        machines: Vec<MachineListItem>,
        current_machine_id: i32,
        enable_apply: bool,
    ) -> Self {
        Self {
            site_name,
            item_type: "机器".to_string(),
            name,
            ip,
            domain: String::new(),
            ipv4: String::new(),
            ipv6: String::new(),
            machines,
            current_machine_id,
            enable_apply,
            is_admin: true,
        }
    }

    /// 创建目标删除模板
    pub fn for_target(
        site_name: String,
        name: String,
        domain: Option<String>,
        ipv4: Option<String>,
        ipv6: Option<String>,
        machines: Vec<MachineListItem>,
        enable_apply: bool,
    ) -> Self {
        Self {
            site_name,
            item_type: "目标".to_string(),
            name,
            ip: String::new(),
            domain: domain.unwrap_or_default(),
            ipv4: ipv4.unwrap_or_default(),
            ipv6: ipv6.unwrap_or_default(),
            machines,
            current_machine_id: 0,
            enable_apply,
            is_admin: true,
        }
    }
}
