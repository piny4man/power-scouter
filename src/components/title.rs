#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub fn Title<'a>(cx: Scope<'a>, children: Element<'a>) -> Element {
    cx.render(rsx!(h2 {
        class: "title",
        children
    }))
}
