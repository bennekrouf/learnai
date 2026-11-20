//! LearnAI — learn AI systems engineering, the quiz first.
//!
//! The course goes étape by étape. Each étape opens on a true story from AI
//! news or literature, then splits into rubriques. A rubrique is taken in two
//! volets: its QCM first, where every answer has a "Pourquoi ?" to unfold,
//! then its lesson, written in short sentences, with a diagram wherever more
//! than two actors talk to each other.
// Release builds on Windows are GUI programs: no cmd window opens beside the app.
#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

mod components;
mod content;
mod kit;
mod lang;
mod markdown;
mod progress;

use desktop_kit::ui::{use_theme, Banners, Header, SettingsWindow, Toolbar};
use dioxus::desktop::LogicalSize;
use dioxus::prelude::*;

use components::{Tab, Ui, Workspace};
use lang::Lang;

const MAIN_CSS: &str = include_str!("../assets/main.css");

/// Mermaid draws the lessons' diagrams in the page. Version 11.4.1, carried
/// in the app (from npm, via jsDelivr), so diagrams draw offline too.
const MERMAID_JS: &str = include_str!("../assets/vendor/mermaid.min.js");

/// The page's head: the stylesheet, and Mermaid inline.
///
/// The library holds `</script`, `</head` and `</body` in its strings. The
/// first would end the tag early, and Dioxus puts its own start-up script
/// before the first `</head>` and `</body>` it finds in the page, which
/// would land inside Mermaid. Written `<\/…`, they mean the same in the
/// strings and match none of these.
fn custom_head() -> String {
    let js = MERMAID_JS
        .replace("</script", "<\\/script")
        .replace("</head", "<\\/head")
        .replace("</body", "<\\/body");
    format!("<style>{MAIN_CSS}</style><script>{js}</script>")
}

#[cfg(test)]
mod tests {
    #[test]
    fn the_head_leaves_the_page_tags_to_dioxus() {
        let head = super::custom_head();
        assert!(!head.contains("</head") && !head.contains("</body"));
        assert_eq!(head.matches("</script").count(), 1);
        assert!(head.contains("globalThis.mermaid"));
    }
}

/// The native title bar, as in every mayorana app: name and version.
fn window_title() -> String {
    format!("LearnAI {}", env!("CARGO_PKG_VERSION"))
}

fn main() {
    kit::init();

    // The stylesheet goes into the page head once: a `style` element inside
    // `App` would be diffed again on every click.
    let cfg = dioxus::desktop::Config::new()
        .with_data_directory(progress::data_dir().join("webview"))
        .with_custom_head(custom_head())
        .with_window(
            dioxus::desktop::WindowBuilder::new()
                .with_title(window_title())
                .with_inner_size(LogicalSize::new(1240.0, 860.0))
                .with_window_icon(window_icon()),
        );
    LaunchBuilder::desktop().with_cfg(cfg).launch(App);
}

/// The window icon, decoded from the embedded logo — same as GitAgent.
///
/// Downscaled to 64px on the way in: tao hands the OS this single bitmap for
/// every size it needs, and stretching a 1024px source down to a 16px title
/// bar is what makes the icon look muddy.
fn window_icon() -> Option<dioxus::desktop::tao::window::Icon> {
    const ICON_PNG: &[u8] = include_bytes!("../assets/icon.png");
    const SIZE: u32 = 64;

    let img = image::load_from_memory(ICON_PNG).ok()?.resize_exact(
        SIZE,
        SIZE,
        image::imageops::FilterType::Lanczos3,
    );
    dioxus::desktop::tao::window::Icon::from_rgba(img.into_rgba8().into_raw(), SIZE, SIZE).ok()
}

