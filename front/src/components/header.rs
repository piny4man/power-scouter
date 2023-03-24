use dioxus::prelude::*;

pub fn Header(cx: Scope) -> Element {
  cx.render(rsx!(header {
    h1 {
        "Power Scouter"
    }
    img {
        src: "/assets/images/scouter.png"
    }
  }))
}
