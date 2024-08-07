#![allow(non_snake_case)]
use dioxus::prelude::*;

// TODO: Make className optional
#[component]
pub fn Field<'a>(
    cx: Scope<'a>,
    label: &'a str,
    className: &'a str,
    children: Element<'a>,
    htmlFor: Option<&'a str>,
) -> Element {
    cx.render(rsx!(
        label {
            class: "field {className}",
            "for": htmlFor.unwrap_or(""),
            "{label}",
            children
        }
    ))
}
