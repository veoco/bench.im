use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, QueryOrder, Set};
use entity::target::{self, Entity as TargetEntity};

use crate::dto::input::target::{CreateTargetRequest, UpdateTargetRequest};
use crate::dto::output::target::Target as TargetDto;
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

    // === 公开 API（泛型版本） ===

    /// 查找所有目标
    ///
    /// 使用示例：
    /// - `let targets: Vec<Target> = service.find_all().await?;`
    /// - `let public: Vec<TargetPublic> = service.find_all().await?;`
    pub async fn find_all<T>(&self) -> Result<Vec<T>, ServiceError>
    where
        T: From<target::Model>,
    {
        let targets = TargetEntity::find()
            .order_by_asc(target::Column::Name)
            .all(self.conn)
            .await?;

        Ok(targets.into_iter().map(Into::into).collect())
    }

    /// 根据 ID 查找目标
    ///
    /// 使用示例：
    /// - `let target: Option<Target> = service.find_by_id(id).await?;`
    pub async fn find_by_id<T>(&self, id: i32) -> Result<Option<T>, ServiceError>
    where
        T: From<target::Model>,
    {
        let target = TargetEntity::find_by_id(id).one(self.conn).await?;
        Ok(target.map(Into::into))
    }

    /// 根据名称查找目标
    ///
    /// 使用示例：
    /// - `let target: Option<Target> = service.find_by_name("name").await?;`
    pub async fn find_by_name<T>(&self, name: &str) -> Result<Option<T>, ServiceError>
    where
        T: From<target::Model>,
    {
        let target = TargetEntity::find()
            .filter(target::Column::Name.eq(name))
            .one(self.conn)
            .await?;
        Ok(target.map(Into::into))
    }

    /// 创建目标
    pub async fn create(&self, req: CreateTargetRequest) -> Result<TargetDto, ServiceError> {
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

        Ok(TargetDto::from(target))
    }

    /// 更新目标
    pub async fn update(&self, id: i32, req: UpdateTargetRequest) -> Result<TargetDto, ServiceError> {
        // 检查目标是否存在
        let _ = self
            .find_by_id_raw(id)
            .await?
            .ok_or_else(|| ServiceError::not_found("Target", id))?;

        let target = target::ActiveModel {
            id: Set(id),
            name: Set(req.name),
            domain: Set(req.domain),
            ipv4: Set(req.ipv4),
            ipv6: Set(req.ipv6),
            ..Default::default()
        }
        .update(self.conn)
        .await?;

        Ok(TargetDto::from(target))
    }

    /// 删除目标
    pub async fn delete(&self, id: i32) -> Result<(), ServiceError> {
        // 检查目标是否存在
        let _ = self
            .find_by_id_raw(id)
            .await?
            .ok_or_else(|| ServiceError::not_found("Target", id))?;

        TargetEntity::delete_by_id(id).exec(self.conn).await?;
        Ok(())
    }

    /// 确保目标存在（辅助方法）
    pub async fn ensure_exists(&self, id: i32) -> Result<TargetDto, ServiceError> {
        self.find_by_id(id).await?
            .ok_or_else(|| ServiceError::not_found("Target", id))
    }

    // === 内部/服务间调用（返回原始 Model） ===

    /// 根据 ID 查找目标（内部使用，返回原始 Model）
    pub(crate) async fn find_by_id_raw(&self, id: i32) -> Result<Option<target::Model>, ServiceError> {
        TargetEntity::find_by_id(id).one(self.conn).await.map_err(Into::into)
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
}
