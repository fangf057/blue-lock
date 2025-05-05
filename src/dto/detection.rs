use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct DetectionConfig<T> {
    pub window_size: usize,    // 采样窗口大小
    pub threshold: T,         // 检测阈值（泛型）
    pub timeout: Duration,    // 设备超时时间
}

#[derive(Debug)]
pub enum DetectionEvent<T> {
    DevicePresent { avg_value: T, stability: f32 },
    DeviceLost,
    RawSample(Vec<T>),
}

#[derive(Debug)]
pub struct DeviceState<T> {
    pub last_value: T,
    pub last_seen: Instant,
}

#[derive(Debug, Clone)]
pub struct AlgoConfig {
    pub window_size: usize,
    pub threshold: f32,
    pub stability_window: usize,
    pub timeout_secs: u64,
    pub batch_size: usize,
}

impl Default for AlgoConfig {
    fn default() -> Self {
        Self {
            window_size: 10,
            threshold: -70.0,
            stability_window: 5,
            timeout_secs: 15,
            batch_size: 50,
        }
    }
}
