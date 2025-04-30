#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_demo::routes::Route;
use tracing::Level;

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}


#[component]
fn App() -> Element {
    rsx! {
        style { {include_str!("../assets/tailwind.css")} }
        Router::<Route> {  }
    }
}
