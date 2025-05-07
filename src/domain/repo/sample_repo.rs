use shaku::Interface;

use crate::{domain::{entity::sample::SampleAggregate, value_objects::SampleID}, errors::AppResult};

#[async_trait::async_trait]
pub trait ISampleRepo:Interface{
    async fn load(&self,id:SampleID)->AppResult<SampleAggregate>;
    async fn save(&self,aggregate:SampleAggregate)->AppResult<()>;
}