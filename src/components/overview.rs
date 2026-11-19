//! An étape in the QCM volet: its hook, and its rubriques to take one by one.

use dioxus::prelude::*;

use super::{etape, etape_label, Sel, Tab, Ui};

#[component]
pub fn Overview(e: usize) -> Element {
    let ui = use_context::<Ui>();
    let Some(et) = etape(e) else {
        return rsx! {};
    };
    let progress = ui.progress.read();
    let accroche = et
        .chapitre
        .as_ref()
        .map(|c| c.accroche.clone())
        .unwrap_or_default();

    rsx! {
        div { class: "col-head",
            div { class: "col-head-main",
                div { class: "col-title", "{etape_label(et)} · {et.titre}" }
                div { class: "col-sub", "Les QCM de l'étape, une rubrique à la fois" }
            }
        }
        div { class: "main-scroll",
            div { class: "page",
                p { class: "eyebrow", "{etape_label(et)}" }
                h1 { class: "page-title", "{et.titre}" }
                blockquote { class: "hook", "{accroche}" }
                button {
                    class: "btn",
                    onclick: move |_| ui.go(Tab::Cours, Sel::Etape(e)),
                    "Lire l'anecdote"
                }
                h2 { class: "section-title", "Rubriques" }
                div { class: "cards",
                    for (r, ru) in et.rubriques.iter().enumerate() {
                        {
                            let s = progress.score(ru);
                            let action = if s.done() {
                                "Revoir"
                            } else if s.answered > 0 {
                                "Continuer"
                            } else {
                                "Commencer"
                            };
                            let width = s.percent();
                            let subs = ru
                                .sous_rubriques
                                .iter()
                                .map(|x| x.titre.as_str())
                                .collect::<Vec<_>>()
                                .join(" · ");
                            rsx! {
                                div { key: "{ru.id}", class: "card",
                                    div { class: "card-main",
                                        div { class: "card-title", "{ru.titre}" }
                                        div { class: "card-sub", "{subs}" }
                                        div { class: "meter meter-small",
                                            div { class: "meter-fill", style: "width: {width}%" }
                                        }
                                        div { class: "card-sub",
                                            if s.answered > 0 {
                                                "{s.answered} / {s.total} répondues · {s.correct} justes"
                                            } else {
                                                "{s.total} questions"
                                            }
                                        }
                                    }
                                    div { class: "card-actions",
                                        button {
                                            class: if s.done() { "btn" } else { "btn btn-primary" },
                                            onclick: move |_| ui.go(Tab::Qcm, Sel::Rubrique(e, r)),
                                            "{action}"
                                        }
                                        button {
                                            class: "btn btn-ghost",
                                            onclick: move |_| ui.go(Tab::Cours, Sel::Rubrique(e, r)),
                                            "Cours"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
