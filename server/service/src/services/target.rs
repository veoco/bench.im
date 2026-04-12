use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, QueryOrder, Set};
use entity::target::{self, Entity as Target};

use crate::dto::input::target::{CreateTargetRequest, UpdateTargetRequest};
use crate::dto::output::target::{TargetDetailResponse, TargetResponse};
use crate::error::ServiceError;

/// 目标服务
pub struct TargetService<'a, C: ConnectionTrait> {
    conn: &'a C,
}

impl<'a, C: ConnectionTrait> TargetService<'a, C> {
    /// 创建新的目标服务实例
    pub fn new(conn: &'a C) -> Self {
        Self { conn }
    }

    // === 公开 API（返回 DTO） ===

    /// 查找所有目标
    pub async fn find_all(&self) -> Result<Vec<TargetResponse>, ServiceError> {
        let targets = Target::find()
            .order_by_asc(target::Column::Name)
            .all(self.conn)
            .await?;

        Ok(targets.into_iter().map(TargetResponse::from).collect())
    }

    /// 根据 ID 查找目标
    pub async fn find_by_id(&self, id: i32) -> Result<Option<TargetResponse>, ServiceError> {
        let target = Target::find_by_id(id).one(self.conn).await?;
        Ok(target.map(TargetResponse::from))
    }

    /// 根据名称查找目标
    pub async fn find_by_name(&self, name: &str) -> Result<Option<TargetResponse>, ServiceError> {
        let target = Target::find()
            .filter(target::Column::Name.eq(name))
            .one(self.conn)
            .await?;
        Ok(target.map(TargetResponse::from))
    }

    // === 管理员 API（返回完整 Model） ===

    /// 根据 ID 查找目标（管理员专用，返回完整 Model）
    pub async fn find_by_id_admin(&self, id: i32) -> Result<Option<target::Model>, ServiceError> {
        Target::find_by_id(id).one(self.conn).await.map_err(Into::into)
    }

    /// 获取所有目标（管理员专用，返回完整 Model）
    pub async fn find_all_admin(&self) -> Result<Vec<target::Model>, ServiceError> {
        Target::find()
            .order_by_asc(target::Column::Name)
            .all(self.conn)
            .await
            .map_err(Into::into)
    }

    /// 查找所有目标详情（管理员专用）
    pub async fn find_all_detail(&self) -> Result<Vec<TargetDetailResponse>, ServiceError> {
        let targets = Target::find()
            .order_by_asc(target::Column::Name)
            .all(self.conn)
            .await?;

        Ok(targets.into_iter().map(TargetDetailResponse::from).collect())
    }

    /// 创建目标
    pub async fn create(&self, req: CreateTargetRequest) -> Result<target::Model, ServiceError> {
        let now = Utc::now().naive_utc();

        let target = target::ActiveModel {
            name: Set(req.name),
            domain: Set(req.domain),
            ipv4: Set(req.ipv4),
            ipv6: Set(req.ipv6),
            created: Set(now),
            ..Default::default()
        }
        .insert(self.conn)
        .await?;

        Ok(target)
    }

    /// 更新目标
    pub async fn update(&self, id: i32, req: UpdateTargetRequest) -> Result<target::Model, ServiceError> {
        // 检查目标是否存在
        let _ = self
            .find_by_id_admin(id)
            .await?
            .ok_or_else(|| ServiceError::not_found("Target", id))?;

        target::ActiveModel {
            id: Set(id),
            name: Set(req.name),
            domain: Set(req.domain),
            ipv4: Set(req.ipv4),
            ipv6: Set(req.ipv6),
            ..Default::default()
        }
        .update(self.conn)
        .await
        .map_err(Into::into)
    }

    /// 更新目标时间戳
    pub async fn touch(&self, id: i32) -> Result<target::Model, ServiceError> {
        target::ActiveModel {
            id: Set(id),
            updated: Set(Some(Utc::now().naive_utc())),
            ..Default::default()
        }
        .update(self.conn)
        .await
        .map_err(Into::into)
    }

    /// 删除目标
    pub async fn delete(&self, id: i32) -> Result<(), ServiceError> {
        // 检查目标是否存在
        let _ = self
            .find_by_id_admin(id)
            .await?
            .ok_or_else(|| ServiceError::not_found("Target", id))?;

        Target::delete_by_id(id).exec(self.conn).await?;
        Ok(())
    }

    /// 确保目标存在（辅助方法）
    pub async fn ensure_exists(&self, id: i32) -> Result<TargetResponse, ServiceError> {
        self.find_by_id(id).await?
            .ok_or_else(|| ServiceError::not_found("Target", id))
    }
}
