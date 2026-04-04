use sea_orm::{DbConn, DbErr, EntityTrait, QueryFilter, ColumnTrait, Condition, PaginatorTrait};
use entity::machine;
use rand::Rng;

use crate::ip_geo::{parse_ip, is_applicant_machine};
use crate::{Query, Mutation};

pub struct ApplyRequest {
    pub ip: String,
    pub province: String,
    pub isp: String,
}

pub struct ApplyResult {
    pub id: i32,
    pub name: String,
    pub key: String,
    pub command: String,
}

#[derive(Debug)]
pub enum ApplyError {
    NotInChina,
    IspNotSupported,
    ProvinceFull,       // 该组合已达3个上限
    IpAlreadyApplied,   // 该IP已有有效申请
    ParseFailed,
    DatabaseError,
}

impl std::fmt::Display for ApplyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApplyError::NotInChina => write!(f, "仅支持中国大陆地区申请"),
            ApplyError::IspNotSupported => write!(f, "当前仅支持联通、电信、移动、铁通、广电运营商"),
            ApplyError::ProvinceFull => write!(f, "该地区该运营商申请人数已达上限"),
            ApplyError::IpAlreadyApplied => write!(f, "该IP已有有效申请，请勿重复申请"),
            ApplyError::ParseFailed => write!(f, "IP解析失败"),
            ApplyError::DatabaseError => write!(f, "数据库错误"),
        }
    }
}

pub struct ApplicationService;

impl ApplicationService {
    /// 检查申请资格
    pub async fn check_eligibility(
        db: &DbConn,
        ip: &str,
    ) -> Result<(String, String), ApplyError> {
        // 1. 解析 IP
        let geo = parse_ip(ip).ok_or(ApplyError::ParseFailed)?;

        // 2. 检查是否在中国
        if geo.country != "中国" {
            return Err(ApplyError::NotInChina);
        }

        // 3. 检查运营商是否支持
        if geo.isp == "其他" {
            return Err(ApplyError::IspNotSupported);
        }

        // 4. 检查 IP 是否已有有效申请
        if Self::has_active_application(db, ip).await? {
            return Err(ApplyError::IpAlreadyApplied);
        }

        // 5. 检查该组合是否已满（3个）
        let count = Self::count_by_province_isp(db, &geo.province, &geo.isp).await?;
        if count >= 3 {
            return Err(ApplyError::ProvinceFull);
        }

        Ok((geo.province, geo.isp))
    }

    /// 提交申请
    pub async fn submit_application(
        db: &DbConn,
        req: ApplyRequest,
    ) -> Result<ApplyResult, ApplyError> {
        // 生成名称：省份运营商001
        let name = Self::generate_machine_name(db, &req.province, &req.isp).await?;

        // 生成随机密钥（32字节，URL-safe base64）
        let key = Self::generate_random_key();

        // 创建 machine
        let machine = Mutation::create_applicant_machine(db, &name, &req.ip, &key)
            .await
            .map_err(|_| ApplyError::DatabaseError)?;

        Ok(ApplyResult {
            id: machine.id,
            name,
            key: key.clone(),
            command: format!("bim -m {} -t {} -s https://your-server.com",
                machine.id, key),
        })
    }

    /// 生成机器名称（单调递增序号）
    async fn generate_machine_name(
        db: &DbConn,
        province: &str,
        isp: &str,
    ) -> Result<String, ApplyError> {
        let prefix = format!("{}{}", province, isp);

        // 查找该前缀下的最大序号
        let machines = Query::find_machines_by_name_prefix(db, &prefix)
            .await
            .map_err(|_| ApplyError::DatabaseError)?;

        let max_seq = machines.iter()
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
        use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
        let mut bytes = [0u8; 32];
        rand::thread_rng().fill(&mut bytes);
        URL_SAFE_NO_PAD.encode(&bytes)
    }

    /// 检查 IP 是否已有有效申请
    async fn has_active_application(
        db: &DbConn,
        ip: &str,
    ) -> Result<bool, ApplyError> {
        use chrono::{Duration, Utc};
        use sea_orm::QueryFilter;

        let one_day_ago = Utc::now().naive_utc() - Duration::hours(24);

        // 查找该 IP 关联的申请者 machine（1天内活跃）
        let count: u64 = machine::Entity::find()
            .filter(machine::Column::Ip.eq(ip))
            .filter(machine::Column::Updated.gt(one_day_ago))
            .filter(
                Condition::any()
                    .add(machine::Column::Name.like("%联通%"))
                    .add(machine::Column::Name.like("%电信%"))
                    .add(machine::Column::Name.like("%移动%"))
                    .add(machine::Column::Name.like("%铁通%"))
                    .add(machine::Column::Name.like("%广电%"))
            )
            .count(db)
            .await
            .map_err(|_| ApplyError::DatabaseError)?;

        Ok(count > 0)
    }

    /// 统计某省份+运营商组合的申请者数量
    async fn count_by_province_isp(
        db: &DbConn,
        province: &str,
        isp: &str,
    ) -> Result<i32, ApplyError> {
        let prefix = format!("{}{}", province, isp);

        let count: u64 = machine::Entity::find()
            .filter(machine::Column::Name.like(format!("{}%", prefix)))
            .count(db)
            .await
            .map_err(|_| ApplyError::DatabaseError)?;

        Ok(count as i32)
    }

    /// 公开：检查是否为申请者机器（供清理任务使用）
    pub fn is_applicant_machine(name: &str) -> bool {
        is_applicant_machine(name)
    }

    /// 清理过期申请者（1天未更新）
    pub async fn clean_expired_applicants(db: &DbConn) -> Result<u64, DbErr> {
        use chrono::{Duration, Utc};

        let one_day_ago = Utc::now().naive_utc() - Duration::hours(24);

        // 查找所有超过1天未更新的 machine
        let machines = machine::Entity::find()
            .filter(
                Condition::any()
                    .add(
                        Condition::all()
                            .add(machine::Column::Updated.is_not_null())
                            .add(machine::Column::Updated.lt(one_day_ago))
                    )
                    .add(
                        Condition::all()
                            .add(machine::Column::Updated.is_null())
                            .add(machine::Column::Created.lt(one_day_ago))
                    )
            )
            .all(db)
            .await?;

        let mut deleted = 0u64;
        for m in machines {
            // 只删除 name 匹配申请者格式的
            if Self::is_applicant_machine(&m.name) {
                let _ = Mutation::delete_machine(db, m.id).await?;
                deleted += 1;
            }
        }

        Ok(deleted)
    }
}
