use std::sync::Arc;

use shaku::{Component, Interface};

use crate::{
    domain::{
        entity::sample::SampleAggregate,
        repo::sample_repo::ISampleRepo,
        value_objects::{ModelResult, SampleID},
    },
    errors::AppResult,
};

use super::{command::CreateSampleCommand, view::SampleView};

#[async_trait::async_trait]
pub trait ISampleService: Interface {
    async fn get_sample(&self, id: u32) -> AppResult<SampleView>;
    async fn create_sample(&self, cmd: CreateSampleCommand) -> AppResult<()>;
}

#[derive(Component)]
#[shaku(interface = ISampleService)]
pub struct SampleService {
    #[shaku(inject)]
    repo: Arc<dyn ISampleRepo>,
}

#[async_trait::async_trait]
impl ISampleService for SampleService {
    async fn get_sample(&self, id: u32) -> AppResult<SampleView> {
        let r = self.repo.load(id as SampleID).await?;
        Ok(SampleView {
            id: r.id,
            device: r.device,
            sample: r.data,
        })
    }

    async fn create_sample(&self, cmd: CreateSampleCommand) -> AppResult<()> {
        self.repo
            .save(SampleAggregate::new(
                0,
                cmd.device,
                cmd.sample,
                ModelResult::from(cmd.predict),
                ModelResult::Unknown
            ))
            .await
            .unwrap();
        Ok(())
    }
}
