use dioxus::prelude::*;

#[inline_props]
pub fn ScoreBubble<'a>(cx: Scope<'a>, label: &'a str, score: &'a f64) -> Element {
    cx.render(rsx!(
		p {
			class: "score-bubble",
			span {
				"{label}"
			}
			"{score:.2}"
		}
  	))
}
