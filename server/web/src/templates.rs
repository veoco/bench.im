use askama::Template;
use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct Target {
    pub id: i32,
    pub name: String,
    pub updated: i64,
}

#[derive(Serialize, Clone)]
pub struct Machine {
    pub id: i32,
    pub name: String,
    pub ip: String,
}

#[derive(Serialize, Clone)]
pub struct MachineForList {
    pub id: i32,
    pub name: String,
    pub updated: i64,
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub targets: Vec<Target>,
    pub machines: Vec<MachineForList>,
}

#[derive(Template)]
#[template(path = "machine.html")]
pub struct MachineTemplate {
    pub machine: Machine,
    pub targets: Vec<Target>,
    pub machines: Vec<MachineForList>,
}

#[derive(Template)]
#[template(path = "admin/login.html")]
pub struct AdminLoginTemplate {
    pub machines: Vec<MachineForList>,
}

#[derive(Template)]
#[template(path = "admin/index.html")]
pub struct AdminIndexTemplate {
    pub machines: Vec<MachineForList>,
}

#[derive(Template)]
#[template(path = "admin/edit_machine.html")]
pub struct EditMachineTemplate {
    pub is_edit: bool,
    pub id: i32,
    pub name: String,
    pub ip: String,
    pub key: String,
    pub machines: Vec<MachineForList>,
}

#[derive(Template)]
#[template(path = "admin/edit_target.html")]
pub struct EditTargetTemplate {
    pub is_edit: bool,
    pub id: i32,
    pub name: String,
    pub domain: String,
    pub ipv4: String,
    pub ipv6: String,
    pub machines: Vec<MachineForList>,
}

#[derive(Template)]
#[template(path = "admin/delete.html")]
pub struct DeleteTemplate {
    pub item_type: String,
    pub name: String,
    pub ip: String,
    pub domain: String,
    pub ipv4: String,
    pub ipv6: String,
    pub machines: Vec<MachineForList>,
}
