use crate::{
    application::{sample_service::ISampleService, view::SampleView},
    di::Deps,
};
use dioxus::prelude::*;
use shaku::HasComponent;
use std::sync::Arc;

#[component]
pub fn Label() -> Element {
    use std::ops::Deref;

    let dps: Signal<Option<Arc<Deps>>> = use_context();
    let samples = use_signal(|| Vec::<SampleView>::new());
    let selected_labels = use_signal(|| Vec::<Option<String>>::new());
    let loading = use_signal(|| false);
    let error = use_signal(|| None::<String>);

    let fetch_and_set_samples = {
        let mut samples = samples.clone();
        let mut selected_labels = selected_labels.clone();
        let dps = dps.clone();
        let mut loading = loading.clone();
        let mut error = error.clone();
        move |count: i32| async move {
            loading.set(true);
            error.set(None);
            let deps = dps.read().deref().clone();
            if let Some(deps) = deps {
                let svc: Arc<dyn ISampleService> = deps.resolve();
                match svc.fetch_latest(count).await {
                    Ok(new_samples) => {
                        selected_labels.set(vec![None; new_samples.len()]);
                        samples.set(new_samples);
                    }
                    Err(e) => error.set(Some(format!("获取失败: {:?}", e))),
                }
            }
            loading.set(false);
        }
    };

    // 初次页面执行一次(拉5条)
    use_future({
        let fetch_and_set_samples = fetch_and_set_samples.clone();
        move || async move {
            fetch_and_set_samples(5).await;
        }
    });

    // 只做高亮，后续补充标注上传
    let mut on_click_label = {
        let mut selected_labels = selected_labels.clone();
        move |idx: usize, label: &'static str| {
            let mut labels = selected_labels.read().clone();
            labels[idx] = Some(label.to_string());
            selected_labels.set(labels);
        }
    };

    let samples = samples.read();
    let selected_labels = selected_labels.read();
    let loading = loading.read();
    let error = error.read();
    rsx! {
        div { class: "min-h-screen bg-gradient-to-br from-cyan-50 to-sky-100 flex flex-col items-center",
            div { class: "w-full max-w-3xl mt-12 mb-8 flex flex-col gap-6",
                div { class: "flex justify-between items-center bg-white/90 rounded-3xl shadow-md p-6 mb-6",
                    h1 { class: "text-2xl font-bold text-cyan-700 tracking-wide", "批量样本标注" }
                }
                if *loading {
                    div { class: "flex flex-col items-center gap-2 justify-center my-12 py-12",
                        svg {
                            class: "animate-spin h-8 w-8 text-cyan-500 mb-3",
                            fill: "none", stroke: "currentColor", stroke_width: "2", view_box: "0 0 24 24",
                            path { d: "M12 19V4M6.343 17.657A8 8 0 1 1 19.657 6.343",
                                stroke_linecap: "round", stroke_linejoin: "round"
                            }
                        }
                        span { class: "text-sky-700 text-lg tracking-wide", "加载/处理中 ..." }
                    }
                } else if let Some(msg) = &*error {
                    div { class: "text-red-700 text-base text-center bg-red-50 p-3 rounded-xl shadow font-semibold", "出错：{msg}" }
                } else if samples.is_empty() {
                    div { class: "text-gray-400 text-center text-lg py-12", "没有可标注样本" }
                } else {
                    for (idx, s) in samples.iter().enumerate() {
                        div { class: "mb-10 bg-white/90 rounded-xl shadow-lg p-7 transition-shadow hover:shadow-2xl",
                            div {
                                class: "flex justify-between items-center mb-1 text-xs text-gray-400",
                                div { "设备 {s.device}" }
                                div { "创建: {s.created_at}" }
                            }
                            div { class: "font-mono text-base mb-1 text-cyan-900 tracking-wide", "ID: {s.id}" }
                            div { class: "flex flex-wrap gap-2 mb-3",
                                for x in s.sample.iter() {
                                    span { class: "px-2 py-1 rounded-md bg-cyan-100 text-cyan-700 font-mono text-xs border border-cyan-200 shadow-sm", "{x}" }
                                }
                            }
                            div { class: "flex flex-row gap-4 mt-4 mb-1 justify-center items-center",
                                for (btn_label, color) in [("靠近", "#0ea5e9"), ("远离", "#f59e42"), ("静止", "#16a34a")] {
                                    button {
                                        class: "rounded-xl px-7 py-2 text-base font-bold border-2 border-transparent shadow hover:border-cyan-500 hover:bg-cyan-50
                                                transition-colors outline-none focus:ring-2 focus:ring-cyan-400
                                                ",
                                        style: if selected_labels.get(idx).and_then(|v| v.as_deref()) == Some(btn_label) {
                                            format!("background:{color};color:white;border-color:{color};")
                                        } else {
                                            "".to_string()
                                        },
                                        disabled: *loading,
                                        onclick: move |_| on_click_label(idx, btn_label),
                                        "{btn_label}"
                                    }
                                }
                            }
                            if let Some(label) = &selected_labels[idx] {
                                div { class: "mt-3 text-cyan-700 text-lg text-center font-semibold", "人工标注: {label}" }
                            }
                        }
                    }
                }
            }
        }
    }
}
