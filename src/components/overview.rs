//! An étape in the QCM volet: its hook, and its rubriques to take one by one.

use dioxus::prelude::*;

use super::{etape_label, FallbackNote, Sel, Tab, Ui};

#[component]
pub fn Overview(e: usize) -> Element {
    let ui = use_context::<Ui>();
    let w = ui.words();
    let Some(et) = ui.etape(e) else {
        return rsx! {};
    };
    let progress = ui.progress.read();
    let accroche = et
        .chapitre
        .as_ref()
        .map(|c| c.accroche.clone())
        .unwrap_or_default();
    let label = etape_label(w, et);
    let fallback = et.chapitre.as_ref().is_some_and(|c| c.fallback);

    rsx! {
        div { class: "col-head",
            div { class: "col-head-main",
                div { class: "col-title", "{label} · {et.titre}" }
                div { class: "col-sub", {w.overview_sub} }
            }
        }
        div { class: "main-scroll",
            div { class: "page",
                p { class: "eyebrow", "{label}" }
                h1 { class: "page-title", "{et.titre}" }
                FallbackNote { show: fallback }
                blockquote { class: "hook", "{accroche}" }
                button {
                    class: "btn",
                    onclick: move |_| ui.go(Tab::Cours, Sel::Etape(e)),
                    {w.read_story}
                }
                h2 { class: "section-title", {w.rubriques} }
                div { class: "cards",
                    for (r, ru) in et.rubriques.iter().enumerate() {
                        {
                            let s = progress.score(ru);
                            let action = if s.done() {
                                w.review
                            } else if s.answered > 0 {
                                w.resume
                            } else {
                                w.start
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
                                                {w.answered_of(s.answered, s.total, s.correct)}
                                            } else {
                                                {w.n_questions(s.total)}
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
                                            {w.tab_lesson}
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
