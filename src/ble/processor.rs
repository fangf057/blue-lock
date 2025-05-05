use tokio::sync::mpsc;
use crate::{dto::detection::DetectionEvent, errors::{AppError, AppResult}};

pub struct Processor<T> {
    threshold: T,
    stability_window: usize,
}

impl<T> Processor<T>
where
    T: Copy 
        + std::iter::Sum 
        + std::ops::Div<Output=T>
        + PartialOrd
        + From<f32>  // 添加 From<f32> 约束
        + Into<f32>,
{
    pub fn new(threshold: T, stability_window: usize) -> Self {
        Self {
            threshold,
            stability_window,
        }
    }

    pub async fn process_samples(
        &self,
        samples: &[T],
        event_tx: &mpsc::Sender<DetectionEvent<T>>,
    ) -> AppResult<()> {
        if samples.len() < self.stability_window {
            return Ok(());
        }

        let avg = self.calculate_avg(samples);
        let stability = self.calculate_stability(samples, avg);

        let event = if avg < self.threshold {
            DetectionEvent::DeviceLost
        } else {
            DetectionEvent::DevicePresent { 
                avg_value: avg, 
                stability 
            }
        };

        event_tx.send(event)
            .await
            .map_err(|e| AppError::ProcessingError { r: e.to_string() })?;

        Ok(())
    }

    fn calculate_avg(&self, samples: &[T]) -> T {
        let sum: T = samples.iter().copied().sum();
        let len_f32 = samples.len() as f32;
        let len_t: T = len_f32.into();  // 使用 From<f32> 转换
        sum / len_t
    }

    fn calculate_stability(&self, samples: &[T], avg: T) -> f32 {
        let variance: f32 = samples.iter()
            .map(|&v| {
                let diff: f32 = v.into() - avg.into();
                diff.powi(2)
            })
            .sum::<f32>() / samples.len() as f32;
        1.0 / (1.0 + variance.sqrt())
    }
}