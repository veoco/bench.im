use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, ConnectionTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set};
use entity::machine::{self, Entity as Machine};

use crate::dto::input::machine::{CreateMachineRequest, UpdateMachineRequest};
use crate::dto::output::machine::{MachineListItem, MachineResponse};
use crate::error::ServiceError;
use crate::infrastructure::ip_geo::is_applicant_machine;

/// 机器服务
pub struct MachineService<'a, C: ConnectionTrait> {
    conn: &'a C,
}

impl<'a, C: ConnectionTrait> MachineService<'a, C> {
    /// 创建新的机器服务实例
    pub fn new(conn: &'a C) -> Self {
        Self { conn }
    }

    // === 公开 API（返回 DTO） ===

    /// 查找所有机器
    pub async fn find_all(&self) -> Result<Vec<MachineResponse>, ServiceError> {
        let machines = Machine::find()
            .order_by_asc(machine::Column::Name)
            .all(self.conn)
            .await?;

        Ok(machines.into_iter().map(MachineResponse::from).collect())
    }

    /// 获取机器列表（精简版，用于侧边栏）
    pub async fn find_all_for_list(&self) -> Result<Vec<MachineListItem>, ServiceError> {
        let machines = Machine::find()
            .order_by_asc(machine::Column::Name)
            .all(self.conn)
            .await?;

        Ok(machines.into_iter().map(MachineListItem::from).collect())
    }

    /// 根据 ID 查找机器（公开 API，返回脱敏 DTO）
    pub async fn find_by_id(&self, id: i32) -> Result<Option<MachineResponse>, ServiceError> {
        let machine = Machine::find_by_id(id).one(self.conn).await?;
        Ok(machine.map(MachineResponse::from))
    }

    /// 根据名称查找机器
    pub async fn find_by_name(&self, name: &str) -> Result<Option<MachineResponse>, ServiceError> {
        let machine = Machine::find()
            .filter(machine::Column::Name.eq(name))
            .one(self.conn)
            .await?;
        Ok(machine.map(MachineResponse::from))
    }

    /// 创建机器
    pub async fn create(&self, req: CreateMachineRequest) -> Result<MachineResponse, ServiceError> {
        // 检查名称是否已存在
        if self.find_by_name(&req.name).await?.is_some() {
            return Err(ServiceError::conflict(format!(
                "Machine with name '{}' already exists",
                req.name
            )));
        }

        let now = Utc::now().naive_utc();

        let machine = machine::ActiveModel {
            name: Set(req.name),
            ip: Set(req.ip),
            key: Set(req.key),
            created: Set(now),
            ..Default::default()
        }
        .insert(self.conn)
        .await?;

        Ok(MachineResponse::from(machine))
    }

    /// 更新机器
    pub async fn update(&self, id: i32, req: UpdateMachineRequest) -> Result<MachineResponse, ServiceError> {
        // 检查机器是否存在
        let _ = self
            .find_by_id_admin(id)
            .await?
            .ok_or_else(|| ServiceError::not_found("Machine", id))?;

        let now = Utc::now().naive_utc();

        let machine = machine::ActiveModel {
            id: Set(id),
            name: Set(req.name),
            ip: Set(req.ip),
            key: Set(req.key),
            created: Set(now),
            ..Default::default()
        }
        .update(self.conn)
        .await?;

        Ok(MachineResponse::from(machine))
    }

    // === 管理员 API（返回完整 Model） ===

    /// 根据 ID 查找机器（管理员专用，返回完整 Model）
    pub async fn find_by_id_admin(&self, id: i32) -> Result<Option<machine::Model>, ServiceError> {
        Machine::find_by_id(id).one(self.conn).await.map_err(Into::into)
    }

    /// 获取所有机器（管理员专用，返回完整 Model）
    pub async fn find_all_admin(&self) -> Result<Vec<machine::Model>, ServiceError> {
        Machine::find()
            .order_by_asc(machine::Column::Name)
            .all(self.conn)
            .await
            .map_err(Into::into)
    }

    // === 内部/服务间调用（返回 Model） ===

