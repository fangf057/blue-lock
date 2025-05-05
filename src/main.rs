#![allow(non_snake_case)]

use btleplug::{api::{Central, Manager as _, ScanFilter}, platform::Manager};
use dioxus::prelude::*;
use dioxus_demo::{ble::presence_detector::PresenceDetector, routes::Route};
use tracing_subscriber::EnvFilter;


fn main() {
    // Init logger
    // dioxus_logger::init(Level::INFO).expect("failed to init logger");
    tracing_subscriber::fmt()
        .json()  // 启用 JSON 格式
        .with_ansi(false)
        .with_env_filter(EnvFilter::new("info"))  // 日志级别
        .init();
    launch(App);
}

#[component]
fn App() -> Element {
    tokio::spawn(async move {
        let target = "5d964bc66dbc1093";
        let dector = PresenceDetector::new(Default::default()).await.unwrap();
        let manager = Manager::new().await.unwrap();
        let adapter = manager
            .adapters()
            .await
            .unwrap()
            .into_iter()
            .next()
            .unwrap();
        adapter.start_scan(ScanFilter::default()).await.unwrap();
        dector.start_detection(adapter, target).await.unwrap();
    });
    rsx! {
        style { {include_str!("../assets/tailwind.css")} }
        Router::<Route> {}
    }
}
