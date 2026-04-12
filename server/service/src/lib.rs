pub mod dto;
pub mod error;
pub mod infrastructure;
pub mod services;

pub use error::ServiceError;
pub use infrastructure::ip_geo::{IpGeoService, IpGeoInfo};

// 导出服务
pub mod service {
    pub use crate::services::{
        machine::MachineService,
        target::TargetService,
        ping::PingService,
        application::{ApplicationService, ApplicationResult},
    };
}

// 导出 DTO
pub mod input {
    pub use crate::dto::input::{
        machine::{CreateMachineRequest, UpdateMachineRequest},
        target::{CreateTargetRequest, UpdateTargetRequest},
        ping::{CreatePingRequest, PingFilter},
        application::CreateApplicationRequest,
    };
}

pub mod output {
    pub use crate::dto::output::{
        machine::{MachineResponse, MachineListItem, MachineWithTargets},
        target::{TargetResponse, TargetDetailResponse},
        ping::{PingResponse, PingData},
    };
}

// 保留 sea_orm 导出供 web 层使用（如 DatabaseConnection）
pub use sea_orm;
