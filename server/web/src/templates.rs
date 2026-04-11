use askama::Template;
use serde::Serialize;

// 从 service 层导入 MachineForList
pub use server_service::MachineForList;

#[derive(Serialize, Clone)]
pub struct Target {
    pub id: i32,
    pub name: String,
    pub updated: i64,
}

// Admin 页面使用的机器结构（包含完整信息）
#[derive(Serialize, Clone)]
pub struct AdminMachine {
    pub id: i32,
    pub name: String,
    pub ip: String,
}

// Admin 页面使用的目标结构（包含完整信息）
#[derive(Serialize, Clone)]
pub struct AdminTarget {
    pub id: i32,
    pub name: String,
    pub domain: String,
    pub ipv4: String,
    pub ipv6: String,
}

#[derive(Serialize, Clone)]
pub struct Machine {
    pub id: i32,
    pub name: String,
    pub ip: String,
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub site_name: String,
    pub targets: Vec<Target>,
    pub machines: Vec<MachineForList>,
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
    pub machines: Vec<MachineForList>,
    pub current_machine_id: i32,
    pub enable_apply: bool,
    pub is_admin: bool,
}

#[derive(Template)]
#[template(path = "target.html")]
pub struct TargetTemplate {
    pub site_name: String,
    pub target: Target,
    pub machines: Vec<MachineForList>, // 用于侧边栏机器列表
    pub target_machines: Vec<Machine>, // 用于目标页面的机器列表（显示图表）
    pub current_machine_id: i32,
    pub enable_apply: bool,
    pub is_admin: bool,
}

#[derive(Template)]
#[template(path = "admin/login.html")]
pub struct AdminLoginTemplate {
    pub site_name: String,
    pub machines: Vec<MachineForList>,
    pub current_machine_id: i32,
    pub enable_apply: bool,
    pub is_admin: bool,
}

#[derive(Template)]
#[template(path = "admin/index.html")]
pub struct AdminIndexTemplate {
    pub site_name: String,
    pub machines: Vec<MachineForList>, // 用于侧边栏
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
    pub machines: Vec<MachineForList>,
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
    pub machines: Vec<MachineForList>,
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
    pub machines: Vec<MachineForList>,
    pub current_machine_id: i32,
    pub enable_apply: bool,
    pub is_admin: bool,
}