    /// 根据名称前缀查找机器（内部使用）
    pub(crate) async fn find_by_name_prefix(&self, prefix: &str) -> Result<Vec<machine::Model>, ServiceError> {
        Machine::find()
            .filter(machine::Column::Name.like(format!("{}%", prefix)))
            .order_by_asc(machine::Column::Name)
            .all(self.conn)
            .await
            .map_err(Into::into)
    }

    /// 创建申请者机器（用于自助申请）
    pub async fn create_applicant(
        &self,
        name: &str,
        ip: &str,
        key: &str,
    ) -> Result<machine::Model, ServiceError> {
        let now = Utc::now().naive_utc();

        machine::ActiveModel {
            name: Set(name.to_string()),
            ip: Set(ip.to_string()),
            key: Set(key.to_string()),
            created: Set(now),
            ..Default::default()
        }
        .insert(self.conn)
        .await
        .map_err(Into::into)
    }

    /// 更新机器 IP 和更新时间
    pub async fn update_ip(&self, id: i32, ip: String) -> Result<machine::Model, ServiceError> {
        machine::ActiveModel {
            id: Set(id),
            ip: Set(ip),
            updated: Set(Some(Utc::now().naive_utc())),
            ..Default::default()
        }
        .update(self.conn)
        .await
        .map_err(Into::into)
    }

    /// 删除机器
    pub async fn delete(&self, id: i32) -> Result<(), ServiceError> {
        // 检查机器是否存在
        let _ = self
            .find_by_id_admin(id)
            .await?
            .ok_or_else(|| ServiceError::not_found("Machine", id))?;

        Machine::delete_by_id(id).exec(self.conn).await?;
        Ok(())
    }

    // === 业务方法 ===

    /// 清理过期申请者（1天未更新）
    pub async fn clean_expired_applicants(&self) -> Result<u64, ServiceError> {
        use chrono::Duration;

        let one_day_ago = Utc::now().naive_utc() - Duration::hours(24);

        // 查找所有超过1天未更新的 machine
        let machines = Machine::find()
            .filter(
                Condition::any()
                    .add(
                        Condition::all()
                            .add(machine::Column::Updated.is_not_null())
                            .add(machine::Column::Updated.lt(one_day_ago)),
                    )
                    .add(
                        Condition::all()
                            .add(machine::Column::Updated.is_null())
                            .add(machine::Column::Created.lt(one_day_ago)),
                    ),
            )
            .all(self.conn)
            .await?;

        let mut deleted = 0u64;
        for m in machines {
            // 只删除 name 匹配申请者格式的
            if is_applicant_machine(&m.name) {
                Machine::delete_by_id(m.id).exec(self.conn).await?;
                deleted += 1;
            }
        }

        Ok(deleted)
    }

    /// 统计某省份+运营商组合的申请者数量
    pub async fn count_by_province_isp(&self, province: &str, isp: &str) -> Result<i32, ServiceError> {
        let prefix = format!("{}{}", province, isp);

        let count = Machine::find()
            .filter(machine::Column::Name.like(format!("{}%", prefix)))
            .count(self.conn)
            .await?;

        Ok(count as i32)
    }

    /// 检查 IP 是否已有有效申请（1天内活跃）
    pub async fn has_active_application(&self, ip: &str) -> Result<bool, ServiceError> {
        use chrono::Duration;

        let one_day_ago = Utc::now().naive_utc() - Duration::hours(24);

        let count = Machine::find()
            .filter(machine::Column::Ip.eq(ip))
            .filter(machine::Column::Updated.gt(one_day_ago))
            .filter(
                Condition::any()
                    .add(machine::Column::Name.like("%联通%"))
                    .add(machine::Column::Name.like("%电信%"))
                    .add(machine::Column::Name.like("%移动%"))
                    .add(machine::Column::Name.like("%铁通%"))
                    .add(machine::Column::Name.like("%广电%")),
            )
            .count(self.conn)
            .await?;

        Ok(count > 0)
    }

    /// 确保机器存在（辅助方法）
    pub async fn ensure_exists(&self, id: i32) -> Result<MachineResponse, ServiceError> {
        self.find_by_id(id).await?
            .ok_or_else(|| ServiceError::not_found("Machine", id))
    }
}
