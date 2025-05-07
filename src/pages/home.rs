use std::{borrow::Borrow, ops::Deref};

use dioxus::prelude::*;
use tokio::sync::mpsc::Receiver;
use tracing::info;

use crate::ble::model::Model;

#[component]
pub fn Home() -> Element {
    use std::ops::Deref;
    let sample: Signal<Vec<f32>> = use_context();
    let mut infer_res = use_signal(String::new);

    use_effect(move || {
        let r = sample.read().deref().clone();
        let model = Model::new(include_bytes!(
            "/Users/xlh/Documents/opensource/blue-lock/ai/hybrid_model.onnx"
        ))
        .unwrap();
        if let Ok(res) = model.inference(r) {
            infer_res.set(res.to_string());
        }
    });

    let sample_vec = sample.read();

    rsx! {
        div { class: "min-h-screen bg-gradient-to-tr from-blue-50 to-teal-50 flex items-center justify-center",
            div { class: "bg-white/90 rounded-2xl shadow-xl p-8  max-w-full",
                div { class: "mb-6 flex flex-wrap gap-2",
                    for (i, x) in sample_vec.iter().enumerate() {
                        div { 
                            key: "{i}",
                            class: "inline-block px-3 py-2 rounded-lg bg-blue-100 text-blue-700 font-mono text-base shadow-sm border border-blue-200",
                            "{x}"
                        }
                    }
                }

                div { 
                    class: "mt-4 p-4 rounded-xl border border-teal-100 bg-teal-50/60 flex items-center gap-3",
                    // 可配icon
                    div { 
                        class: "w-full flex justify-center",
                        span { 
                            class: "text-4xl font-semibold text-cyan-800",
                            "{infer_res.read()}"
                        }
                    }
                }
            }
        }
    }
}
