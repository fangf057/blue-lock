use crate::pages::prelude::*;
use dioxus::prelude::*;

// 路由定义
#[derive(Routable, Clone, PartialEq, Debug)]
pub enum Route {
    #[layout(Layout)]
    #[route("/")]
    Home,
    #[route("/device")]
    Device,
    #[route("/about")]
    About,
    #[route("/log")]
    Log,
    #[route("/label")]
    Label,
}
