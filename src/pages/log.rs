use dioxus::prelude::*;

pub fn Log() -> Element {
    rsx!(
        div {
            class: "flex flex-col items-center justify-center h-screen",
            div {
                class: "text-4xl font-bold mb-4",
                "Welcome to the Log Page!"
            }
            div {
                class: "text-xl",
                "This is where you can view your log."
            }
        }
    )
}