//! The course, read once from the files embedded at build time.
//!
//! `cursus.yaml` gives the plan: étapes, then rubriques, then sous-rubriques.
//! Each étape's folder holds `chapitre.md` (the anecdote that opens it) and
//! one Markdown file per rubrique, whose YAML header carries its questions.

use std::sync::LazyLock;

use serde::Deserialize;

use crate::lang::Lang;

mod embedded {
    include!(concat!(env!("OUT_DIR"), "/content_files.rs"));
}

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
    /// Shown in French because the language asked for has no translation yet.
    pub fallback: bool,
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
    /// Shown in French because the language asked for has no translation yet.
    pub fallback: bool,
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

/// A course, and why it could not be read if it couldn't: then the course
/// is empty and the app says what went wrong.
type Loaded = (Course, Option<String>);

fn loaded(lang: Lang) -> Loaded {
    match load(embedded::FILES, lang) {
        Ok(course) => (course, None),
        Err(e) => (
            Course {
                titre: String::new(),
                etapes: Vec::new(),
            },
            Some(e),
        ),
    }
}

static FR: LazyLock<Loaded> = LazyLock::new(|| loaded(Lang::Fr));
static EN: LazyLock<Loaded> = LazyLock::new(|| loaded(Lang::En));

fn get(lang: Lang) -> &'static Loaded {
    match lang {
        Lang::Fr => &FR,
        Lang::En => &EN,
    }
}

/// The course in a language. Every language has the French course's étapes
/// and rubriques at the same places, so an index means the same in each.
pub fn course_in(lang: Lang) -> &'static Course {
    &get(lang).0
}

/// The French course: the structure every language shares.
pub fn course() -> &'static Course {
    course_in(Lang::Fr)
}

pub fn load_error(lang: Lang) -> Option<&'static str> {
    get(lang).1.as_deref()
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

fn find<'a>(files: &[(&str, &'a str)], path: &str) -> Option<&'a str> {
    files
        .iter()
        .find(|(p, _)| *p == path)
        .map(|(_, text)| *text)
}

fn read_plan(files: &[(&str, &str)], lang: &str) -> Result<Option<PlanFile>, String> {
    let Some(text) = find(files, &format!("{lang}/cursus.yaml")) else {
        return Ok(None);
    };
    serde_yaml::from_str(text)
        .map(Some)
        .map_err(|e| format!("{lang}/cursus.yaml : {e}"))
}

/// An étape folder's chapitre and rubriques, as written in one language.
fn read_folder(
    files: &[(&str, &str)],
    lang: &str,
    dossier: &str,
) -> Result<(Option<Chapitre>, Vec<Rubrique>), String> {
    let folder = format!("{lang}/{dossier}/");
    let mut chapitre = None;
    let mut found = Vec::new();
    for (path, text) in files.iter().filter(|(p, _)| p.starts_with(&folder)) {
        let (head, body) =
            split_front_matter(text).ok_or_else(|| format!("{path} : en-tête YAML manquant"))?;
        if path.ends_with("/chapitre.md") {
            let h: ChapitreHead =
                serde_yaml::from_str(head).map_err(|e| format!("{path} : {e}"))?;
            chapitre = Some(Chapitre {
                accroche: h.accroche,
                sources: h.sources,
                body: body.to_owned(),
                fallback: false,
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
                fallback: false,
            });
        }
    }
    Ok((chapitre, found))
}

