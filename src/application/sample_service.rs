use std::sync::Arc;

use sea_orm::{DatabaseBackend, FromQueryResult, QueryResult, Statement};
use shaku::{Component, Interface};

use crate::{
    domain::{
        entity::sample::SampleAggregate,
        repo::sample_repo::ISampleRepo,
        value_objects::{ModelResult, SampleID},
    },
    errors::{AppError, AppResult},
    infrastructure::IDbProvider,
};

use super::view::SampleViewRow;
use super::{command::CreateSampleCommand, view::SampleView};

#[async_trait::async_trait]
pub trait ISampleService: Interface {
    async fn get_sample(&self, id: u32) -> AppResult<SampleView>;
    async fn create_sample(&self, cmd: CreateSampleCommand) -> AppResult<()>;
    async fn fetch_latest(&self, count: i32) -> AppResult<Vec<SampleView>>; // 新增
}

#[derive(Component)]
#[shaku(interface = ISampleService)]
pub struct SampleService {
    #[shaku(inject)]
    repo: Arc<dyn ISampleRepo>,
    #[shaku(inject)]
    db: Arc<dyn IDbProvider>,
}

const DB_BACKEND: DatabaseBackend = DatabaseBackend::Sqlite;

#[async_trait::async_trait]
impl ISampleService for SampleService {
    async fn get_sample(&self, id: u32) -> AppResult<SampleView> {
        let r = self.repo.load(id as SampleID).await?;
        Ok(SampleView {
            id: r.id,
            device: r.device,
            created_at: r.created_at,
            sample: r.data,
            predict: r.predict.into(),
            actual: r.actual.into(),
        })
    }

    async fn create_sample(&self, cmd: CreateSampleCommand) -> AppResult<()> {
        self.repo
            .save(SampleAggregate::new(
                0,
                cmd.device,
                cmd.sample,
                ModelResult::from(cmd.predict),
                ModelResult::Unknown,
                "".to_string(),
            ))
            .await
            .unwrap();
        Ok(())
    }
    async fn fetch_latest(&self, count: i32) -> AppResult<Vec<SampleView>> {
        let db = self.db.get_connection();
        let stmt = Statement::from_sql_and_values(
            DB_BACKEND,
            r#"
                SELECT id, device, created_at, sample, predict, actual
                FROM t_sample
                ORDER BY created_at DESC
                LIMIT $1
            "#,
            vec![count.into()],
        );
        let rows: Vec<SampleViewRow> = SampleViewRow::find_by_statement(stmt)
            .all(db.as_ref())
            .await
            .map_err(|e| AppError::DbError { source: e })?;

        let result: Vec<SampleView> = rows
            .into_iter()
            .map(|row| SampleView {
                id: row.id,
                device: row.device,
                created_at: row.created_at,
                sample: serde_json::from_str(&row.sample).unwrap_or_default(),
                predict: row.predict,
                actual: row.actual,
            })
            .collect();
        Ok(result)
    }
}
