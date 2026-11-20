//! A rubrique's QCM: one question at a time. Once answered, the right choice
//! shows, the "Pourquoi ?" button unfolds the reason, and a link leads to the
//! part of the lesson it comes from.

use dioxus::prelude::*;

use super::{etape_label, etape_num, FallbackNote, Sel, Tab, Ui};
use crate::progress::Answer;

const LETTERS: [&str; 4] = ["A", "B", "C", "D"];

#[component]
pub fn Quiz(e: usize, r: usize) -> Element {
    let ui = use_context::<Ui>();
    let w = ui.words();
    let (Some(et), Some(ru)) = (ui.etape(e), ui.rubrique(e, r)) else {
        return rsx! {};
    };
    let total = ru.qcm.len();
    // Picks up at the first question left; past the last one, the result.
    let mut index = use_signal(|| ui.progress.peek().next_open(ru).unwrap_or(total));
    let mut why = use_signal(|| false);

    let mut progress = ui.progress;
    let score = progress.read().score(ru);
    let i = index();

    let head = rsx! {
        div { class: "col-head",
            div { class: "col-head-main",
                div { class: "col-title", "{ru.titre}" }
                div { class: "col-sub", "{etape_label(w, et)} · {et.titre}" }
            }
            span { class: "score", title: w.score_hint,
                "{score.correct} / {score.answered}"
            }
            button {
                class: "btn btn-small",
                onclick: move |_| ui.go(Tab::Cours, Sel::Rubrique(e, r)),
                {w.tab_lesson}
            }
        }
        div { class: "strip",
            for (n, q) in ru.qcm.iter().enumerate() {
                {
                    let state = match progress.read().answers.get(&ru.key(q)) {
                        Some(a) if a.correct => "pip pip-good",
                        Some(_) => "pip pip-bad",
                        None => "pip",
                    };
                    let current = if n == i { " pip-on" } else { "" };
                    rsx! {
                        button {
                            key: "{q.id}",
                            class: "{state}{current}",
                            title: "Question {n + 1}",
                            onclick: move |_| {
                                index.set(n);
                                why.set(false);
                            },
                        }
                    }
                }
            }
            button {
                class: if i >= total { "pip pip-end pip-on" } else { "pip pip-end" },
                title: w.result,
                onclick: move |_| index.set(total),
                "\u{2211}"
            }
        }
    };

    // ── The result ──────────────────────────────────────────────────────
    let Some(q) = ru.qcm.get(i) else {
        let missed: Vec<_> = ru
            .qcm
            .iter()
            .enumerate()
            .filter_map(|(n, q)| {
                let a = progress.read().answers.get(&ru.key(q)).copied();
                match a {
                    Some(a) if a.correct => None,
                    _ => Some((n, q, a)),
                }
            })
            .collect();
        let next_label = match Sel::Rubrique(e, r).next() {
            Some(Sel::Etape(n)) => ui.etape(n).map(|x| w.etape_titled(etape_num(x), &x.titre)),
            Some(Sel::Rubrique(ne, nr)) => ui.rubrique(ne, nr).map(|x| w.next_rubrique(&x.titre)),
            None => None,
        };
        return rsx! {
            {head}
            div { class: "main-scroll",
                div { class: "page",
                    p { class: "eyebrow", {w.result} }
                    h1 { class: "page-title", "{score.correct} / {total}" }
                    if score.answered < total {
                        p { class: "lead", {w.left(total - score.answered)} }
                    } else if score.correct == total {
                        p { class: "lead", {w.all_right} }
                    } else {
                        p { class: "lead", {w.to_review} }
                    }
                    if !missed.is_empty() {
                        div { class: "missed",
                            for (n, mq, a) in missed {
                                {
                                    let sous = ru.sous_rubrique(&mq.sous_rubrique).map(|s| s.titre.clone()).unwrap_or_default();
                                    let anchor = mq.sous_rubrique.clone();
                                    let right = mq.choix.get(mq.reponse).cloned().unwrap_or_default();
                                    rsx! {
                                        div { key: "{mq.id}", class: "missed-row",
                                            button {
                                                class: "missed-q",
                                                onclick: move |_| {
                                                    index.set(n);
                                                    why.set(false);
                                                },
                                                span { class: "missed-num", "{n + 1}" }
                                                span { "{mq.question}" }
                                            }
                                            div { class: "missed-answer",
                                                if a.is_some() {
                                                    {w.right_answer(&right)}
                                                } else {
                                                    {w.not_answered}
                                                }
                                            }
                                            button {
                                                class: "link",
                                                onclick: move |_| ui.read_at(e, r, &anchor),
                                                {w.review_in_lesson(&sous)}
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    div { class: "actions",
                        if let Some(label) = next_label {
                            button {
                                class: "btn btn-primary",
                                onclick: move |_| ui.go_next(Sel::Rubrique(e, r)),
                                "{label} \u{2192}"
                            }
                        }
                        button {
                            class: "btn",
                            onclick: move |_| ui.go(Tab::Cours, Sel::Rubrique(e, r)),
                            {w.read_lesson}
                        }
                        button {
                            class: "btn btn-ghost",
                            title: w.restart_hint,
                            onclick: move |_| {
                                progress.write().reset(ru);
                                index.set(0);
                                why.set(false);
                            },
                            {w.restart}
                        }
                    }
                }
            }
        };
    };

    // ── One question ────────────────────────────────────────────────────
    let key = ru.key(q);
    let answer = progress.read().answers.get(&key).copied();
    let sous = ru
        .sous_rubrique(&q.sous_rubrique)
        .map(|s| s.titre.clone())
        .unwrap_or_default();
    let anchor = q.sous_rubrique.clone();
    let reponse = q.reponse;
    let last = i + 1 >= total;
    let open = why();

    rsx! {
        {head}
        div { class: "main-scroll",
            div { class: "page",
                FallbackNote { show: ru.fallback }
                div { class: "q-meta",
                    span { {w.question_of(i + 1, total)} }
                    span { class: "chip", "{sous}" }
                }
                h1 { class: "q-text", "{q.question}" }
                div { class: "choices",
                    for (c, text) in q.choix.iter().enumerate() {
                        {
                            let class = match answer {
                                None => "choice",
                                Some(_) if c == reponse => "choice choice-right",
                                Some(a) if a.choice == c => "choice choice-wrong",
                                Some(_) => "choice choice-dim",
                            };
                            let key = key.clone();
                            let letter = LETTERS.get(c).copied().unwrap_or("?");
                            rsx! {
                                button {
                                    key: "{c}",
                                    class,
                                    disabled: answer.is_some(),
                                    onclick: move |_| {
                                        progress.write().answers.insert(
                                            key.clone(),
                                            Answer { choice: c, correct: c == reponse },
                                        );
                                    },
                                    span { class: "choice-letter", "{letter}" }
                                    span { class: "choice-text", "{text}" }
                                }
                            }
                        }
                    }
                }
                if let Some(a) = answer {
                    div { class: if a.correct { "verdict verdict-good" } else { "verdict verdict-bad" },
                        if a.correct { {w.right} } else { {w.wrong} }
                    }
                    div { class: "why-box",
                        button {
                            class: if open { "why-btn why-btn-on" } else { "why-btn" },
                            onclick: move |_| why.toggle(),
                            if open { {w.why_hide} } else { {w.why} }
                        }
                        if open {
                            div { class: "why-text",
                                for line in split_sentences(&q.pourquoi) {
                                    p { "{line}" }
                                }
                            }
                        }
                    }
                    div { class: "actions",
                        button {
                            class: "btn btn-primary",
                            onclick: move |_| {
                                index.set(i + 1);
                                why.set(false);
                            },
                            if last { {w.see_result} } else { {w.next_question} }
                        }
                        button {
                            class: "link",
                            onclick: move |_| ui.read_at(e, r, &anchor),
                            {w.see_in_lesson(&sous)}
                        }
                    }
                } else {
                    p { class: "hint", {w.pick_one} }
                }
            }
        }
    }
}

/// The reason, one short sentence per line, as it was written.
fn split_sentences(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut current = String::new();
    let mut chars = text.trim().chars().peekable();
    while let Some(ch) = chars.next() {
        current.push(ch);
        if matches!(ch, '.' | '?' | '!') && chars.peek() == Some(&' ') {
            // "p. ex." or "1. " are not sentence ends: the next word starts
            // with a capital when a new sentence begins.
            let mut ahead = chars.clone();
            ahead.next();
            if ahead.peek().is_some_and(|c| c.is_uppercase() || *c == '«') {
                out.push(current.trim().to_owned());
                current.clear();
            }
        }
    }
    if !current.trim().is_empty() {
        out.push(current.trim().to_owned());
    }
    out
}

#[cfg(test)]
mod tests {
    use super::split_sentences;

    #[test]
    fn sentences_split_on_capitals() {
        assert_eq!(
            split_sentences("Un chiffre se compare. Le code est rapide, p. ex. ici. Fin."),
            vec![
                "Un chiffre se compare.",
                "Le code est rapide, p. ex. ici.",
                "Fin."
            ]
        );
    }
}
