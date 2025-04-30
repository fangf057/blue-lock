use dioxus::prelude::*;
#[component]
pub fn About() -> Element {
    rsx! {
        div { class: "p-4",
            h1 { class: "text-2xl font-bold mb-4", "About" }
        }
    }
}