use dioxus::prelude::*;

use crate::components::device_list::DeviceList;

#[component]
pub fn Device() -> Element {
    rsx! {
        div { class: "p-4 bg-gray-100 min-h-screen w-full",
            // h1 { class: "text-2xl font-bold mb-4", "Device List" }
            // 这里可以添加实际的设备列表组件
            DeviceList {}
        }
    }
}