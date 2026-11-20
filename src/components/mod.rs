//! The window under the header: the course down the left, and on the right
//! the étape or rubrique open in one of the two volets, QCM or Cours.

mod lesson;
mod overview;
mod quiz;
mod sidebar;

use dioxus::prelude::*;

use crate::content::{course, course_in, Course, Etape, Rubrique};
use crate::lang::{Lang, Words};
use crate::progress::Progress;

pub use lesson::Lesson;
pub use overview::Overview;
pub use quiz::Quiz;
pub use sidebar::Sidebar;

/// The two volets. QCM comes first: the questions show what is known, the
/// lesson then fills the gaps.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tab {
    Qcm,
    Cours,
}

/// What is open: an étape (its anecdote, or its overview in the QCM volet),
/// or one of its rubriques. Indexes into the course.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Sel {
    Etape(usize),
    Rubrique(usize, usize),
}

impl Sel {
    pub fn etape(self) -> usize {
        match self {
            Sel::Etape(e) | Sel::Rubrique(e, _) => e,
        }
    }

    /// The rubrique after this one, or the next étape's opening.
    pub fn next(self) -> Option<Sel> {
        let etapes = &course().etapes;
        match self {
            Sel::Etape(e) => etapes
                .get(e)
                .filter(|et| !et.rubriques.is_empty())
                .map(|_| Sel::Rubrique(e, 0)),
            Sel::Rubrique(e, r) => {
                let count = etapes.get(e).map_or(0, |et| et.rubriques.len());
                if r + 1 < count {
                    Some(Sel::Rubrique(e, r + 1))
                } else if e + 1 < etapes.len() {
                    Some(Sel::Etape(e + 1))
                } else {
                    None
                }
            }
        }
    }
}

/// What every part of the window reads and changes.
#[derive(Clone, Copy, PartialEq)]
pub struct Ui {
    pub tab: Signal<Tab>,
    pub sel: Signal<Sel>,
    /// The sous-rubrique the lesson scrolls to once it is drawn.
    pub anchor: Signal<Option<String>>,
    pub progress: Signal<Progress>,
    pub lang: Signal<Lang>,
}

impl Ui {
    pub fn new(progress: Signal<Progress>, lang: Signal<Lang>) -> Self {
        let first = first_open(&progress.peek());
        Ui {
            tab: Signal::new(Tab::Qcm),
            sel: Signal::new(first),
            anchor: Signal::new(None),
            progress,
            lang,
        }
    }

    /// The course in the language shown. Reading it redraws the caller when
    /// the language changes.
    pub fn course(self) -> &'static Course {
        course_in((self.lang)())
    }

    pub fn words(self) -> &'static Words {
        (self.lang)().words()
    }

    pub fn etape(self, e: usize) -> Option<&'static Etape> {
        self.course().etapes.get(e)
    }

    pub fn rubrique(self, e: usize, r: usize) -> Option<&'static Rubrique> {
        self.etape(e).and_then(|et| et.rubriques.get(r))
    }

    pub fn go(mut self, tab: Tab, sel: Sel) {
        self.anchor.set(None);
        self.tab.set(tab);
        self.sel.set(sel);
    }

    /// The lesson of a rubrique, at one of its sous-rubriques.
    pub fn read_at(mut self, e: usize, r: usize, sous_rubrique: &str) {
        self.tab.set(Tab::Cours);
        self.sel.set(Sel::Rubrique(e, r));
        self.anchor.set(Some(sous_rubrique.to_owned()));
    }

    /// Where to go after `sel`: a new étape opens on its anecdote, a
    /// rubrique on its QCM.
    pub fn go_next(self, sel: Sel) {
        match sel.next() {
            Some(next @ Sel::Etape(_)) => self.go(Tab::Cours, next),
            Some(next) => self.go(Tab::Qcm, next),
            None => {}
        }
    }
}

/// Where a learner picks up: the first rubrique with a question left, or the
/// start of the course.
fn first_open(progress: &Progress) -> Sel {
    for (e, etape) in course().etapes.iter().enumerate() {
        for (r, rubrique) in etape.rubriques.iter().enumerate() {
            if progress.next_open(rubrique).is_some() {
                // A fresh étape opens on its anecdote.
                let started = etape
                    .rubriques
                    .iter()
                    .any(|ru| progress.score(ru).answered > 0);
                return if started {
                    Sel::Rubrique(e, r)
                } else {
                    Sel::Etape(e)
                };
            }
        }
    }
    Sel::Etape(0)
}

/// The étape's number, from the plan's "03".
pub fn etape_num(et: &Etape) -> u32 {
    et.id.parse::<u32>().unwrap_or(0)
}

/// "Étape 3" or "Step 3".
pub fn etape_label(words: &Words, et: &Etape) -> String {
    words.etape(etape_num(et))
}

/// Says that what is shown is still in French, when it is.
#[component]
pub fn FallbackNote(show: bool) -> Element {
    let ui = use_context::<Ui>();
    let text = ui.words().not_translated;
    if !show || text.is_empty() {
        return rsx! {};
    }
    rsx! {
        p { class: "fallback-note", "{text}" }
    }
}

#[component]
pub fn Workspace() -> Element {
    let ui = use_context::<Ui>();
    let tab = (ui.tab)();
    let sel = (ui.sel)();
    // The zoom applies to the page only: the sidebar and bars keep their size.
    let zoom = f64::from(ui.progress.read().zoom()) / 100.0;
    // A lesson builds its page once: a new language opens it anew.
    let lang = (ui.lang)().code();

    // The left panel's width. While the handle is dragged it follows the
    // pointer here only; the file gets the width once, when it is let go.
    let mut progress = ui.progress;
    let saved = progress
        .read()
        .sidebar
        .unwrap_or(crate::progress::SIDEBAR_DEFAULT);
    let mut dragging = use_signal(|| None::<u32>);
    let width = dragging().unwrap_or(saved);
    let mut finish = move || {
        if let Some(w) = dragging.take() {
            progress.write().sidebar = (w != crate::progress::SIDEBAR_DEFAULT).then_some(w);
        }
    };
    rsx! {
        div {
            class: if dragging().is_some() { "body body-resizing" } else { "body" },
            style: "--sidebar-w: {width}px",
            onmousemove: move |e| {
                if dragging().is_some() {
                    let x = e.client_coordinates().x;
                    dragging.set(Some(crate::progress::clamp_sidebar(x)));
                }
            },
            onmouseup: move |_| finish(),
            onmouseleave: move |_| finish(),
            Sidebar {}
            div {
                class: "resize-handle",
                title: ui.words().resize_hint,
                onmousedown: move |e| {
                    e.prevent_default();
                    dragging.set(Some(width));
                },
                ondoubleclick: move |_| {
                    dragging.set(None);
                    progress.write().sidebar = None;
                },
            }
            main { class: "main-col", style: "--zoom: {zoom}",
                match (tab, sel) {
                    (Tab::Qcm, Sel::Etape(e)) => rsx! { Overview { key: "o{e}", e } },
                    (Tab::Qcm, Sel::Rubrique(e, r)) => rsx! { Quiz { key: "q{e}-{r}", e, r } },
                    (Tab::Cours, Sel::Etape(e)) => rsx! { Lesson { key: "l{e}-{lang}", e, r: None } },
                    (Tab::Cours, Sel::Rubrique(e, r)) => rsx! { Lesson { key: "l{e}-{r}-{lang}", e, r: Some(r) } },
                }
            }
        }
    }
}
