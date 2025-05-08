use std::sync::Arc;

use dioxus::events::TspanExtension;
use sea_orm::{sea_query::Table, ActiveModelTrait, ActiveValue, EntityTrait, IntoActiveModel};
use shaku::Component;

use crate::{
    domain::{
        entity::sample::SampleAggregate,
        repo::sample_repo::ISampleRepo,
        value_objects::{ModelResult, SampleID},
    },
    errors::{AppError, AppResult},
};

use super::{
    model::t_sample::{self, Entity as TSampleEntity},
    DbProvider, IDbProvider,
};

#[derive(Component)]
#[shaku(interface = ISampleRepo)]
pub struct SampleRepo {
    #[shaku(inject)]
    db_provier: Arc<dyn IDbProvider>,
}

#[async_trait::async_trait]
impl ISampleRepo for SampleRepo {
    async fn load(&self, id: SampleID) -> AppResult<SampleAggregate> {
        let conn = self.db_provier.get_connection();
        let s = TSampleEntity::find_by_id(id)
            .one(conn.as_ref())
            .await
            .map_err(|e| AppError::DbError { source: e })?
            .ok_or(AppError::NotFound)?;
        let v = serde_json::from_str::<Vec<f32>>(&s.sample.as_str())
            .map_err(|e| AppError::InvalidData { source: e })?;
        // let t_sample = s.get(0).unwrap();
        // let sample = SampleAggregate::from(t_sample1
        Ok(SampleAggregate::new(
            s.id as SampleID,
            s.device,
            v,
            ModelResult::from(s.predict),
            ModelResult::from(s.actual),
            s.created_at,
        ))
    }
    async fn save(&self, aggregate: SampleAggregate) -> AppResult<()> {
        let conn = self.db_provier.get_connection();
        let mut m = t_sample::ActiveModel {
            id: ActiveValue::Set(aggregate.id as i32),
            device: ActiveValue::Set(aggregate.device),
            sample: ActiveValue::Set(serde_json::to_string(&aggregate.data).unwrap()),
            predict: ActiveValue::Set(aggregate.predict.into()),
            actual: ActiveValue::Set(aggregate.actual.into()),
            ..Default::default()
        };
        if aggregate.id == 0 {
            m. id = ActiveValue::NotSet;
        }

        m.save(conn.as_ref()).await.map_err(|e| AppError::DbError { source: e })?;
        Ok(())
    }
}
