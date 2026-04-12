use std::collections::HashMap;
use std::time::Duration;

use chrono::{Timelike, Utc};
use sea_orm::{ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, QueryOrder, QuerySelect, Set, TransactionTrait};
use entity::ping::{self, Entity as Ping};

use crate::services::machine::MachineService;
use crate::services::target::TargetService;

use crate::dto::input::ping::CreatePingRequest;
use crate::dto::output::ping::{PingData, PingResponse};
use crate::error::ServiceError;

/// Ping 服务
pub struct PingService<'a, C: ConnectionTrait> {
    conn: &'a C,
}

impl<'a, C: ConnectionTrait> PingService<'a, C> {
    /// 创建新的 Ping 服务实例
    pub fn new(conn: &'a C) -> Self {
        Self { conn }
    }

    /// 查找指定机器和目标的 Ping 记录
    pub async fn find_by_machine_and_target(
        &self,
        machine_id: i32,
        target_id: i32,
        delta: &str,
        ipv6: bool,
    ) -> Result<Vec<PingResponse>, ServiceError> {
        let time_delta = if delta == "7d" {
            Duration::from_secs(7 * 24 * 3600)
        } else {
            Duration::from_secs(24 * 3600)
        };

        let pings = Ping::find()
            .filter(ping::Column::MachineId.eq(machine_id))
            .filter(ping::Column::TargetId.eq(target_id))
            .filter(ping::Column::Ipv6.eq(ipv6))
            .filter(ping::Column::Created.gt(Utc::now().naive_utc() - time_delta))
            .order_by_asc(ping::Column::Created)
            .all(self.conn)
            .await?;

        Ok(pings.into_iter().map(PingResponse::from).collect())
    }

    /// 批量获取指定机器所有目标的图表数据（用于机器页，解决 N+1 问题）
    pub async fn find_for_machine_targets(
        &self,
        machine_id: i32,
        target_ids: Vec<i32>,
        delta: &str,
        ipv6: bool,
    ) -> Result<HashMap<i32, Vec<PingData>>, ServiceError> {
        if target_ids.is_empty() {
            return Ok(HashMap::new());
        }

        let time_delta = if delta == "7d" {
            Duration::from_secs(7 * 24 * 3600)
        } else {
            Duration::from_secs(24 * 3600)
        };

        // 一次性查询所有目标的 ping 数据
        let pings = Ping::find()
            .filter(ping::Column::MachineId.eq(machine_id))
            .filter(ping::Column::TargetId.is_in(target_ids))
            .filter(ping::Column::Ipv6.eq(ipv6))
            .filter(ping::Column::Created.gt(Utc::now().naive_utc() - time_delta))
            .order_by_asc(ping::Column::Created)
            .all(self.conn)
            .await?;

        // 按 target_id 分组
        let mut result: HashMap<i32, Vec<PingData>> = HashMap::new();
        for ping in pings {
            let data: PingData = (
                ping.created.and_utc().timestamp(),
                ping.min,
                ping.avg,
                ping.fail,
            );
            result.entry(ping.target_id).or_default().push(data);
        }

        Ok(result)
    }

    /// 获取指定 target 的最新 ping 记录时间（用于首页状态显示）
    pub async fn find_latest_by_target(
        &self,
        target_id: i32,
    ) -> Result<Option<chrono::NaiveDateTime>, ServiceError> {
        let result: Option<(chrono::NaiveDateTime,)> = Ping::find()
            .select_only()
            .column(ping::Column::Created)
            .filter(ping::Column::TargetId.eq(target_id))
            .order_by_desc(ping::Column::Created)
            .into_tuple()
            .one(self.conn)
            .await?;
        Ok(result.map(|(dt,)| dt))
    }

    /// 获取指定机器和目标的最新 ping 记录时间（用于 machine 页面状态显示）
    pub async fn find_latest_by_machine_and_target(
        &self,
        machine_id: i32,
        target_id: i32,
    ) -> Result<Option<chrono::NaiveDateTime>, ServiceError> {
        let result: Option<(chrono::NaiveDateTime,)> = Ping::find()
            .select_only()
            .column(ping::Column::Created)
            .filter(ping::Column::MachineId.eq(machine_id))
            .filter(ping::Column::TargetId.eq(target_id))
            .order_by_desc(ping::Column::Created)
            .into_tuple()
            .one(self.conn)
            .await?;
        Ok(result.map(|(dt,)| dt))
    }

    /// 创建 Ping 记录
    pub async fn create(
        &self,
        machine_id: i32,
        target_id: i32,
        req: CreatePingRequest,
    ) -> Result<ping::Model, ServiceError> {
        let now = Utc::now().naive_utc();
        // 将时间戳对齐到 5 分钟
        let rounded = now
            .with_minute((now.minute() / 5) * 5)
            .and_then(|dt| dt.with_second(0))
            .and_then(|dt| dt.with_nanosecond(0))
            .unwrap_or(now);

        let ping = ping::ActiveModel {
            machine_id: Set(machine_id),
            target_id: Set(target_id),
            ipv6: Set(req.ipv6),
            created: Set(rounded),
            min: Set(req.min as i32),
            avg: Set(req.avg as i32),
            fail: Set(req.fail as i32),
            ..Default::default()
        }
        .insert(self.conn)
        .await?;

        Ok(ping)
    }

    /// 删除过期 Ping 记录（7天前）
    pub async fn delete_expired(&self) -> Result<u64, ServiceError> {
        let result = Ping::delete_many()
            .filter(ping::Column::Created.lt(Utc::now().naive_utc() - Duration::from_secs(7 * 24 * 3600)))
            .exec(self.conn)
            .await?;

        Ok(result.rows_affected)
    }

    /// 创建 Ping 记录并更新相关 machine 和 target（事务包裹）
    pub async fn create_with_updates(
        &self,
        machine_id: i32,
        target_id: i32,
        req: CreatePingRequest,
        client_ip: String,
    ) -> Result<(), ServiceError>
    where
        C: TransactionTrait,
    {
        self.conn.transaction::<_, _, ServiceError>(|txn| {
            Box::pin(async move {
                // 1. 创建 ping 记录
                let ping_service = PingService::new(txn);
                ping_service.create(machine_id, target_id, req).await?;

                // 2. 更新 machine IP 和更新时间
                let machine_service = MachineService::new(txn);
                machine_service.update_ip(machine_id, client_ip).await?;

                // 3. 更新 target 时间戳
                let target_service = TargetService::new(txn);
                target_service.touch(target_id).await?;

                Ok(())
            })
        })
        .await
        .map_err(|e| match e {
            sea_orm::TransactionError::Connection(err) => ServiceError::Database(err),
            sea_orm::TransactionError::Transaction(err) => err,
        })
    }
}
