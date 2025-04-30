use dioxus::prelude::*;

#[component]
pub fn About() -> Element {
    rsx! {
        // 关键: 高度100%
        div { class: "h-full bg-gray-50 flex items-center justify-center",
            div { class: "max-w-2xl",
                div { class: "flex items-center gap-3 mb-6",
                    svg {
                        class: "h-10 w-10 text-primary",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        xmlns: "http://www.w3.org/2000/svg",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            d: "M16.5 9.4a5.001 5.001 0 00-9 0M12 3v6M12 21v-3M3 12h3m15 0h-3M5.4 16.5L7.8 14.1m10.8 2.4l-2.4-2.4",
                        }
                    }
                    span { class: "text-2xl font-bold text-gray-700", "BLE Unlock" }
                }
                p { class: "text-gray-600 text-lg mb-4", "通过蓝牙技术，实现设备解锁的便捷与高效。" }
                div { class: "text-gray-500 text-sm mt-6 flex flex-col gap-1",
                    span { "版本: v1.0.0" }
                    span { "作者: 小明 / Ming" }
                    a {
                        class: "text-primary underline hover:text-blue-700 mt-1 flex gap-1 items-center",
                        href: "https://github.com/your-github/project",
                        target: "_blank",
                        svg {
                            class: "w-4 h-4 inline",
                            fill: "currentColor",
                            view_box: "0 0 24 24",
                            path { d: "M12 .5C5.8.5.5 5.8.5 12c0 5.1 3.3 9.5 7.9 11.1.6.1.8-.2.8-.5 0-.3 0-1.1 0-2.1-3.2.7-3.8-1.5-3.8-1.5-.5-1.3-1.2-1.6-1.2-1.6-1-.7.1-.7.1-.7 1.1.1 1.7 1.2 1.7 1.2 1 .1 1.6-1.3 1.6-1.3.9-1.6 2.3-1.2 2.8-.9.1-.6.4-1.2.7-1.4-2.6-.3-5.3-1.3-5.3-5.5 0-1.2.5-2.1 1.2-2.8-.1-.3-.5-1.5.1-3.2 0 0 .9-.3 3.1 1.1a10.7 10.7 0 012.8-.4c.9 0 1.8.1 2.8.4 2.2-1.4 3.1-1.1 3.1-1.1.6 1.7.2 2.9.1 3.2.8.8 1.2 1.7 1.2 2.8 0 4.2-2.7 5.1-5.3 5.5.4.3.7.9.7 1.8 0 1.3 0 2.3 0 2.6 0 .3.2.7.8.5C20.7 21.5 24 17.1 24 12c0-6.2-5.3-11.5-12-11.5z" }
                        }
                        "GitHub 项目主页"
                    }
                }
            }
        }
    }
}