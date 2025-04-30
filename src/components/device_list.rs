use dioxus::prelude::*;
use tracing::info;

use crate::{ble::service::get_all_device_list, dto::device::Device};

// 可根据实际拓展

#[component]
pub fn DeviceList() -> Element {
    // use_signal
    let mut devices = use_signal(|| Vec::<Device>::new());

    use_future(move || async move {
        loop {
            let lst = get_all_device_list().await.unwrap();
            info!("get device list: {:?}", lst);
            devices.set(lst);
            // tokio::time::sleep(std::time::Duration::from_secs(3)).await;
        }
    });

    rsx! {
        // 设备列表头部
        div { class: "mb-6 flex items-center justify-between",
            div { class: "flex items-center gap-3",
                h1 { class: "text-2xl font-bold text-slate-800", "附近设备" }
                span { class: "rounded-full bg-blue-50 px-2 py-1 text-sm text-blue-600",
                    "{devices.len()} 个设备在线"
                }
            }
            div { class: "flex items-center gap-2",
                button { class: "btn btn-sm btn-primary",
                    svg {
                        class: "h-4 w-4",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        view_box: "0 0 24 24",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            d: "M16.023 9.348h4.992v-.001M2.985 19.644v-4.992m0 0h4.992m-4.993 0l3.181 3.183a8.25 8.25 0 0013.803-3.7M4.031 9.865a8.25 8.25 0 0113.803-3.7l3.181 3.182m0-4.991v4.99",
                        }
                    }
                    "刷新列表"
                }
            }
        }

        // 设备列表容器
        div { class: "space-y-4",
            {devices.iter().map(|d| rsx! {
                div { class: "group relative",
                    div { class: "absolute -inset-0.5 rounded-xl bg-gradient-to-r from-blue-50 to-transparent opacity-0 transition-all group-hover:opacity-100" }
                    div { class: "relative rounded-xl border border-slate-100 bg-white p-4 shadow-[0_2px_12px_rgba(0,0,0,0.04)] transition-all hover:shadow-[0_4px_20px_rgba(0,114,245,0.1)]",
                        div { class: "flex items-center gap-4",
                            // 设备图标
                            div { class: "rounded-lg bg-gradient-to-br from-blue-50 to-white p-2.5 shadow-sm",
                                svg {
                                    class: "h-5 w-5 text-blue-600",
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_width: "1.5",
                                    view_box: "0 0 24 24",
                                    path {
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        d: "M8.288 15.038a5.25 5.25 0 017.424 0M5.106 11.856c3.807-3.808 9.98-3.808 13.788 0M1.042 8.464a13.46 13.46 0 013.521-3.518M21 12a9 9 0 11-18 0 9 9 0 0118 0z",
                                    }
                                }
                            }
                            // 设备信息
                            div { class: "min-w-0 flex-1",
                                div { class: "mb-1 flex items-baseline gap-2",
                                    h3 { class: "truncate text-base font-semibold text-slate-800",
                                        "{d.name}"
                                    }
                                    span { class: "rounded bg-blue-50 px-1.5 py-0.5 text-xs text-blue-600",
                                        "{d.device_type}"
                                    }
                                }
                                div { class: "flex items-center gap-2 text-sm text-slate-500",
                                    span { class: "flex items-center gap-1",
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
                                        span { "{d.rssi} dBm" }
                                    }
                                    span { class: "text-slate-300", "|" }
                                    span { "{d.mac}" }
                                }
                            }
                            // 连接状态和按钮
                            div { class: "flex flex-col items-end gap-2",
                                div { class: "relative",
                                    div { class: "absolute -top-1 -right-1 h-2.5 w-2.5 rounded-full bg-emerald-400 shadow-[0_0_8px_2px_rgba(74,222,128,0.25)]" }
                                    button { class: "btn btn-sm btn-primary px-3",
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
                                        "立即配对"
                                    }
                                }
                                div { class: "flex items-center gap-1.5 text-xs text-slate-500",
                                    span { class: "h-2 w-2 animate-pulse rounded-full bg-emerald-400" }
                                    "{d.signal_text}"
                                }
                            }
                        }
                        // 渐进式细节
                        div { class: "mt-3 border-t border-slate-100 pt-3",
                            div { class: "flex items-center justify-between text-sm",
                                // 信号强度标尺
                                div { class: "max-w-[60%] flex-1",
                                    div { class: "flex items-center gap-2",
                                        div { class: "h-1.5 w-full overflow-hidden rounded-full bg-slate-100",
                                            div { class: format!("relative h-full w-4/5 bg-gradient-to-r {}", d.signal_color),
                                                div { class: "absolute -top-0.5 right-0 h-3 w-1.5 rounded-full bg-blue-600" }
                                            }
                                        }
                                        span { class: "text-sm font-medium text-blue-600", "{d.percent}%" }
                                    }
                                }
                                // 设备属性
                                div { class: "flex items-center gap-3 text-slate-600",
                                    div { class: "flex items-center gap-1",
                                        svg {
                                            class: "h-4 w-4",
                                            fill: "none",
                                            stroke: "currentColor",
                                            stroke_width: "1.8",
                                            view_box: "0 0 24 24",
                                            path {
                                                stroke_linecap: "round",
                                                stroke_linejoin: "round",
                                                d: "M9 12.75L11.25 15 15 9.75M21 12a9 9 0 11-18 0 9 9 0 0118 0z",
                                            }
                                        }
                                        span { "{d.status}" }
                                    }
                                    div { class: "h-4 w-px bg-slate-200" }
                                    div { class: "flex items-center gap-1",
                                        svg {
                                            class: "h-4 w-4",
                                            fill: "none",
                                            stroke: "currentColor",
                                            stroke_width: "1.8",
                                            view_box: "0 0 24 24",
                                            path {
                                                stroke_linecap: "round",
                                                stroke_linejoin: "round",
                                                d: "M12 6v6h4.5m4.5 0a9 9 0 11-18 0 9 9 0 0118 0z",
                                            }
                                        }
                                        span { "{d.last}" }
                                    }
                                }
                            }
                        }
                    }
                }
            })}
        }
    }
}