#[component]
fn App() -> Element {
    // The answers live in one signal; the file on disk follows it.
    let progress = use_signal(progress::load);
    use_effect(move || progress::save(&progress.read()));
    // The language picked in Settings, or else the system's, which only the
    // webview knows: it is asked once at start.
    let mut lang = use_signal(|| progress.peek().lang.unwrap_or_default());
    use_future(move || async move {
        if progress.peek().lang.is_some() {
            return;
        }
        let tag = document::eval("return navigator.language || ''")
            .join::<String>()
            .await
            .unwrap_or_default();
        lang.set(Lang::from_tag(&tag));
    });
    let ui = use_context_provider(|| Ui::new(progress, lang));
    let w = lang().words();
    let mut chosen = progress;

    let theme = use_theme();
    use_context_provider(|| theme);
    let mut settings_open = use_signal(|| false);
    let tab = (ui.tab)();
    let mut tab_signal = ui.tab;
    let mut zoom_of = progress;
    let zoom = progress.read().zoom();
    let light = (theme.light)();

    // ⌘+ / ⌘− / ⌘0 (Ctrl on Windows and Linux) zoom the page, as in a
    // browser. The webview reports the keys; the step is taken here.
    use_future(move || async move {
        let mut keys = document::eval(
            "document.addEventListener('keydown', (e) => {
                 if (!(e.metaKey || e.ctrlKey) || e.altKey) return;
                 const step = { '=': 1, '+': 1, '-': -1, '_': -1, '0': 0 }[e.key];
                 if (step === undefined) return;
                 e.preventDefault();
                 dioxus.send(step);
             });
             await new Promise(() => {});",
        );
        while let Ok(step) = keys.recv::<i32>().await {
            zoom_of.write().step_zoom(step);
        }
    });

    rsx! {
        // A newer release and the notice from mayorana.ch. No usage
        // statistics: LearnAI keeps none.
        Banners {}
        Header { on_settings: move |_| settings_open.set(true) }
        Toolbar {
            div { class: "kit-tabs",
                button {
                    class: if tab == Tab::Qcm { "kit-tab kit-on" } else { "kit-tab" },
                    title: w.tab_quiz_hint,
                    onclick: move |_| tab_signal.set(Tab::Qcm),
                    {w.tab_quiz}
                }
                button {
                    class: if tab == Tab::Cours { "kit-tab kit-on" } else { "kit-tab" },
                    title: w.tab_lesson_hint,
                    onclick: move |_| tab_signal.set(Tab::Cours),
                    {w.tab_lesson}
                }
            }
            div { class: "kit-spacer" }
            div { class: "zoom", title: w.zoom_hint,
                button {
                    class: "kit-btn zoom-btn",
                    aria_label: w.zoom_out,
                    disabled: zoom <= progress::ZOOMS[0],
                    onclick: move |_| zoom_of.write().step_zoom(-1),
                    "\u{2212}"
                }
                button {
                    class: "kit-btn zoom-level",
                    title: w.zoom_reset,
                    onclick: move |_| zoom_of.write().step_zoom(0),
                    "{zoom} %"
                }
                button {
                    class: "kit-btn zoom-btn",
                    aria_label: w.zoom_in,
                    disabled: zoom >= progress::ZOOMS[progress::ZOOMS.len() - 1],
                    onclick: move |_| zoom_of.write().step_zoom(1),
                    "+"
                }
            }
            // Always at hand, beside the zoom: the language, and day or night.
            div { class: "lang-switch", role: "group", title: w.settings_language,
                for l in Lang::ALL {
                    button {
                        key: "{l.code()}",
                        class: if lang() == l { "kit-btn lang-btn kit-on" } else { "kit-btn lang-btn" },
                        title: l.name(),
                        aria_pressed: lang() == l,
                        onclick: move |_| {
                            lang.set(l);
                            chosen.write().lang = Some(l);
                        },
                        {l.code().to_uppercase()}
                    }
                }
            }
            button {
                class: "kit-btn theme-btn",
                title: if light { w.theme_dark } else { w.theme_light },
                aria_label: if light { w.theme_dark } else { w.theme_light },
                onclick: move |_| theme.toggle(),
                if light { "\u{263E}" } else { "\u{2600}" }
            }
        }
        if settings_open() {
            SettingsWindow { theme, on_close: move |_| settings_open.set(false),
                div { class: "kit-set-row lang-row",
                    span { class: "kit-set-label", {w.settings_language} }
                    div { class: "kit-set-seg",
                        for l in Lang::ALL {
                            button {
                                key: "{l.code()}",
                                class: if lang() == l { "kit-on" } else { "" },
                                onclick: move |_| {
                                    lang.set(l);
                                    chosen.write().lang = Some(l);
                                },
                                {l.name()}
                            }
                        }
                    }
                }
                div { class: "kit-set-actions",
                    button {
                        class: "kit-set-btn",
                        title: w.settings_progress_hint,
                        onclick: move |_| {
                            let _ = desktop_kit::launch::reveal(&progress::file());
                        },
                        {w.settings_progress}
                    }
                }
            }
        }
        if let Some(error) = content::load_error(lang()) {
            div { class: "body",
                div { class: "page",
                    h1 { class: "page-title", {w.load_failed} }
                    pre { class: "error", "{error}" }
                }
            }
        } else {
            Workspace {}
        }
    }
}
