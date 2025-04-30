use dioxus::prelude::*;

use crate::{pages::side_bar::Sidebar, routes::Route};

#[component]
pub fn Layout() -> Element {
    let route = use_route::<Route>();

    rsx! {
        div { class: "flex min-h-screen",
            Sidebar { current_route: route.clone() }
            Outlet::<Route> {}
        }
    }
}
