use dioxus::prelude::*;
use tracing::info;

use crate::routes::Route;

// 菜单项组件
#[derive(Props, Clone, PartialEq, Debug)]
pub struct MenuItem {
    pub name: String,
    pub icon: String,
    pub route: Route,
}

#[derive(Props, Clone, PartialEq, Debug)]
pub struct MenuProps {
    pub items: Vec<MenuItem>,
    pub current_route: Route,
}

pub fn parse_svg(svg: &str) -> Element {
    match svg {
        "home" => rsx! {
            svg {
                class: "h-5 w-5",
                fill: "none",
                stroke: "currentColor",
                view_box: "0 0 24 24",
                xmlns: "http://www.w3.org/2000/svg",
                path {
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    stroke_width: "2",
                    d: "M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6",
                }
            }
        },
        "device" => rsx! {
            svg {
                class: "h-5 w-5",
                fill: "none",
                stroke: "currentColor",
                view_box: "0 0 24 24",
                xmlns: "http://www.w3.org/2000/svg",
                path {
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    stroke_width: "2",
                    d: "M9 3v2m6-2v2M9 19v2m6-2v2M5 9H3m2 6H3m18-6h-2m2 6h-2M7 19h10a2 2 0 002-2V7a2 2 0 00-2-2H7a2 2 0 00-2 2v10a2 2 0 002 2zM9 9h6v6H9V9z",
                }
            }
        },
        "info" => rsx! {
            svg {
                class: "h-5 w-5",
                fill: "none",
                stroke: "currentColor",
                view_box: "0 0 24 24",
                xmlns: "http://www.w3.org/2000/svg",
                path {
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    stroke_width: "2",
                    d: "M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z",
                }
            }
        },
        _ => rsx! {
            svg {
                class: "h-5 w-5",
                view_box: "0 0 24 24",
                fill: "currentColor",
                path { d: "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8 8 3.59 8 8-3.59 8-8 8z" }
            }
        },
    }
}
#[component]
pub fn Menu(props: MenuProps) -> Element {
    info!("Rendering menu with items: {:?}", props.items);
    rsx! {
        ul { class: "menu menu-lg bg-base-200 rounded-box",
            {
                props
                    .items
                    .iter()
                    .map(|item| {
                        let is_active = props.current_route == item.route;
                        info!("item router: {:?}", item.route);
                        rsx! {
                            li { key: "{item.name}",
                                Link { to: item.route.clone(), class: if is_active { "menu-active" } else { "" },
                                    {parse_svg(&item.icon)}
                                    span { class: "ml-2", "{item.name}" }
                                }
                            }
                        }
                    })
            }
        }
    }
}
