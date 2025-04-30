use dioxus::prelude::*;

use crate::{pages::side_bar::Sidebar, routes::Route};

#[component]
pub fn Layout() -> Element {
    let route = use_route::<Route>();

    rsx! {
        div { class: "flex h-screen", // 最外层是 h-screen
            Sidebar { current_route: route.clone() }
            div { class: "flex-1 h-full overflow-auto",
                Outlet::<Route> {}
            }
        }
    }
}
