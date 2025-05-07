#![allow(non_snake_case)]

use std::sync::Arc;

use btleplug::{
    api::{Central, Manager as _, ScanFilter},
    platform::Manager,
};
use dioxus::prelude::*;
use dioxus_demo::{
    ble::presence_detector::PresenceDetector,
    di::Deps,
    errors::AppResult,
    infrastructure::{DbProvider, DbProviderParameters},
    routes::Route,
};
use sea_orm::Database;
use tracing::info;
use tracing_subscriber::EnvFilter;

fn main() -> AppResult<()> {
    // Init logger
    // dioxus_logger::init(Level::INFO).expect("failed to init logger");

    tracing_subscriber::fmt()
        .json() // 启用 JSON 格式
        .with_ansi(false)
        .with_env_filter(EnvFilter::new("info")) // 日志级别
        .init();
    launch(App);
    Ok(())
}

#[component]
fn App() -> Element {
    let samples_signal = use_signal(|| Vec::<f32>::new());
    let mut deps = use_signal::<Option<Arc<Deps>>>(|| None);


    use_future({
        let mut signal = samples_signal.clone();
        // 不要 let sample_rx = ... 再 move！
        move || async move {
            let db = Database::connect("sqlite:sample.db")
                .await
                .expect("Database connection failed");

            let dps = Deps::builder()
                .with_component_parameters::<DbProvider>(DbProviderParameters {
                    conn: Arc::new(db),
                })
                .build();

            deps.set(Some(Arc::new(dps)));

            let (sample_tx, mut sample_rx) = tokio::sync::mpsc::channel::<Vec<f32>>(100);
            let target = "5d964bc66dbc1093";
            let dector = PresenceDetector::new(Default::default(), sample_tx)
                .await
                .unwrap();
            let manager = Manager::new().await.unwrap();
            let adapter = manager
                .adapters()
                .await
                .unwrap()
                .into_iter()
                .next()
                .unwrap();
            tokio::spawn(async move {
                adapter.start_scan(ScanFilter::default()).await.unwrap();
                dector.start_detection(adapter, target).await.unwrap();
            });

            while let Some(samples) = sample_rx.recv().await {
                info!("received samples: {:?}", samples);
                signal.set(samples);
            }
        }
    });

    use_context_provider(|| samples_signal.clone());
    use_context_provider(|| deps.clone());

    rsx! {
        style { {include_str!("../assets/tailwind.css")} }
        Router::<Route> {}
    }
}
