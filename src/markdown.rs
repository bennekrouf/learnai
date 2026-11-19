//! A lesson's Markdown to HTML, with the course's two additions:
//!
//! * a `:::pourquoi … :::` block becomes a "Pourquoi ?" button that unfolds
//!   the reason;
//! * a ```` ```mermaid ```` block becomes a diagram, drawn by Mermaid in the
//!   page (see `assets/mermaid.js`).

use pulldown_cmark::{html, CodeBlockKind, Event, Options, Parser, Tag, TagEnd};

/// The HTML for a lesson.
pub fn to_html(markdown: &str) -> String {
    let source = expand_pourquoi(markdown);
    let options = Options::ENABLE_TABLES | Options::ENABLE_HEADING_ATTRIBUTES;
    let mut in_mermaid = false;
    let mut diagram = String::new();
    let events = Parser::new_ext(&source, options).filter_map(|event| match event {
        Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(lang))) if lang.as_ref() == "mermaid" => {
            in_mermaid = true;
            diagram.clear();
            None
        }
        Event::Text(text) if in_mermaid => {
            diagram.push_str(&text);
            None
        }
        Event::End(TagEnd::CodeBlock) if in_mermaid => {
            in_mermaid = false;
            Some(Event::Html(
                format!(
                    "<div class=\"diagram\"><pre class=\"mermaid\">{}</pre></div>\n",
                    escape(&diagram)
                )
                .into(),
            ))
        }
        other => Some(other),
    });
    let mut out = String::with_capacity(source.len() * 2);
    html::push_html(&mut out, events);
    // A link in a lesson opens in the browser, not in place of the lesson.
    out.replace("<a href=", "<a target=\"_blank\" href=")
}

/// `:::pourquoi` … `:::` to a `<details>` the reader opens with a click. The
/// blank lines around the inside keep it Markdown.
fn expand_pourquoi(markdown: &str) -> String {
    let mut out = String::with_capacity(markdown.len() + 1024);
    let mut open = false;
    for line in markdown.lines() {
        let trimmed = line.trim();
        if !open && trimmed == ":::pourquoi" {
            open = true;
            out.push_str("<details class=\"why\"><summary>Pourquoi ?</summary>\n<div class=\"why-text\">\n\n");
        } else if open && trimmed == ":::" {
            open = false;
            out.push_str("\n</div>\n</details>\n");
        } else {
            out.push_str(line);
            out.push('\n');
        }
    }
    if open {
        out.push_str("\n</div>\n</details>\n");
    }
    out
}

fn escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pourquoi_becomes_a_button() {
        let html = to_html("Une phrase.\n\n:::pourquoi\nParce que **oui**.\n:::\n");
        assert!(html.contains("<details class=\"why\"><summary>Pourquoi ?</summary>"));
        assert!(html.contains("<strong>oui</strong>"));
        assert!(html.contains("</details>"));
    }

    #[test]
    fn headings_keep_their_anchor() {
        let html = to_html("## La réponse {#reponse}\n");
        assert!(
            html.contains("<h2 id=\"reponse\">La réponse</h2>"),
            "{html}"
        );
    }

    #[test]
    fn mermaid_becomes_a_diagram() {
        let html = to_html("```mermaid\nflowchart LR\n  A -->|x| B\n```\n");
        assert!(
            html.contains("<pre class=\"mermaid\">flowchart LR\n  A --&gt;|x| B\n</pre>"),
            "{html}"
        );
        assert!(!html.contains("<code"));
    }

    #[test]
    fn tables_render() {
        let html = to_html("| a | b |\n|---|---|\n| 1 | 2 |\n");
        assert!(html.contains("<table>"));
    }

    #[test]
    fn links_open_outside() {
        let html = to_html("[x](https://example.org)");
        assert!(html.contains("target=\"_blank\""));
    }
}
