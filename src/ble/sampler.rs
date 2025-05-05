use std::fmt::Debug;

use tokio::sync::mpsc::Sender;
use crate::errors::{AppError, AppResult};
use super::ring_buffer::RingBuffer;

pub struct Sampler<T> {
    buffer: RingBuffer<T>,
    sample_tx: Sender<Vec<T>>,
}

impl<T> Sampler<T>
where
    T: Default + Copy + Debug + Send + 'static,
{
    pub fn new(window_size: usize, sample_tx: Sender<Vec<T>>) -> Self {
        Self {
            buffer: RingBuffer::new(window_size),
            sample_tx,
        }
    }
    
    pub async fn feed(&mut self, val: T) -> AppResult<()> {
        self.buffer.push(val);
        if self.buffer.is_full() {
            let sample = self.buffer.window_data();
            self.sample_tx
                .send(sample)
                .await
                .map_err(|_| AppError::SampleSendError {})?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::sync::mpsc;

    async fn collect_samples<T>(mut rx: mpsc::Receiver<Vec<T>>, count: usize) -> Vec<Vec<T>> {
        let mut samples = Vec::new();
        for _ in 0..count {
            if let Some(sample) = rx.recv().await {
                samples.push(sample);
            }
        }
        samples
    }

    #[tokio::test]
    async fn test_sampler_with_i16() {
        let (tx, rx) = mpsc::channel(128);
        
        let mut sampler = Sampler::new(10, tx);
        for i in 0..100 {
            sampler.feed(i as i16).await.unwrap();
        }

        let samples = collect_samples(rx, 10).await;
        assert_eq!(samples[0], (0..10).collect::<Vec<_>>());
        assert_eq!(samples[9], (90..100).collect::<Vec<_>>());
    }

    #[tokio::test]
    async fn test_sampler_with_f32() {
        let (tx, rx) = mpsc::channel(128);
        
        let mut sampler = Sampler::new(5, tx);
        for i in 0..20 {
            sampler.feed(i as f32 * 0.1).await.unwrap();
        }

        let samples = collect_samples(rx, 4).await;
        assert_eq!(samples[0], vec![0.0, 0.1, 0.2, 0.3, 0.4]);
        assert_eq!(samples[3], vec![1.5, 1.6, 1.7, 1.8, 1.9]);
    }
}