#![allow(non_snake_case)]
use crate::components::{Button, Field, Header, ScoreBubble, ScoreCompetitor, Title};
use crate::models::{Category, CompetitorInfo, Gendre, Movements, Score, Units};
use dioxus::prelude::*;

// const HOST: &str = "http://127.0.0.1:8000/api";
const HOST: &str = "/api";

fn score_endpoint() -> String {
    format!("{HOST}/score")
}

pub fn home_route(cx: Scope) -> Element {
    let gendre = use_state::<Gendre>(cx, || Gendre::Male);
    let units = use_state::<Units>(cx, || Units::Kg);
    let body_weight = use_state(cx, || "".to_string());
    let lifted_weight = use_state(cx, || "".to_string());
    let category = use_state::<Category>(cx, || Category::Raw);
    let movements = use_state::<Movements>(cx, || Movements::FullMeet);
    let score = use_state::<Option<Score>>(cx, || None);
    let score_history = use_state::<Option<Vec<Score>>>(cx, || None);
    let is_body_weight_numeric = body_weight.get().to_string().parse::<f64>().is_ok();
    let is_lifted_weight_numeric = lifted_weight.get().to_string().parse::<f64>().is_ok();

    let clear_fields = move || {
        gendre.set(Gendre::Male);
        units.set(Units::Kg);
        body_weight.set("".to_string());
        lifted_weight.set("".to_string());
        category.set(Category::Raw);
        movements.set(Movements::FullMeet);
    };

    let clear_history = move |_| {
        score_history.set(None);
        score.set(None);
    };

    let get_score = move |_| {
        let gendre_copy = gendre.get();
        let units_copy = units.get();
        let category_copy = category.get();
        let movements_copy = movements.get();
        let body_weight_copy = body_weight.to_owned().get().to_string();
        let lifted_weight_copy = lifted_weight.to_owned().get().to_string();
        let competitor = CompetitorInfo {
            gendre: gendre_copy.clone(),
            units: units_copy.clone(),
            body_weight: body_weight_copy,
            lifted_weight: lifted_weight_copy,
            category: category_copy.clone(),
            movements: movements_copy.clone(),
        };
        let score = score.to_owned();
        let score_history = score_history.to_owned();
        let mut score_history_values = match score_history.get() {
            Some(score_history) => score_history.to_vec(),
            None => Vec::new(),
        };

        cx.spawn({
            async move {
                let calculated_score = reqwest::Client::new()
                    .post(&score_endpoint())
                    .json(&competitor)
                    .send()
                    .await;

                match calculated_score {
                    Ok(new_score) => {
                        log::info!("Score calculated!");
                        let new_score_data: Score = new_score.json().await.unwrap();
                        score.set(Some(new_score_data.clone()));
                        score_history_values.push(new_score_data);
                        score_history.set(Some(score_history_values.to_vec()))
                    }
                    Err(err) => {
                        log::info!("User creation failed, {err:?}");
                    }
                }
            }
        });
        clear_fields()
    };

    cx.render(rsx! {
        head {
            title {
                "Power Scouter 🐉 Powerlifting scores calculator"
            }
            link {
                rel: "stylesheet",
                href: "/public/styles/index.css"
            }
        }
        body {
            main {
                Header {}
                article { class: "container",
                    section { class: "competitor",
                        header {
                            svg {
                                "fill": "none",
                                "viewBox": "0 0 24 24",
                                "stroke-width": "1.5",
                                "stroke": "currentColor",
                                path {
                                    "stroke-linecap": "round",
                                    "stroke-linejoin": "round",
                                    "d": "M15.75 6a3.75 3.75 0 11-7.5 0 3.75 3.75 0 017.5 0zM4.501 20.118a7.5 7.5 0 0114.998 0A17.933 17.933 0 0112 21.75c-2.676 0-5.216-.584-7.499-1.632z"
                                }
                            }
                        }
                        div {
                            Title { "Competitor information" }
                            section { class: "row",
                                div { class: "column",
                                    h4 { "Gendre" }
                                    Field { className: "radio", label: "Male", htmlFor: "male",
                                        input {
                                            "type": "radio",
                                            id: "male",
                                            name: "gendre",
                                            value: "male",
                                            checked: gendre.to_string() == Gendre::Male.to_string(),
                                            onchange: move |_| gendre.set(Gendre::Male)
                                        }
                                    }
                                    Field { className: "radio", label: "Female", htmlFor: "female",
                                        input {
                                            "type": "radio",
                                            name: "gendre",
                                            id: "female",
                                            value: "female",
                                            checked: gendre.to_string() == Gendre::Female.to_string(),
                                            onchange: move |_| gendre.set(Gendre::Female)
                                        }
                                    }
                                }
                                div { class: "column",
                                    h4 { "Units" }
                                    Field { className: "radio", label: "Kilogram (kg)", htmlFor: "kilograms",
                                        input {
                                            "type": "radio",
                                            name: "units",
                                            id: "kilograms",
                                            value: "kilograms",
                                            checked: units.to_string() == Units::Kg.to_string(),
                                            onchange: move |_| units.set(Units::Kg)
                                        }
                                    }
                                    Field { className: "radio", label: "Pounds (lb)", htmlFor: "pounds",
                                        input {
                                            "type": "radio",
                                            name: "units",
                                            id: "pounds",
                                            value: "pounds",
                                            checked: units.to_string() == Units::Lb.to_string(),
                                            onchange: move |_| units.set(Units::Lb)
                                        }
                                    }
                                }
                            }
                            section { class: "row",
                                div { class: "column  full-width",
                                    h4 { "Weight" }
                                    div { class: "content",
                                        Field { className: "free-text", label: "Body", htmlFor: "body_weight",
                                            input {
                                                "type": "text",
                                                id: "body_weight",
                                                value: "{body_weight}",
                                                class: if !body_weight.get().to_string().is_empty() && !is_body_weight_numeric {
        "invalid"
    } else {
        ""
    },
                                                oninput: move |evt| body_weight.set(evt.value.clone())
                                            }
                                        }
                                        Field { className: "free-text", label: "Lifted", htmlFor: "lifted_weight",
                                            input {
                                                "type": "text",
                                                id: "lifted_weight",
                                                value: "{lifted_weight}",
                                                class: if !lifted_weight.get().to_string().is_empty() && !is_lifted_weight_numeric {
        "invalid"
    } else {
        ""
    },
                                                oninput: move |evt| lifted_weight.set(evt.value.clone())
                                            }
                                        }
                                    }
                                }
                            }
                            section { class: "row",
                                div { class: "column",
                                    h4 { "Category" }
                                    Field { className: "radio", label: "Raw/Classic", htmlFor: "raw",
                                        input {
                                            "type": "radio",
                                            name: "category",
                                            id: "raw",
                                            value: "raw",
                                            checked: category.to_string() == Category::Raw.to_string(),
                                            onchange: move |_| category.set(Category::Raw)
                                        }
                                    }
                                    Field { className: "radio", label: "Equipped", htmlFor: "equipped",
                                        input {
                                            "type": "radio",
                                            name: "category",
                                            id: "equipped",
                                            value: "equipped",
                                            checked: category.to_string() == Category::Equipped.to_string(),
                                            onchange: move |_| category.set(Category::Equipped)
                                        }
                                    }
                                }
                                div { class: "column",
                                    h4 { "Movements" }
                                    Field { className: "radio", label: "Full meet", htmlFor: "fullmeet",
                                        input {
                                            "type": "radio",
                                            name: "movements",
                                            id: "fullmeet",
                                            value: "fullmeet",
                                            checked: movements.to_string() == Movements::FullMeet.to_string(),
                                            onchange: move |_| movements.set(Movements::FullMeet)
                                        }
                                    }
                                    Field { className: "radio", label: "Bench only", htmlFor: "bench",
                                        input {
                                            "type": "radio",
                                            name: "movements",
                                            id: "bench",
                                            value: "bench",
                                            checked: movements.to_string() == Movements::BenchOnly.to_string(),
                                            onchange: move |_| movements.set(Movements::BenchOnly)
                                        }
                                    }
                                }
                            }
                            section { class: "row",
                                Button { on_click: get_score, "Calculate" }
                            }
                        }
                    }
                    section { class: "results",
                        header {
                            svg {
                                "fill": "none",
                                "viewBox": "0 0 24 24",
                                "stroke-width": "1.5",
                                "stroke": "currentColor",
                                path {
                                    "stroke-linecap": "round",
                                    "stroke-linejoin": "round",
                                    "d": "M16.5 18.75h-9m9 0a3 3 0 013 3h-15a3 3 0 013-3m9 0v-3.375c0-.621-.503-1.125-1.125-1.125h-.871M7.5 18.75v-3.375c0-.621.504-1.125 1.125-1.125h.872m5.007 0H9.497m5.007 0a7.454 7.454 0 01-.982-3.172M9.497 14.25a7.454 7.454 0 00.981-3.172M5.25 4.236c-.982.143-1.954.317-2.916.52A6.003 6.003 0 007.73 9.728M5.25 4.236V4.5c0 2.108.966 3.99 2.48 5.228M5.25 4.236V2.721C7.456 2.41 9.71 2.25 12 2.25c2.291 0 4.545.16 6.75.47v1.516M7.73 9.728a6.726 6.726 0 002.748 1.35m8.272-6.842V4.5c0 2.108-.966 3.99-2.48 5.228m2.48-5.492a46.32 46.32 0 012.916.52 6.003 6.003 0 01-5.395 4.972m0 0a6.726 6.726 0 01-2.749 1.35m0 0a6.772 6.772 0 01-3.044 0"
                                }
                            }
                        }
                        div { class: "scores",
                            Title { "Results" }
                            div { class: "score-container",
                                match score.get() {
                                    Some(res) => cx.render(rsx! {
                                        ScoreCompetitor {
                                            gendre: res.gendre.clone().to_string(),
                                            category: res.category.clone().to_string(),
                                            movements: res.movements.clone().to_string(),
                                            unit: res.unit.clone().to_string(),
                                            body_weight: res.clone().body_weight,
                                            lifted_weight: res.clone().lifted_weight,
                                        }
                                        div {
                                            class: "score-row",
                                            ScoreBubble {
                                                label: "IPF GL:",
                                                score: &res.ipfgl,
                                            }
                                            ScoreBubble {
                                                label: "IPF:",
                                                score: &res.ipf,
                                            }
                                            ScoreBubble {
                                                label: "Wilks:",
                                                score: &res.new_wilks,
                                            }
                                            ScoreBubble {
                                                label: "Old Wilks:",
                                                score: &res.old_wilks,
                                            }
                                            ScoreBubble {
                                                label: "DOTS:",
                                                score: &res.dots,
                                            }
                                        }
                                    }),
                                    _ => cx.render(rsx! {
                                        "No score"
                                    })
                                }
                            }
                        }
                        div { class: "history",
                            header {
                                Title { "Recent results" }
                                Button { on_click: clear_history, "Clear history" }
                            }
                            if let Some(score_history) = score_history.get() {
                                rsx!(
                                    ul {
                                        {score_history.iter().map(|item| {
                                            rsx!(
                                                li {
                                                    class: "score-container",
                                                    ScoreCompetitor {
                                                        gendre: item.gendre.clone().to_string(),
                                                        category: item.category.clone().to_string(),
                                                        movements: item.movements.clone().to_string(),
                                                        unit: item.unit.clone().to_string(),
                                                        body_weight: item.clone().body_weight,
                                                        lifted_weight: item.clone().lifted_weight,
                                                    }
                                                    div {
                                                        class: "score-row",
                                                        ScoreBubble {
                                                            label: "IPF GL:",
                                                            score: &item.ipfgl,
                                                        }
                                                        ScoreBubble {
                                                            label: "IPF:",
                                                            score: &item.ipf,
                                                        }
                                                        ScoreBubble {
                                                            label: "Wilks:",
                                                            score: &item.new_wilks,
                                                        }
                                                        ScoreBubble {
                                                            label: "Old Wilks:",
                                                            score: &item.old_wilks,
                                                        }
                                                        ScoreBubble {
                                                            label: "DOTS:",
                                                            score: &item.dots,
                                                        }
                                                    }
                                                }
                                            )
                                        })}
                                    }
                                )
                            } else {
                                rsx!("There is no score history yet")
                            }
                        }
                    }
                }
            }
        }
    })
}
