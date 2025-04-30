use crate::{ble::service::get_all_device_list, dto::device::Device};
use dioxus::prelude::*;
use tracing::info;

#[component]
pub fn DeviceList() -> Element {
    // 信号量用于更新设备列表
    let devices = use_signal(|| Vec::<Device>::new());
    use_future({
        to_owned![devices];
        move || async move {
            loop {
                let lst = get_all_device_list().await.unwrap();
                info!("get device list: {:?}", lst);
                devices.set(lst);
                // tokio::time::sleep(std::time::Duration::from_secs(3)).await;
            }
        }
    });

    rsx! {
        // --- 头部 ---
        div { class: "mb-8 flex justify-between items-center",
            div { class: "flex gap-3 items-center",
                h1 { class: "text-2xl font-bold", "附近的蓝牙设备" }
                div { class: "badge badge-primary badge-outline text-base text-2xl", "{devices.len()} 在线" }
            }
            // button { class: "btn btn-sm btn-primary btn-outline flex gap-2 items-center",
            //     svg {
            //         class: "h-4 w-4",
            //         fill: "none",
            //         stroke: "currentColor",
            //         stroke_width: "2",
            //         view_box: "0 0 24 24",
            //         path {
            //             stroke_linecap: "round",
            //             stroke_linejoin: "round",
            //             d: "M16.023 9.348h4.992v-.001M2.985 19.644v-4.992m0 0h4.992m-4.993 0l3.181 3.183a8.25 8.25 0 0013.803-3.7M4.031 9.865a8.25 8.25 0 0113.803-3.7l3.181 3.182m0-4.991v4.99",
            //         }
            //     }
            //     "刷新"
            // }
        }
        // --- 设备表格区域 ---
        div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",
            {
                devices
                    .iter()
                    .map(|d| {
                        let initial = d
                            .name
                            .chars()
                            .next()
                            .unwrap_or('?')
                            .to_uppercase()
                            .collect::<String>();
                        rsx! {
                            // DaisyUI 卡片
                            div { class: "card bg-base-100 shadow-xl transition-all border border-base-200 hover:shadow-2xl rounded-2xl",
                                div { class: "card-body p-5",
                                    div { class: "flex gap-4 items-center",
                                        // 头像
                                        div { 
                                            class: "avatar h-12 w-12 flex items-center justify-center",
                                            div {
                                                class: "rounded-full bg-primary text-primary-content text-2xl font-bold uppercase select-none flex items-center justify-center leading-none text-center",
                                                style: "height:3rem;width:3rem;line-height:1;display:flex;align-items:center;justify-content:center;",
                                                "{initial}"
                                            }
                                        }
                                        // 设备信息
                                        div { class: "flex-1 min-w-0",
                                            div { class: "flex gap-2 items-center",
                                                span { class: "font-bold text-lg truncate", "{d.name}" }
                                                div { class: "badge badge-info badge-outline badge-sm", "{d.device_type}" }
                                            }
                                            div { class: "flex gap-2 items-center text-gray-500 mt-1 text-sm",
                                                span { class: "flex gap-1 items-center",
                                                    svg {
                                                        class: "h-4 w-4 text-blue-500",
                                                        fill: "none",
                                                        stroke: "currentColor",
                                                        stroke_width: "2",
                                                        view_box: "0 0 24 24",
                                                        path {
                                                            stroke_linecap: "round",
                                                            stroke_linejoin: "round",
                                                            d: "M3.75 13.5l10.5-11.25L12 10.5h8.25L9.75 21.75 12 13.5H3.75z",
                                                        }
                                                    }
                                                    "{d.rssi} dBm"
                                                }
                                                span { class: "opacity-60", "|" }
                                                span { "{d.mac}" }
                                            }
                                        }
                                        // 右侧状态/操作
                                        div { class: "flex flex-col items-end gap-1 min-w-fit",
                                            // 在线 Badge
                                            div { class: "badge badge-success badge-xs animate-pulse mb-1", "在线" }
                                            button { class: "btn btn-primary btn-sm px-3 flex gap-1 items-center",
                                                svg {
                                                    class: "h-4 w-4",
                                                    fill: "none",
                                                    stroke: "currentColor",
                                                    stroke_width: "2",
                                                    view_box: "0 0 24 24",
                                                    path {
                                                        stroke_linecap: "round",
                                                        stroke_linejoin: "round",
                                                        d: "M13.19 8.688a4.5 4.5 0 011.242 7.244l-4.5 4.5a4.5 4.5 0 01-6.364-6.364l1.757-1.757m13.35-.622l1.757-1.757a4.5 4.5 0 00-6.364-6.364l-4.5 4.5a4.5 4.5 0 001.242 7.244",
                                                    }
                                                }
                                                "选择"
                                            }
                                        }
                                    }
                                    // ---- 进度条等细节区 ----
                                    div { class: "mt-4 grid grid-cols-2 gap-4 items-center",
                                        // --- 进度条区域 ---
                                        div { class: "flex flex-col gap-2",
                                            span { class: "label-text text-xs text-gray-400 mb-1", "信号强度 {d.percent}%" }
                                            progress {
                                                class: "progress progress-primary w-full h-2",
                                                max: "100",
                                                value: "{d.percent}",
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    })
            }
        }
    }
}
