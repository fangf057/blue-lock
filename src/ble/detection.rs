use std::{collections::HashMap, fmt::Debug, time::{Duration, Instant}};

use tokio::sync::mpsc;

use crate::{dto::detection::DetectionEvent, errors::AppResult};

use super::sampler::Sampler;

pub struct Detector<T> {
    sampler: Sampler<T>,
    event_tx: mpsc::Sender<DetectionEvent<T>>,
}

impl<T> Detector<T>
where
    T: Default + Copy + Debug + Send + 'static,
{
    pub fn new(
        window_size: usize,
        sample_tx: mpsc::Sender<Vec<T>>,
        event_tx: mpsc::Sender<DetectionEvent<T>>,
    ) -> Self {
        Self {
            sampler: Sampler::new(window_size, sample_tx),
            event_tx,
        }
    }

    pub async fn process(&mut self,  value: T) -> AppResult<()> {
        self.sampler.feed(value).await
    }

    pub fn event_tx(&self) -> &mpsc::Sender<DetectionEvent<T>> {
        &self.event_tx
    }
}