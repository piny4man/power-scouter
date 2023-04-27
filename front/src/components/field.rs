use dioxus::prelude::*;

#[inline_props]
pub fn Field<'a>(cx: Scope<'a>, label: &'a str, className: &'a str, children: Element<'a>) -> Element {
  cx.render(rsx!(
    label {
      class: "{className}",
      children,
      "{label}"
    }
  ))
}