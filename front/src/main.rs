#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::{prelude::*};

fn main() {
    dioxus_web::launch(App);
}

// create a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {
    let body_weight = use_state(cx, || "".to_string());
    let lifted_weight = use_state(cx, || "".to_string());

    cx.render(rsx! {
        style { include_str!("./styles.css") }
        main {
            header {
                // img {
                //     src: "/assets/images/powerlifting.svg"
                // }
                h1 {
                    "Power Scouter"
                }
                img {
                    src: "/assets/images/scouter.png"
                }
            }
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
                        div {
                            class: "row",
                            h4 {
                                "Gendre"
                            }
                            label {
                                input {
                                    "type": "radio",
                                    name: "gendre",
                                    value: "male"
                                }
                                "Male"
                            }
                            label {
                                input {
                                    "type": "radio",
                                    name: "gendre",
                                    value: "female"
                                }
                                "Female"
                            }
                        }
                        div {
                            class: "row",
                            h4 {
                                "Units"
                            }
                            label {
                                input {
                                    "type": "radio",
                                    name: "units",
                                    value: "kilograms"
                                }
                                "Kilogram (kg)"
                            }
                            label {
                                input {
                                    "type": "radio",
                                    name: "units",
                                    value: "pounds"
                                }
                                "Pounds (lb)"
                            }
                        }
                        div {
                            class: "row",
                            h4 {
                                "Weight"
                            }
                            label {
                                "Body"
                                input {
                                    "type": "text",
                                    value: "{body_weight}",
                                    oninput: move |evt| body_weight.set(evt.value.clone())
                                    // value: "kilograms"
                                }
                            }
                            label {
                                "Lifted"
                                input {
                                    "type": "text",
                                    value: "{lifted_weight}",
                                    oninput: move |evt| lifted_weight.set(evt.value.clone())
                                    // value: "pounds"
                                }
                            }
                        }
                        div {
                            class: "row",
                            h4 {
                                "Category"
                            }
                            label {
                                input {
                                    "type": "radio",
                                    name: "category",
                                    value: "raw"
                                }
                                "Raw/Classic"
                            }
                            label {
                                input {
                                    "type": "radio",
                                    name: "category",
                                    value: "equipped"
                                }
                                "Equipped"
                            }
                        }
                        div {
                            class: "row",
                            h4 {
                                "Movements"
                            }
                            label {
                                input {
                                    "type": "radio",
                                    name: "movements",
                                    value: "fullmet"
                                }
                                "Full meet"
                            }
                            label {
                                input {
                                    "type": "radio",
                                    name: "movements",
                                    value: "bench"
                                }
                                "Bench only"
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
                            "{body_weight}"
                        }

                        div {
                            class: "row",
                            "{lifted_weight}"
                        }
                    }
                }
            }
        }
    })
}
