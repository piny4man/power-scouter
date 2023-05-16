use dioxus::prelude::*;

#[inline_props]
pub fn ScoreCompetitor<'a>(
    cx: Scope<'a>,
    gendre: String,
    category: &'a str,
    movements: &'a str,
    unit: &'a str,
    body_weight: &'a f64,
    lifted_weight: &'a f64,
) -> Element {
    cx.render(rsx!(header {
        class: "score-competitor",
        span {
            "{gendre} {category} {movements} {body_weight:.2}{unit} {lifted_weight:.2}{unit}"
        }
    }))
}
