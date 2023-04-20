use dioxus::prelude::*;

#[inline_props]
pub fn Title<'a>(cx: Scope<'a>, children: Element<'a>) -> Element {
  cx.render(rsx!(
    h2 {
      class: "title",
      children
    }
  ))
}
