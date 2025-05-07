use crate::{
    ble::model::Model,
    dto::detection::{AlgoConfig, DetectionEvent},
};
use btleplug::{
    api::{Central as _, Peripheral as _},
    platform::{Adapter, PeripheralId},
};
use dioxus::logger::tracing::{info, warn};
use futures::StreamExt as _;
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};
use tokio::sync::{mpsc, oneshot};

use super::{detection::Detector, service::get_device_fingerprint};

pub struct PresenceDetector {
    cmd_tx: mpsc::Sender<ProcessorMsg>,
}

enum ProcessorMsg {
    Sample { device_id: String, rssi: i16 },
    GetStatus(oneshot::Sender<HashMap<String, DeviceStatus>>),
    Shutdown,
}

#[derive(Debug)]
struct DeviceStatus {
    last_rssi: f32,
    last_seen: Instant,
}

impl PresenceDetector {
    pub async fn new(config: AlgoConfig,sample_tx: mpsc::Sender<Vec<f32>>) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let (cmd_tx, cmd_rx) = mpsc::channel(100);
        let (event_tx, mut event_rx) = mpsc::channel(100);

        // 使用 f32 类型初始化 Processor
        let detector = Detector::new(
            config.window_size,
            sample_tx,
            event_tx,
        );

        // 处理事件
        tokio::spawn(async move {
            while let Some(event) = event_rx.recv().await {
                match event {
                    DetectionEvent::DevicePresent {
                        avg_value,
                        stability,
                    } => {
                        info!(
                            "Device present: avg_value={:.2}, stability={:.2}",
                            avg_value, stability
                        );
                    }
                    DetectionEvent::DeviceLost => {
                        info!("Device lost");
                    }
                    DetectionEvent::RawSample(items) => {
                        info!("Raw samples: {:?}", items);
                    }
                }
            }
        });

        tokio::spawn(Self::processing_task(
            cmd_rx, detector,
        ));

        Ok(Self { cmd_tx})
    }

    async fn processing_task(
        mut cmd_rx: mpsc::Receiver<ProcessorMsg>,
        // mut sample_rx: mpsc::Receiver<Vec<f32>>,
        mut detector: Detector<f32>,
    ) {
        let mut last_sample_at = Instant::now();

        loop {
            tokio::select! {
                Some(msg) = cmd_rx.recv() => {
                    match msg {
                        ProcessorMsg::Sample { device_id, rssi } => {
                            // 采样
                            if last_sample_at.elapsed() >= Duration::from_millis(10) {
                                info!(
                                    name: "processor",
                                    device_id = %format!(r#""{}""#, device_id),  // 用引号包裹
                                    rssi,
                                    "Processing sample for device"
                                );
                                let _ = detector.process(rssi as f32).await;
                                last_sample_at = Instant::now();
                            }
                        }
                        ProcessorMsg::GetStatus(reply) => {
                            let _ = reply.send(HashMap::new());
                        }
                        ProcessorMsg::Shutdown => break,
                    }
                }
            }
        }
    }

    pub async fn start_detection(
        self,
        adapter: Adapter,
        target: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut events = adapter.events().await?;

        loop {
            if let Some(btleplug::api::CentralEvent::DeviceUpdated(id)) = events.next().await {
                if let Err(e) = self.handle_device_update(&adapter, &id, target).await {
                    warn!("Device update error: {}", e);
                }
            }
        }
    }

    async fn handle_device_update(
        &self,
        adapter: &Adapter,
        device_id: &PeripheralId,
        target: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let device = adapter.peripheral(device_id).await?;
        let props = device.properties().await?.unwrap_or_default();

        if let (Some(rssi), Some(name)) = (props.rssi, props.local_name) {
            let fingerprint = get_device_fingerprint(&name);
            if fingerprint.eq(target) {
                self.cmd_tx
                    .send(ProcessorMsg::Sample {
                        device_id: name.clone(),
                        rssi,
                    })
                    .await?;
            }
        }

        Ok(())
    }

    pub async fn shutdown(&self) {
        let _ = self.cmd_tx.send(ProcessorMsg::Shutdown).await;
    }
}
