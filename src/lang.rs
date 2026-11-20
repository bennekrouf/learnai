//! The two languages of the app, and every word it shows of its own: the
//! course text itself comes from `content/<lang>/`.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Lang {
    #[default]
    Fr,
    En,
}

impl Lang {
    pub const ALL: [Lang; 2] = [Lang::Fr, Lang::En];

    /// The folder under `content/`, and the code kept in `progress.json`.
    pub fn code(self) -> &'static str {
        match self {
            Lang::Fr => "fr",
            Lang::En => "en",
        }
    }

    /// The name of the language, in that language.
    pub fn name(self) -> &'static str {
        match self {
            Lang::Fr => "Français",
            Lang::En => "English",
        }
    }

    /// From the system's language tag, e.g. `fr-CH` or `en-US`. Any language
    /// but French gets English.
    pub fn from_tag(tag: &str) -> Lang {
        if tag.to_ascii_lowercase().starts_with("fr") {
            Lang::Fr
        } else {
            Lang::En
        }
    }

    pub fn words(self) -> &'static Words {
        match self {
            Lang::Fr => &FR,
            Lang::En => &EN,
        }
    }
}

/// The app's own words. A field per fixed text; the methods below build
/// the ones with a number or a name in them.
pub struct Words {
    pub lang: Lang,
    pub tab_quiz: &'static str,
    pub tab_quiz_hint: &'static str,
    pub tab_lesson: &'static str,
    pub tab_lesson_hint: &'static str,
    pub zoom_hint: &'static str,
    pub zoom_out: &'static str,
    pub zoom_in: &'static str,
    pub zoom_reset: &'static str,
    pub resize_hint: &'static str,
    pub settings_language: &'static str,
    pub theme_light: &'static str,
    pub theme_dark: &'static str,
    pub settings_progress: &'static str,
    pub settings_progress_hint: &'static str,
    pub load_failed: &'static str,
    pub not_translated: &'static str,

    pub why: &'static str,
    pub why_hide: &'static str,

    pub sidebar_answered: &'static str,
    pub sidebar_score: &'static str,
    pub sidebar_questions: &'static str,
    pub sidebar_foot_quiz: &'static str,
    pub sidebar_foot_lesson: &'static str,

    pub overview_sub: &'static str,
    pub read_story: &'static str,
    pub rubriques: &'static str,
    pub review: &'static str,
    pub resume: &'static str,
    pub start: &'static str,

    pub score_hint: &'static str,
    pub result: &'static str,
    pub all_right: &'static str,
    pub to_review: &'static str,
    pub not_answered: &'static str,
    pub read_lesson: &'static str,
    pub restart: &'static str,
    pub restart_hint: &'static str,
    pub right: &'static str,
    pub wrong: &'static str,
    pub see_result: &'static str,
    pub next_question: &'static str,
    pub pick_one: &'static str,

    pub take_quiz: &'static str,
    pub take_rubrique_quiz: &'static str,
    pub next: &'static str,
    pub sources: &'static str,
}

impl Words {
    pub fn etape(&self, n: u32) -> String {
        match self.lang {
            Lang::Fr => format!("Étape {n}"),
            _ => format!("Step {n}"),
        }
    }

    pub fn questions_count(&self, answered: usize, total: usize, correct: usize) -> String {
        match self.lang {
            Lang::Fr => format!("{answered} / {total} questions · {correct} justes"),
            _ => format!("{answered} / {total} questions · {correct} right"),
        }
    }

    pub fn answered_of(&self, answered: usize, total: usize, correct: usize) -> String {
        match self.lang {
            Lang::Fr => format!("{answered} / {total} répondues · {correct} justes"),
            _ => format!("{answered} / {total} answered · {correct} right"),
        }
    }

    pub fn meter(&self, answered: usize, total: usize) -> String {
        match self.lang {
            Lang::Fr => format!("{answered} questions répondues sur {total}"),
            Lang::En => format!("{answered} of {total} questions answered"),
        }
    }

    pub fn n_questions(&self, n: usize) -> String {
        format!("{n} questions")
    }

    pub fn question_of(&self, i: usize, total: usize) -> String {
        format!("Question {i} / {total}")
    }

    pub fn left(&self, n: usize) -> String {
        match self.lang {
            Lang::Fr => format!("Il reste {n} question(s) sans réponse."),
            _ if n == 1 => "1 question left unanswered.".to_owned(),
            _ => format!("{n} questions left unanswered."),
        }
    }

    pub fn right_answer(&self, text: &str) -> String {
        match self.lang {
            Lang::Fr => format!("Bonne réponse : {text}"),
            _ => format!("Right answer: {text}"),
        }
    }

    pub fn see_in_lesson(&self, part: &str) -> String {
        match self.lang {
            Lang::Fr => format!("Voir « {part} » dans le cours \u{2192}"),
            _ => format!("See \u{201c}{part}\u{201d} in the lesson \u{2192}"),
        }
    }

    pub fn next_rubrique(&self, title: &str) -> String {
        match self.lang {
            Lang::Fr => format!("Rubrique suivante : {title}"),
            _ => format!("Next topic: {title}"),
        }
    }

    pub fn review_in_lesson(&self, part: &str) -> String {
        match self.lang {
            Lang::Fr => format!("Revoir « {part} » dans le cours \u{2192}"),
            Lang::En => format!("Review \u{201c}{part}\u{201d} in the lesson \u{2192}"),
        }
    }

    /// "Étape 3 : titre", "Step 3: title".
    pub fn etape_titled(&self, n: u32, title: &str) -> String {
        match self.lang {
            Lang::Fr => format!("Étape {n} : {title}"),
            Lang::En => format!("Step {n}: {title}"),
        }
    }

