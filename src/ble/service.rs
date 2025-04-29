use btleplug::api::{Central, PeripheralProperties};
use btleplug::api::{Manager as _, Peripheral as _, ScanFilter};
use btleplug::platform::{Manager, Peripheral};
use futures::stream::StreamExt;
use md5::{Digest, Md5};
use std::error::Error;
use std::process::Command;
use std::time::Duration;
use tokio::time;
use tracing::info;
use uuid::Uuid;

// 配置你要监控的蓝牙设备地址
const TARGET_DEVICE_ADDR: &str = "00:11:22:33:44:55"; // 替换为你的设备地址

use std::collections::HashMap;

use crate::dto::device::Device;

fn get_device_fingerprint(props: &Option<PeripheralProperties>) -> String {
    let props = props.to_owned().unwrap_or(PeripheralProperties::default());
    let salt = format!(
        "{}-{:?}-{:?}-{:?}",
        props.local_name.as_deref().unwrap_or(""),
        props.services,
        props.manufacturer_data,
        props.service_data
    );
    // md5
    let mut hasher = Md5::new();
    hasher.update(salt.as_bytes());
    let result = hasher.finalize();
    // 截断 16 位
    format!("{:x}", result)[..16].to_string()
}

pub async fn get_all_device_list() -> Result<Vec<Device>, Box<dyn Error + Send + Sync>> {
    info!("Starting BLE Unlock...");

    #[cfg(target_os = "macos")]
    info!("Monitoring device name: {}", TARGET_DEVICE_ADDR);
    #[cfg(not(target_os = "macos"))]
    info!("Monitoring device address: {}", TARGET_DEVICE_ADDR);

    let manager = Manager::new().await?;
    let adapters = manager.adapters().await?;
    let adapter = adapters
        .into_iter()
        .next()
        .ok_or("No Bluetooth adapters found")?;

    info!("Using adapter: {}", adapter.adapter_info().await?);
    adapter.start_scan(ScanFilter::default()).await?;
    tokio::time::sleep(std::time::Duration::from_secs(5)).await; // 给蓝牙堆栈时间找设备

    info!("Scanning for BLE devices...");

    let mut was_connected = false;

    let peripherals = adapter.peripherals().await?;


    let mut devices  = vec![];

    // 调试：打印所有发现的设备
    info!("Found {} peripherals", peripherals.len());
    for peripheral in &peripherals {
        if let Ok(props) = peripheral.properties().await {
            let fingerprint = get_device_fingerprint(&props);
            if let Some(device_name) = props.as_ref().and_then(|p| p.local_name.as_deref()) {

                // 信号映射 百分比 值越小 信号越强 负数
                let precent  = match props.as_ref().and_then(|p| p.rssi).unwrap_or(0){
                    rssi if rssi > -50 => 20,
                    rssi if rssi > -70 => 40,
                    rssi if rssi > -80 => 60,
                    rssi if rssi > -90 => 80,
                    rssi if rssi > -100 => 100,
                    _ => 0
                };

                // 包含 AppleWatch 就是手表
                let device_type = match device_name{
                    name if name.contains("Watch") => "watch".to_string(),
                    name if name.contains("iPhone") => "phone".to_string(),
                    name if name.contains("iPad") => "pad".to_string(),
                    name if name.contains("MacBook") => "macbook".to_string(),
                    _ => "unknown".to_string()
                };

                devices.push(Device {
                    name: device_name.to_string(),
                    device_type: device_type,
                    rssi: props.as_ref().and_then(|p| p.rssi).unwrap_or(0),
                    percent:precent,
                    signal_color: "from-blue-400 to-blue-600".to_string(),
                    signal_text:"信号".to_string(),
                    status: format!("{}%",precent),
                    ..Default::default()
                })
            }
        }
    }

    let is_connected = match find_target_device(&peripherals).await? {
        Some(peripheral) => peripheral.is_connected().await.unwrap_or(false),
        None => false,
    };


    was_connected = is_connected;

    // sort by rssi
    devices.sort_by(|a, b| a.rssi.cmp(&b.rssi));

    Ok(devices)
}
async fn find_target_device(
    peripherals: &[Peripheral],
) -> Result<Option<Peripheral>, Box<dyn Error + Send + Sync>> {
    for peripheral in peripherals {
        if let Ok(Some(props)) = peripheral.properties().await {
            #[cfg(target_os = "macos")]
            {
                if props.local_name.as_deref().unwrap_or("") == TARGET_DEVICE_ADDR {
                    return Ok(Some(peripheral.clone()));
                }
            }
            #[cfg(not(target_os = "macos"))]
            {
                if peripheral.address().to_string() == TARGET_DEVICE_ADDR {
                    return Ok(Some(peripheral.clone()));
                }
            }
        }
    }
    Ok(None)
}

/// 锁定系统
fn lock_system() -> Result<(), Box<dyn Error + Send + Sync>> {
    info!("lock");
    // cfg_if::cfg_if! {
    //     if #[cfg(target_os = "macos")] {
    //         Command::new("pmset")
    //             .arg("displaysleepnow")
    //             .spawn()?
    //             .wait()?;
    //     } else if #[cfg(target_os = "linux")] {
    //         Command::new("loginctl")
    //             .arg("lock-session")
    //             .spawn()?
    //             .wait()?;
    //     } else if #[cfg(target_os = "windows")] {
    //         Command::new("rundll32.exe")
    //             .arg("user32.dll,LockWorkStation")
    //             .spawn()?
    //             .wait()?;
    //     } else {
    //         return Err("Unsupported operating system".into());
    //     }
    // }
    Ok(())
}

/// 解锁系统
fn unlock_system() -> Result<(), Box<dyn Error + Send + Sync>> {
    info!("unlock");
    // cfg_if::cfg_if! {
    //     if #[cfg(target_os = "macos")] {
    //         Command::new("osascript")
    //             .arg("-e")
    //             .arg("tell application \"System Events\" to keystroke \" \"")
    //             .spawn()?
    //             .wait()?;
    //     } else if #[cfg(target_os = "linux")] {
    //         // Linux可能需要模拟用户输入来解锁
    //         // 这里简单打印消息，实际需要根据你的桌面环境调整
    //         info!("Linux解锁通常需要GUI环境支持");
    //     } else if #[cfg(target_os = "windows")] {
    //         // Windows通常不需要专门解锁
    //         info!("Windows自动解锁通常不需要额外操作");
    //     } else {
    //         return Err("Unsupported operating system".into());
    //     }
    // }
    Ok(())
}
