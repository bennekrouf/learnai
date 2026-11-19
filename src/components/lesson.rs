//! The Cours volet: an étape's anecdote, or a rubrique's lesson, with its
//! "Pourquoi ?" buttons, its diagrams and its sources.

use desktop_kit::ui::Theme;
use dioxus::prelude::*;

use super::{etape, etape_label, rubrique, Sel, Tab, Ui};
use crate::content::Source;
use crate::markdown;

#[component]
pub fn Lesson(e: usize, r: Option<usize>) -> Element {
    let ui = use_context::<Ui>();
    let theme = use_context::<Theme>();
    let Some(et) = etape(e) else {
        return rsx! {};
    };
    let ru = r.and_then(|r| rubrique(e, r));
    let light = (theme.light)();

    // Built once per page: the Markdown doesn't change while it is open.
    let html = use_hook(move || {
        let body = match ru {
            Some(ru) => ru.body.as_str(),
            None => et.chapitre.as_ref().map_or("", |c| c.body.as_str()),
        };
        markdown::to_html(body)
    });

    // Once drawn, and again when the theme changes: the diagrams, then the
    // sous-rubrique asked for (diagrams change the page's height first).
    // Scrolls on the first drawing only: a theme change keeps the place.
    let drawn = use_hook(|| std::rc::Rc::new(std::cell::Cell::new(false)));
    use_effect(move || {
        let light = (theme.light)();
        let anchor = (ui.anchor)().unwrap_or_default();
        let scroll = !drawn.replace(true);
        let script = format!(
            "requestAnimationFrame(async () => {{
                const root = document.querySelector('.lesson');
                if (root && window.mermaid) {{
                    try {{
                        mermaid.initialize({{ startOnLoad: false, securityLevel: 'strict', theme: {theme} }});
                        await mermaid.run({{ nodes: root.querySelectorAll('pre.mermaid') }});
                    }} catch (e) {{ console.warn(e); }}
                }}
                if (!{scroll}) return;
                const target = {anchor:?} ? document.getElementById({anchor:?}) : null;
                const scroller = document.querySelector('.main-col .main-scroll');
                if (target) target.scrollIntoView({{ block: 'start' }});
                else if (scroller) scroller.scrollTop = 0;
            }});",
            theme = if light { "'default'" } else { "'dark'" },
        );
        document::eval(&script);
    });

    let (eyebrow, title, lead, sources) = match ru {
        Some(ru) => (
            format!("{} · {}", etape_label(et), et.titre),
            ru.titre.clone(),
            ru.sous_rubriques
                .iter()
                .map(|s| s.titre.as_str())
                .collect::<Vec<_>>()
                .join(" · "),
            ru.sources.clone(),
        ),
        None => (
            etape_label(et),
            et.titre.clone(),
            et.chapitre
                .as_ref()
                .map(|c| c.accroche.clone())
                .unwrap_or_default(),
            et.chapitre
                .as_ref()
                .map(|c| c.sources.clone())
                .unwrap_or_default(),
        ),
    };
    let here = match r {
        Some(r) => Sel::Rubrique(e, r),
        None => Sel::Etape(e),
    };
    let first = et.rubriques.first().map(|x| x.titre.clone());

    rsx! {
        div { class: "col-head",
            div { class: "col-head-main",
                div { class: "col-title", "{title}" }
                div { class: "col-sub", "{eyebrow}" }
            }
            if ru.is_some() {
                button {
                    class: "btn btn-small btn-primary",
                    onclick: move |_| ui.go(Tab::Qcm, here),
                    "Faire le QCM"
                }
            }
        }
        div { class: "main-scroll",
            article { class: "page lesson",
                p { class: "eyebrow", "{eyebrow}" }
                h1 { class: "page-title", "{title}" }
                if ru.is_some() {
                    p { class: "lead", "{lead}" }
                } else {
                    blockquote { class: "hook", "{lead}" }
                }
                // Keyed by the theme, so a theme change draws the diagrams
                // again from their source.
                for k in [light] {
                    div { key: "{k}", class: "lesson-body", dangerous_inner_html: "{html}" }
                }
                Sources { sources }
                div { class: "actions",
                    match ru {
                        Some(_) => rsx! {
                            button {
                                class: "btn btn-primary",
                                onclick: move |_| ui.go(Tab::Qcm, here),
                                "Faire le QCM de cette rubrique"
                            }
                            if here.next().is_some() {
                                button {
                                    class: "btn",
                                    onclick: move |_| ui.go_next(here),
                                    "Suite \u{2192}"
                                }
                            }
                        },
                        None => rsx! {
                            if let Some(first) = first {
                                button {
                                    class: "btn btn-primary",
                                    onclick: move |_| ui.go(Tab::Qcm, Sel::Rubrique(e, 0)),
                                    "Commencer le QCM : {first} \u{2192}"
                                }
                            }
                        },
                    }
                }
            }
        }
    }
}

#[component]
fn Sources(sources: Vec<Source>) -> Element {
    if sources.is_empty() {
        return rsx! {};
    }
    rsx! {
        section { class: "sources",
            h2 { class: "section-title", "Sources" }
            ul {
                for s in sources {
                    li { key: "{s.url}",
                        // A link, not a button: the webview hands it to the browser.
                        a { href: "{s.url}", target: "_blank", "{s.titre}" }
                    }
                }
            }
        }
    }
}