    pub fn start_quiz_of(&self, title: &str) -> String {
        match self.lang {
            Lang::Fr => format!("Commencer le QCM : {title} \u{2192}"),
            _ => format!("Start the quiz: {title} \u{2192}"),
        }
    }
}

pub static FR: Words = Words {
    lang: Lang::Fr,
    tab_quiz: "QCM",
    tab_quiz_hint: "Les questions d'abord, chacune avec son pourquoi",
    tab_lesson: "Cours",
    tab_lesson_hint: "Le cours, phrase courte par phrase courte",
    zoom_hint: "Taille du texte et des schémas (⌘+ / ⌘− / ⌘0)",
    zoom_out: "Réduire",
    zoom_in: "Agrandir",
    zoom_reset: "Revenir à 100 %",
    resize_hint: "Glisser pour élargir ou réduire · double-clic pour revenir à la taille d'origine",
    settings_language: "Langue",
    theme_light: "Passer en mode jour",
    theme_dark: "Passer en mode nuit",
    settings_progress: "Afficher le fichier de progression",
    settings_progress_hint: "Le fichier progress.json : vos réponses",
    load_failed: "Le cours n'a pas pu être lu",
    not_translated: "",

    why: "Pourquoi ?",
    why_hide: "Masquer le pourquoi",

    sidebar_answered: "Questions répondues",
    sidebar_score: "Bonnes réponses sur le total",
    sidebar_questions: "Questions",
    sidebar_foot_quiz: "Le QCM d'abord, le cours ensuite.",
    sidebar_foot_lesson: "Chaque « Pourquoi ? » s'ouvre d'un clic.",

    overview_sub: "Les QCM de l'étape, une rubrique à la fois",
    read_story: "Lire l'anecdote",
    rubriques: "Rubriques",
    review: "Revoir",
    resume: "Continuer",
    start: "Commencer",

    score_hint: "Bonnes réponses sur les questions répondues",
    result: "Résultat",
    all_right: "Tout juste. Le cours reste là pour les détails.",
    to_review: "Les questions à revoir, avec la partie du cours qui y répond :",
    not_answered: "Pas encore répondu",
    read_lesson: "Lire le cours",
    restart: "Recommencer",
    restart_hint: "Efface les réponses de cette rubrique",
    right: "Bonne réponse.",
    wrong: "Pas tout à fait.",
    see_result: "Voir le résultat \u{2192}",
    next_question: "Question suivante \u{2192}",
    pick_one: "Choisis une réponse.",

    take_quiz: "Faire le QCM",
    take_rubrique_quiz: "Faire le QCM de cette rubrique",
    next: "Suite \u{2192}",
    sources: "Sources",
};

pub static EN: Words = Words {
    lang: Lang::En,
    tab_quiz: "Quiz",
    tab_quiz_hint: "The questions first, each with its why",
    tab_lesson: "Lesson",
    tab_lesson_hint: "The lesson, one short sentence at a time",
    zoom_hint: "Size of the text and diagrams (⌘+ / ⌘− / ⌘0)",
    zoom_out: "Smaller",
    zoom_in: "Larger",
    zoom_reset: "Back to 100 %",
    resize_hint: "Drag to widen or narrow · double-click for the original size",
    settings_language: "Language",
    theme_light: "Switch to day mode",
    theme_dark: "Switch to night mode",
    settings_progress: "Show the progress file",
    settings_progress_hint: "The progress.json file: your answers",
    load_failed: "The course could not be read",
    not_translated: "Not translated into English yet: shown in French.",

    why: "Why?",
    why_hide: "Hide the why",

    sidebar_answered: "Questions answered",
    sidebar_score: "Right answers out of all",
    sidebar_questions: "Questions",
    sidebar_foot_quiz: "The quiz first, the lesson next.",
    sidebar_foot_lesson: "Each \u{201c}Why?\u{201d} opens with a click.",

    overview_sub: "The step's quizzes, one topic at a time",
    read_story: "Read the story",
    rubriques: "Topics",
    review: "Review",
    resume: "Continue",
    start: "Start",

    score_hint: "Right answers out of those answered",
    result: "Result",
    all_right: "All right. The lesson is there for the details.",
    to_review: "The questions to review, with the part of the lesson that answers them:",
    not_answered: "Not answered yet",
    read_lesson: "Read the lesson",
    restart: "Start again",
    restart_hint: "Clears this topic's answers",
    right: "Right.",
    wrong: "Not quite.",
    see_result: "See the result \u{2192}",
    next_question: "Next question \u{2192}",
    pick_one: "Pick an answer.",

    take_quiz: "Take the quiz",
    take_rubrique_quiz: "Take this topic's quiz",
    next: "Next \u{2192}",
    sources: "Sources",
};

#[cfg(test)]
mod tests {
    use super::Lang;

    #[test]
    fn system_tags() {
        assert_eq!(Lang::from_tag("fr-CH"), Lang::Fr);
        assert_eq!(Lang::from_tag("FR"), Lang::Fr);
        assert_eq!(Lang::from_tag("en-US"), Lang::En);
        assert_eq!(Lang::from_tag("de-DE"), Lang::En);
    }

    #[test]
    fn words_differ_by_language() {
        assert_eq!(Lang::Fr.words().etape(3), "Étape 3");
        assert_eq!(Lang::En.words().etape(3), "Step 3");
        assert_eq!(Lang::En.words().left(1), "1 question left unanswered.");
    }
}
