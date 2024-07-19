#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub fn Button<'a>(
    cx: Scope<'a>,
    children: Element<'a>,
    on_click: EventHandler<'a, MouseEvent>,
) -> Element {
    cx.render(rsx!(button {
        class: "button",
        onclick: move |event| on_click.call(event),
        children
    }))
}
