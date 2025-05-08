use dioxus::prelude::*;

use crate::{
    components::menu::{Menu, MenuItem},
    routes::Route,
};

#[component]
pub fn Sidebar(current_route: Route) -> Element {
    let MENU_ITEMS: Vec<MenuItem> = vec![
        MenuItem {
            name: "首页".to_owned(),
            icon: "home".to_owned(),
            route: Route::Home,
        },
        MenuItem {
            name: "标注".to_owned(),
            icon: "home".to_owned(),
            route: Route::Label,
        },

        MenuItem {
            name: "设备".to_owned(),
            icon: "device".to_owned(),
            route: Route::Device,
        },
        MenuItem {
            name: "日志".to_owned(),
            icon: "log".to_owned(),
            route: Route::Log,
        },
        MenuItem {
            name: "关于".to_owned(),
            icon: "info".to_owned(),
            route: Route::About,
        },
    ];

    // 动态添加active状态
    let items = MENU_ITEMS
        .iter()
        .map(|item| MenuItem { ..item.clone() })
        .collect::<Vec<_>>();

        rsx! {
            aside { 
                class: "w-32 h-full bg-base-300 flex flex-col items-center",  // 修改这里
                h1 { class: "text-xl font-bold mb-4", "BLE Unlock" }
                Menu { items, current_route: current_route.clone() }
            }
        }
}
