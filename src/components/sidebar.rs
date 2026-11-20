//! The course down the left: each étape, and under the open one its
//! rubriques, with how many questions are answered.

use dioxus::prelude::*;

use super::{Sel, Tab, Ui};
use crate::progress::Score;

#[component]
pub fn Sidebar() -> Element {
    let ui = use_context::<Ui>();
    let sel = (ui.sel)();
    let tab = (ui.tab)();
    let progress = ui.progress.read();
    let course = ui.course();
    let w = ui.words();

    let scores: Vec<Vec<Score>> = course
        .etapes
        .iter()
        .map(|et| et.rubriques.iter().map(|r| progress.score(r)).collect())
        .collect();
    let all = scores
        .iter()
        .flatten()
        .fold(Score::default(), |acc, s| acc.add(*s));
    let pct = all.percent();

    rsx! {
        aside { class: "sidebar",
            div { class: "sidebar-head",
                span { class: "sidebar-title", "{course.titre}" }
                div { class: "meter", title: w.meter(all.answered, all.total),
                    div { class: "meter-fill", style: "width: {pct}%" }
                }
                div { class: "sidebar-sub", {w.questions_count(all.answered, all.total, all.correct)} }
            }
            div { class: "sidebar-list",
                for (e, et) in course.etapes.iter().enumerate() {
                    {
                        let s = scores
                            .get(e)
                            .map(|v| v.iter().fold(Score::default(), |acc, x| acc.add(*x)))
                            .unwrap_or_default();
                        let open = sel.etape() == e;
                        let on = sel == Sel::Etape(e);
                        let num = et.id.parse::<u32>().unwrap_or(0);
                        let accroche = et.chapitre.as_ref().map(|c| c.accroche.clone()).unwrap_or_default();
                        rsx! {
                            button {
                                key: "e{e}",
                                class: if on { "sidebar-row sidebar-row-on" } else if open { "sidebar-row sidebar-row-open" } else { "sidebar-row" },
                                title: "{et.titre}",
                                onclick: move |_| {
                                    // Staying in the volet in use: the anecdote in Cours,
                                    // the étape's rubriques in QCM.
                                    ui.go(tab, Sel::Etape(e));
                                },
                                span { class: if s.done() { "step-num step-done" } else { "step-num" }, "{num}" }
                                div { class: "sidebar-main",
                                    div { class: "sidebar-label", "{et.titre}" }
                                    div { class: "sidebar-sub", "{accroche}" }
                                }
                                span { class: "sidebar-count", title: w.sidebar_answered, "{s.answered}/{s.total}" }
                            }
                            if open {
                                for (r, ru) in et.rubriques.iter().enumerate() {
                                    {
                                        let rs = scores.get(e).and_then(|v| v.get(r)).copied().unwrap_or_default();
                                        let on = sel == Sel::Rubrique(e, r);
                                        let state = if rs.done() && rs.correct == rs.total {
                                            "dot dot-good"
                                        } else if rs.done() {
                                            "dot dot-done"
                                        } else if rs.answered > 0 {
                                            "dot dot-half"
                                        } else {
                                            "dot"
                                        };
                                        rsx! {
                                            button {
                                                key: "r{e}-{r}",
                                                class: if on { "sidebar-row sidebar-sub-row sidebar-row-on" } else { "sidebar-row sidebar-sub-row" },
                                                title: "{ru.titre}",
                                                onclick: move |_| ui.go(tab, Sel::Rubrique(e, r)),
                                                span { class: state }
                                                div { class: "sidebar-main",
                                                    div { class: "sidebar-label sidebar-label-light", "{ru.titre}" }
                                                }
                                                span {
                                                    class: "sidebar-count",
                                                    title: if rs.answered > 0 { w.sidebar_score } else { w.sidebar_questions },
                                                    if rs.answered > 0 { "{rs.correct}/{rs.total}" } else { "{rs.total}" }
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
            div { class: "sidebar-foot",
                if tab == Tab::Qcm {
                    {w.sidebar_foot_quiz}
                } else {
                    {w.sidebar_foot_lesson}
                }
            }
        }
    }
}
