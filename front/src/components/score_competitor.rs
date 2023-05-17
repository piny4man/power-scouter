use dioxus::prelude::*;

#[inline_props]
pub fn ScoreCompetitor(
    cx: Scope,
    gendre: String,
    category: String,
    movements: String,
    unit: String,
    body_weight: f64,
    lifted_weight: f64,
) -> Element {
    cx.render(rsx!(header {
        class: "score-competitor",
        h4 {
            "{gendre} {category} {movements}"
        }
        aside {
            span {
                "Body {body_weight:.2} {unit}"
            }
            span {
                "Lifted {lifted_weight:.2} {unit}"
            }
        }
    }))
}
