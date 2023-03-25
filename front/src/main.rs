#![allow(non_snake_case)]
mod components;
mod models;

use components::Header;
use models::{Category, Gendre, Movements, Units};
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;

fn main() {
    dioxus_web::launch(App);
}

// create a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {
    let gendre = use_state::<Gendre>(cx, || Gendre::Male);
    let units = use_state::<Units>(cx, || Units::Kg);
    let body_weight = use_state(cx, || "".to_string());
    let lifted_weight = use_state(cx, || "".to_string());
    let category = use_state::<Category>(cx, || Category::Raw);
    let movements = use_state::<Movements>(cx, || Movements::FullMeet);
    let is_body_weight_numeric = body_weight.get().to_string().parse::<f64>().is_ok();
    let is_lifted_weight_numeric = lifted_weight.get().to_string().parse::<f64>().is_ok();

    cx.render(rsx! {
        style { include_str!("./styles.css") }
        main {
            Header {},
            article {
                class: "container",
                section {
                    class: "competitor",
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
                        h2 {
                            "Competitor information"
                        }
                        section {
                            class: "row",
                            div {
                                class: "column",
                                h4 {
                                    "Gendre"
                                }
                                label {
                                    class: "radio-label",
                                    input {
                                        "type": "radio",
                                        name: "gendre",
                                        value: "male",
                                        checked: gendre.to_string() == Gendre::Male.to_string(),
                                        onchange: move |_| gendre.set(Gendre::Male)
                                    }
                                    "Male"
                                }
                                label {
                                    class: "radio-label",
                                    input {
                                        "type": "radio",
                                        name: "gendre",
                                        value: "female",
                                        checked: gendre.to_string() == Gendre::Female.to_string(),
                                        onchange: move |_| gendre.set(Gendre::Female)
                                    }
                                    "Female"
                                }
                            }
                            div {
                                class: "column",
                                h4 {
                                    "Units"
                                }
                                label {
                                    class: "radio-label",
                                    input {
                                        "type": "radio",
                                        name: "units",
                                        value: "kilograms",
                                        checked: units.to_string() == Units::Kg.to_string(),
                                        onchange: move |_| units.set(Units::Kg)
                                    }
                                    "Kilogram (kg)"
                                }
                                label {
                                    class: "radio-label",
                                    input {
                                        "type": "radio",
                                        name: "units",
                                        value: "pounds",
                                        checked: units.to_string() == Units::Lb.to_string(),
                                        onchange: move |_| units.set(Units::Lb)
                                    }
                                    "Pounds (lb)"
                                }
                            }
                        }
                        section  {
                            class: "row",
                            div {
                                class: "row",
                                h4 {
                                    "Weight"
                                }
                                label {
                                    class: "free-text-label",
                                    "for": "body_weight",
                                    "Body"
                                }
                                input {
                                    "type": "text",
                                    id: "body_weight",
                                    value: "{body_weight}",
                                    class: if !body_weight.get().to_string().is_empty() && !is_body_weight_numeric { "invalid" } else { "" },
                                    oninput: move |evt| body_weight.set(evt.value.clone())
                                    // value: "kilograms"
                                }
                                label {
                                    class: "free-text-label",
                                    "for": "lifted_weight",
                                    "Lifted"
                                }
                                input {
                                    "type": "text",
                                    id: "lifted_weight",
                                    value: "{lifted_weight}",
                                    class: if !lifted_weight.get().to_string().is_empty() && !is_lifted_weight_numeric { "invalid" } else { "" },
                                    oninput: move |evt| lifted_weight.set(evt.value.clone())
                                    // value: "pounds"
                                }
                            }
                        }
                        section {
                            class: "row",
                            div {
                                class: "column",
                                h4 {
                                    "Category"
                                }
                                label {
                                    class: "radio-label",
                                    input {
                                        "type": "radio",
                                        name: "category",
                                        value: "raw",
                                        checked: category.to_string() == Category::Raw.to_string(),
                                        onchange: move |_| category.set(Category::Raw)
                                    }
                                    "Raw/Classic"
                                }
                                label {
                                    class: "radio-label",
                                    input {
                                        "type": "radio",
                                        name: "category",
                                        value: "equipped",
                                        checked: category.to_string() == Category::Equipped.to_string(),
                                        onchange: move |_| category.set(Category::Equipped)
                                    }
                                    "Equipped"
                                }
                            }
                            div {
                                class: "column",
                                h4 {
                                    "Movements"
                                }
                                label {
                                    class: "radio-label",
                                    input {
                                        "type": "radio",
                                        name: "movements",
                                        value: "fullmeet",
                                        checked: movements.to_string() == Movements::FullMeet.to_string(),
                                        onchange: move |_| movements.set(Movements::FullMeet)
                                    }
                                    "Full meet"
                                }
                                label {
                                    class: "radio-label",
                                    input {
                                        "type": "radio",
                                        name: "movements",
                                        value: "bench",
                                        checked: movements.to_string() == Movements::BenchOnly.to_string(),
                                        onchange: move |_| movements.set(Movements::BenchOnly)
                                    }
                                    "Bench only"
                                }
                            }
                        }
                    }
                }
                section {
                    class: "results",
                    header {
                        svg {
                            "fill": "none",
                            "viewBox": "0 0 24 24",
                            "stroke-width": "1.5",
                            "stroke": "currentColor",
                            path  {
                                "stroke-linecap": "round",
                                "stroke-linejoin": "round",
                                "d": "M16.5 18.75h-9m9 0a3 3 0 013 3h-15a3 3 0 013-3m9 0v-3.375c0-.621-.503-1.125-1.125-1.125h-.871M7.5 18.75v-3.375c0-.621.504-1.125 1.125-1.125h.872m5.007 0H9.497m5.007 0a7.454 7.454 0 01-.982-3.172M9.497 14.25a7.454 7.454 0 00.981-3.172M5.25 4.236c-.982.143-1.954.317-2.916.52A6.003 6.003 0 007.73 9.728M5.25 4.236V4.5c0 2.108.966 3.99 2.48 5.228M5.25 4.236V2.721C7.456 2.41 9.71 2.25 12 2.25c2.291 0 4.545.16 6.75.47v1.516M7.73 9.728a6.726 6.726 0 002.748 1.35m8.272-6.842V4.5c0 2.108-.966 3.99-2.48 5.228m2.48-5.492a46.32 46.32 0 012.916.52 6.003 6.003 0 01-5.395 4.972m0 0a6.726 6.726 0 01-2.749 1.35m0 0a6.772 6.772 0 01-3.044 0"
                            }

                        }
                    }
                    div {
                        h2 {
                            "Results"
                        }
                        div {
                            class: "row",
                            "{gendre}"
                        }
                        div {
                            class: "row",
                            "{units}"
                        }
                        div {
                            class: "row",
                            "{body_weight}"
                        }
                        div {
                            class: "row",
                            "{lifted_weight}"
                        }
                        div {
                            class: "row",
                            "{category}"
                        }
                        div {
                            class: "row",
                            "{movements}"
                        }
                    }
                }
            }
        }
    })
}
