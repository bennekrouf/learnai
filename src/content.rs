//! The course, read once from the files embedded at build time.
//!
//! `cursus.yaml` gives the plan: étapes, then rubriques, then sous-rubriques.
//! Each étape's folder holds `chapitre.md` (the anecdote that opens it) and
//! one Markdown file per rubrique, whose YAML header carries its questions.

use std::sync::LazyLock;

use serde::Deserialize;

mod embedded {
    include!(concat!(env!("OUT_DIR"), "/content_files.rs"));
}

/// The language the app shows. One for now.
const LANG: &str = "fr";

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Source {
    pub titre: String,
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct SousRubrique {
    pub id: String,
    pub titre: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Question {
    pub id: String,
    pub sous_rubrique: String,
    pub question: String,
    pub choix: Vec<String>,
    pub reponse: usize,
    pub pourquoi: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Rubrique {
    pub id: String,
    pub titre: String,
    pub sous_rubriques: Vec<SousRubrique>,
    pub sources: Vec<Source>,
    pub qcm: Vec<Question>,
    /// The lesson, in Markdown.
    pub body: String,
}

impl Rubrique {
    /// The key a question's answer is kept under: question ids repeat from
    /// one étape to the next, rubrique ids don't.
    pub fn key(&self, q: &Question) -> String {
        format!("{}/{}", self.id, q.id)
    }

    pub fn sous_rubrique(&self, id: &str) -> Option<&SousRubrique> {
        self.sous_rubriques.iter().find(|s| s.id == id)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Chapitre {
    pub accroche: String,
    pub sources: Vec<Source>,
    pub body: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Etape {
    pub id: String,
    pub titre: String,
    pub chapitre: Option<Chapitre>,
    pub rubriques: Vec<Rubrique>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Course {
    pub titre: String,
    pub etapes: Vec<Etape>,
}

impl Course {
    /// Every question of the course, with its rubrique.
    #[cfg(test)]
    pub fn questions(&self) -> impl Iterator<Item = (&Rubrique, &Question)> {
        self.etapes
            .iter()
            .flat_map(|e| e.rubriques.iter())
            .flat_map(|r| r.qcm.iter().map(move |q| (r, q)))
    }
}

/// The course, and why it could not be read if it couldn't: then the course
/// is empty and the app says what went wrong.
static COURSE: LazyLock<(Course, Option<String>)> = LazyLock::new(|| match load(embedded::FILES) {
    Ok(course) => (course, None),
    Err(e) => (
        Course {
            titre: String::new(),
            etapes: Vec::new(),
        },
        Some(e),
    ),
});

pub fn course() -> &'static Course {
    &COURSE.0
}

pub fn load_error() -> Option<&'static str> {
    COURSE.1.as_deref()
}

// ── Reading ─────────────────────────────────────────────────────────────

#[derive(Deserialize)]
struct PlanFile {
    titre: String,
    etapes: Vec<PlanEtape>,
}

#[derive(Deserialize)]
struct PlanEtape {
    id: String,
    dossier: String,
    titre: String,
    rubriques: Vec<PlanRubrique>,
}

#[derive(Deserialize)]
struct PlanRubrique {
    id: String,
}

#[derive(Deserialize)]
struct RubriqueHead {
    id: String,
    titre: String,
    sous_rubriques: Vec<SousRubrique>,
    #[serde(default)]
    sources: Vec<Source>,
    #[serde(default)]
    qcm: Vec<Question>,
}

#[derive(Deserialize)]
struct ChapitreHead {
    accroche: String,
    #[serde(default)]
    sources: Vec<Source>,
}

/// The YAML header and the Markdown under it.
fn split_front_matter(text: &str) -> Option<(&str, &str)> {
    let rest = text.strip_prefix("---")?;
    let end = rest.find("\n---")?;
    let head = &rest[..end];
    let body = rest[end + 4..].trim_start_matches(['\r', '\n']);
    Some((head, body))
}

fn load(files: &[(&str, &str)]) -> Result<Course, String> {
    let file = |path: &str| {
        files
            .iter()
            .find(|(p, _)| *p == path)
            .map(|(_, text)| *text)
    };
    let plan_text = file(&format!("{LANG}/cursus.yaml")).ok_or("cursus.yaml manque")?;
    let plan: PlanFile =
        serde_yaml::from_str(plan_text).map_err(|e| format!("cursus.yaml : {e}"))?;

    let mut etapes = Vec::new();
    for pe in plan.etapes {
        let folder = format!("{LANG}/{}/", pe.dossier);
        let mut chapitre = None;
        let mut found = Vec::new();
        for (path, text) in files.iter().filter(|(p, _)| p.starts_with(&folder)) {
            let (head, body) = split_front_matter(text)
                .ok_or_else(|| format!("{path} : en-tête YAML manquant"))?;
            if path.ends_with("/chapitre.md") {
                let h: ChapitreHead =
                    serde_yaml::from_str(head).map_err(|e| format!("{path} : {e}"))?;
                chapitre = Some(Chapitre {
                    accroche: h.accroche,
                    sources: h.sources,
                    body: body.to_owned(),
                });
            } else {
                let h: RubriqueHead =
                    serde_yaml::from_str(head).map_err(|e| format!("{path} : {e}"))?;
                found.push(Rubrique {
                    id: h.id,
                    titre: h.titre,
                    sous_rubriques: h.sous_rubriques,
                    sources: h.sources,
                    qcm: h
                        .qcm
                        .into_iter()
                        .filter(|q| q.reponse < q.choix.len())
                        .collect(),
                    body: body.to_owned(),
                });
            }
        }
        // In the plan's order; a rubrique not written yet is left out.
        let rubriques = pe
            .rubriques
            .iter()
            .filter_map(|pr| found.iter().find(|r| r.id == pr.id).cloned())
            .collect();
        etapes.push(Etape {
            id: pe.id,
            titre: pe.titre,
            chapitre,
            rubriques,
        });
    }
    Ok(Course {
        titre: plan.titre,
        etapes,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_embedded_course_reads() {
        assert_eq!(load_error(), None);
        let course = course();
        assert_eq!(course.etapes.len(), 10);
        for e in &course.etapes {
            assert!(e.chapitre.is_some(), "étape {} sans chapitre", e.id);
            assert!(!e.rubriques.is_empty(), "étape {} sans rubrique", e.id);
            for r in &e.rubriques {
                assert!(!r.qcm.is_empty(), "{} sans question", r.id);
                for q in &r.qcm {
                    assert_eq!(q.choix.len(), 4, "{}", r.key(q));
                    assert!(r.sous_rubrique(&q.sous_rubrique).is_some(), "{}", r.key(q));
                }
            }
        }
        assert!(course.questions().count() > 400);
    }

    #[test]
    fn front_matter_splits() {
        let (head, body) = split_front_matter("---\nid: x\n---\n\n## Titre\n").unwrap();
        assert_eq!(head.trim(), "id: x");
        assert_eq!(body, "## Titre\n");
    }
}