/// The course in `lang`: the French plan, with each part taken from `lang`
/// where it is translated and from French, marked as such, where it isn't.
fn load(files: &[(&str, &str)], lang: Lang) -> Result<Course, String> {
    let plan = read_plan(files, "fr")?.ok_or("cursus.yaml manque")?;
    let code = lang.code();
    let local = if lang == Lang::Fr {
        None
    } else {
        read_plan(files, code)?
    };
    let local_etape = |id: &str| {
        local
            .as_ref()
            .and_then(|p| p.etapes.iter().find(|e| e.id == id))
    };

    let mut etapes = Vec::new();
    for pe in &plan.etapes {
        let (chapitre, found) = read_folder(files, "fr", &pe.dossier)?;
        let (chapitre, mut found) = if lang == Lang::Fr {
            (chapitre, found)
        } else {
            let (local_chapitre, local_found) = read_folder(files, code, &pe.dossier)?;
            let chapitre = local_chapitre.or_else(|| {
                chapitre.map(|c| Chapitre {
                    fallback: true,
                    ..c
                })
            });
            let found = found
                .into_iter()
                .map(|r| {
                    local_found
                        .iter()
                        .find(|l| l.id == r.id)
                        .cloned()
                        .unwrap_or(Rubrique {
                            fallback: true,
                            ..r
                        })
                })
                .collect();
            (chapitre, found)
        };
        // In the plan's order; a rubrique not written yet is left out.
        let rubriques = pe
            .rubriques
            .iter()
            .filter_map(|pr| {
                let i = found.iter().position(|r| r.id == pr.id)?;
                Some(found.swap_remove(i))
            })
            .collect();
        etapes.push(Etape {
            id: pe.id.clone(),
            titre: local_etape(&pe.id).map_or_else(|| pe.titre.clone(), |l| l.titre.clone()),
            chapitre,
            rubriques,
        });
    }
    Ok(Course {
        titre: local.map_or(plan.titre, |l| l.titre),
        etapes,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_embedded_course_reads() {
        assert_eq!(load_error(Lang::Fr), None);
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

    /// English mirrors French: the same étapes and rubriques, and in each
    /// translated rubrique the same sous-rubriques and questions, with the
    /// right answer at the same place. Progress is kept by id, so it carries
    /// from one language to the other only if they match.
    #[test]
    fn english_mirrors_french() {
        assert_eq!(load_error(Lang::En), None);
        let fr = course_in(Lang::Fr);
        let en = course_in(Lang::En);
        assert_eq!(fr.etapes.len(), en.etapes.len());
        for (ef, ee) in fr.etapes.iter().zip(&en.etapes) {
            assert_eq!(ef.id, ee.id);
            assert_eq!(
                ef.chapitre.is_some(),
                ee.chapitre.is_some(),
                "étape {}",
                ef.id
            );
            assert_eq!(ef.rubriques.len(), ee.rubriques.len(), "étape {}", ef.id);
            for (rf, re) in ef.rubriques.iter().zip(&ee.rubriques) {
                assert_eq!(rf.id, re.id);
                if re.fallback {
                    continue;
                }
                let ids = |r: &Rubrique| {
                    r.sous_rubriques
                        .iter()
                        .map(|s| s.id.clone())
                        .collect::<Vec<_>>()
                };
                assert_eq!(ids(rf), ids(re), "{} : sous-rubriques", rf.id);
                assert_eq!(rf.qcm.len(), re.qcm.len(), "{} : questions", rf.id);
                for (qf, qe) in rf.qcm.iter().zip(&re.qcm) {
                    let at = rf.key(qf);
                    assert_eq!(qf.id, qe.id, "{at}");
                    assert_eq!(qf.sous_rubrique, qe.sous_rubrique, "{at}");
                    assert_eq!(qf.reponse, qe.reponse, "{at}");
                    assert_eq!(qe.choix.len(), 4, "{at}");
                }
                for s in &rf.sous_rubriques {
                    let anchor = format!("{{#{}}}", s.id);
                    assert!(re.body.contains(&anchor), "{} : ancre {anchor}", rf.id);
                }
            }
        }
    }

    #[test]
    fn a_missing_translation_falls_back_to_french() {
        let files: &[(&str, &str)] = &[
            ("fr/cursus.yaml", "titre: Cours\netapes:\n  - id: \"00\"\n    dossier: 00-a\n    titre: Début\n    rubriques:\n      - id: r1\n      - id: r2\n"),
            ("fr/00-a/chapitre.md", "---\naccroche: Une\n---\nTexte\n"),
            ("fr/00-a/01.md", "---\nid: r1\ntitre: Un\nsous_rubriques: []\n---\nCorps\n"),
            ("fr/00-a/02.md", "---\nid: r2\ntitre: Deux\nsous_rubriques: []\n---\nCorps\n"),
            ("en/cursus.yaml", "titre: Course\netapes:\n  - id: \"00\"\n    dossier: 00-a\n    titre: Start\n    rubriques: []\n"),
            ("en/00-a/02.md", "---\nid: r2\ntitre: Two\nsous_rubriques: []\n---\nBody\n"),
        ];
        let c = load(files, Lang::En).unwrap();
        assert_eq!(c.titre, "Course");
        let e = &c.etapes[0];
        assert_eq!(e.titre, "Start");
        assert!(e.chapitre.as_ref().unwrap().fallback);
        assert_eq!(
            (e.rubriques[0].titre.as_str(), e.rubriques[0].fallback),
            ("Un", true)
        );
        assert_eq!(
            (e.rubriques[1].titre.as_str(), e.rubriques[1].fallback),
            ("Two", false)
        );
    }

    #[test]
    fn front_matter_splits() {
        let (head, body) = split_front_matter("---\nid: x\n---\n\n## Titre\n").unwrap();
        assert_eq!(head.trim(), "id: x");
        assert_eq!(body, "## Titre\n");
    }
}
