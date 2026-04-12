pub use server_service::output::MachineListItem;

pub mod application;
pub mod pages;

use serde::Serialize;

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
