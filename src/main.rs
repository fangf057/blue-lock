#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_demo::components::device_list::DeviceList;
use dioxus_html_macro::html;
use tracing::info;
use tracing::Level;
use web_sys::wasm_bindgen::JsCast;

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}

#[component]
fn HtmlComponent() -> Element {
    html!(
        <div class="container mx-auto">"inner contetn"</div>
    )
}

#[component]
fn ChatMessages() -> Element {
    rsx! {
        div {
            class: "chat chat-start",
            div {
                class: "chat-bubble",
                "It's over Anakin,"
                br {}
                "I have the high ground."
            }
        }

        div {
            class: "chat chat-end",
            div {
                class: "chat-bubble",
                "You underestimate my power!"
            }
        }
    }
}

#[component]
fn CardComponent() -> Element {
    rsx! {
        div {
            class: "card bg-base-100 w-96 shadow-sm",

            figure {
                img {
                    src: "https://img.daisyui.com/images/stock/photo-1606107557195-0e29a4b5b4aa.webp",
                    alt: "Shoes"
                }
            }

            div {
                class: "card-body",

                h2 {
                    class: "card-title",
                    "Card Title"
                }

                p {
                    "A card component has a figure, a body part, and inside body there are title and actions parts"
                }

                div {
                    class: "card-actions justify-end",

                    button {
                        class: "btn btn-primary",
                        "Buy Now"
                    }
                }
            }
        }
    }
}
#[component]
fn CodeMockup() -> Element {
    rsx! {
        div {
            class: "mockup-code w-full",

            pre {
                "data-prefix": "1",
                code { "npm i daisyui" }
            }

            pre {
                "data-prefix": "2",
                code { "installing..." }
            }

            pre {
                "data-prefix": "3",
                class: "bg-warning text-warning-content",
                code { "Error!" }
            }
        }
    }
}
#[component]
fn App() -> Element {
    // 设置样式表
    // Build cool things ✌️
    //
    // let mut video_ele = use_signal(|| None);



    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/tailwind.css") }
        div {            class: "container mx-auto max-w-3xl bg-gray-100 p-8",
            DeviceList {  }
         }
        // HtmlComponent {  }
        // CodeMockup {  }
        // ChatMessages {  }

    }
}
