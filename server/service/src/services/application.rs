use std::sync::Arc;

use rand::Rng;
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use sea_orm::ConnectionTrait;

use crate::dto::input::application::CreateApplicationRequest;
use crate::error::ServiceError;
use crate::infrastructure::ip_geo::IpGeoService;
use crate::services::machine::MachineService;

/// 申请结果
#[derive(Debug)]
pub struct ApplicationResult {
    pub id: i32,
    pub name: String,
    pub key: String,
    pub command: String,
}

/// 申请服务
pub struct ApplicationService<'a, C: ConnectionTrait> {
    conn: &'a C,
    ip_geo: Arc<IpGeoService>,
}

impl<'a, C: ConnectionTrait> ApplicationService<'a, C> {
    /// 创建新的申请服务实例
    pub fn new(conn: &'a C, ip_geo: Arc<IpGeoService>) -> Self {
        Self { conn, ip_geo }
    }

    /// 检查申请资格
    /// 返回：(省份, 运营商, 当前申请数量)
    pub async fn check_eligibility(&self, ip: &str) -> Result<(String, String, i32), ServiceError> {
        // 1. 解析 IP
        let geo = self
            .ip_geo
            .parse_ip(ip)
            .ok_or_else(|| ServiceError::Application("IP解析失败".to_string()))?;

        // 2. 检查是否在中国
        if geo.country != "中国" {
            return Err(ServiceError::Application("仅支持中国大陆地区申请".to_string()));
        }

        // 3. 检查运营商是否支持
        if geo.isp == "其他" {
            return Err(ServiceError::Application(
                "当前仅支持联通、电信、移动、铁通、广电运营商".to_string(),
            ));
        }

        // 4. 检查 IP 是否已有有效申请
        let machine_service = MachineService::new(self.conn);
        if machine_service.has_active_application(ip).await? {
            return Err(ServiceError::Application(
                "该IP已有有效申请，请勿重复申请".to_string(),
            ));
        }

        // 5. 检查该组合是否已满（3个）
        let count = machine_service
            .count_by_province_isp(&geo.province, &geo.isp)
            .await?;
        if count >= 3 {
            return Err(ServiceError::Application(format!(
                "该地区该运营商申请人数已达上限（{}/3）",
                count
            )));
        }

        Ok((geo.province, geo.isp, count))
    }

    /// 提交申请
    pub async fn submit(
        &self,
        req: CreateApplicationRequest,
        server_url: &str,
    ) -> Result<ApplicationResult, ServiceError> {
        // 生成名称
        let name = self.generate_machine_name(&req.province, &req.isp).await?;

        // 生成随机密钥（32字节，URL-safe base64）
        let key = Self::generate_random_key();

        // 创建 machine
        let machine_service = MachineService::new(self.conn);
        let machine = machine_service
            .create_applicant(&name, &req.ip, &key)
            .await?;

        Ok(ApplicationResult {
            id: machine.id,
            name,
            key: key.clone(),
            command: format!(
                "bim -m {} \\\n  -t {} \\\n  -s {}",
                machine.id, key, server_url
            ),
        })
    }

    /// 生成机器名称（单调递增序号）
    async fn generate_machine_name(
        &self,
        province: &str,
        isp: &str,
    ) -> Result<String, ServiceError> {
        let prefix = format!("{}{}", province, isp);

        let machine_service = MachineService::new(self.conn);
        let machines = machine_service.find_by_name_prefix(&prefix).await?;

        let max_seq = machines
            .iter()
            .filter_map(|m| {
                let num_part = m.name.strip_prefix(&prefix)?;
                num_part.parse::<i32>().ok()
            })
            .max()
            .unwrap_or(0);

        Ok(format!("{}{:03}", prefix, max_seq + 1))
    }

    /// 生成随机密钥
    fn generate_random_key() -> String {
        let mut bytes = [0u8; 32];
        rand::thread_rng().fill(&mut bytes);
        URL_SAFE_NO_PAD.encode(&bytes)
    }
}
