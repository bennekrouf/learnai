//! What the learner answered, kept in `progress.json` in the app's data
//! folder, with the zoom they read at. Only the latest answer to each
//! question counts.

use std::collections::BTreeMap;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::content::Rubrique;

const FILE: &str = "progress.json";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Answer {
    /// The choice clicked, from 0.
    pub choice: usize,
    pub correct: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Progress {
    /// By `rubrique/question` key (see `Rubrique::key`).
    #[serde(default)]
    pub answers: BTreeMap<String, Answer>,
    /// The page's zoom in percent; none is 100 %.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zoom: Option<u32>,
    /// The left panel's width in pixels; none is the default.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sidebar: Option<u32>,
}

/// The left panel's width: default, and the range a drag stays within.
pub const SIDEBAR_DEFAULT: u32 = 300;
pub const SIDEBAR_MIN: u32 = 200;
pub const SIDEBAR_MAX: u32 = 560;

/// A width the left panel can take, from where the pointer is.
pub fn clamp_sidebar(px: f64) -> u32 {
    // In range after the clamp, so the cast cannot truncate.
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let w = px
        .round()
        .clamp(f64::from(SIDEBAR_MIN), f64::from(SIDEBAR_MAX)) as u32;
    w
}

/// The zoom steps, in percent, that − and + go through.
pub const ZOOMS: [u32; 9] = [80, 90, 100, 110, 125, 140, 160, 180, 200];

impl Progress {
    pub fn zoom(&self) -> u32 {
        self.zoom.unwrap_or(100)
    }

    /// One step up (`+1`) or down (`-1`), or back to 100 % (`0`).
    pub fn step_zoom(&mut self, dir: i32) {
        let now = self.zoom();
        let next = match dir.signum() {
            1 => ZOOMS.iter().copied().find(|z| *z > now).unwrap_or(now),
            -1 => ZOOMS
                .iter()
                .rev()
                .copied()
                .find(|z| *z < now)
                .unwrap_or(now),
            _ => 100,
        };
        self.zoom = (next != 100).then_some(next);
    }
}

/// How far a rubrique, or an étape, has got.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Score {
    pub answered: usize,
    pub correct: usize,
    pub total: usize,
}

impl Score {
    pub fn done(self) -> bool {
        self.total > 0 && self.answered == self.total
    }

    /// Answered, as a percentage of the questions.
    pub fn percent(self) -> usize {
        (self.answered * 100).checked_div(self.total).unwrap_or(0)
    }

    pub fn add(self, other: Score) -> Score {
        Score {
            answered: self.answered + other.answered,
            correct: self.correct + other.correct,
            total: self.total + other.total,
        }
    }
}

impl Progress {
    pub fn score(&self, r: &Rubrique) -> Score {
        let mut s = Score {
            total: r.qcm.len(),
            ..Score::default()
        };
        for q in &r.qcm {
            if let Some(a) = self.answers.get(&r.key(q)) {
                s.answered += 1;
                if a.correct {
                    s.correct += 1;
                }
            }
        }
        s
    }

    /// The first question of the rubrique not answered yet.
    pub fn next_open(&self, r: &Rubrique) -> Option<usize> {
        r.qcm
            .iter()
            .position(|q| !self.answers.contains_key(&r.key(q)))
    }

    /// Forgets the rubrique's answers, to take its QCM again.
    pub fn reset(&mut self, r: &Rubrique) {
        for q in &r.qcm {
            self.answers.remove(&r.key(q));
        }
    }
}

pub fn data_dir() -> PathBuf {
    dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("learnai")
}

pub fn file() -> PathBuf {
    data_dir().join(FILE)
}

/// The saved progress; empty on a first run or if the file can't be read.
/// Before the app was called LearnAI it was LearnKit: its answers carry over.
pub fn load() -> Progress {
    let old = dirs::data_local_dir().map(|d| d.join("learnkit").join(FILE));
    std::fs::read_to_string(file())
        .ok()
        .or_else(|| old.and_then(|p| std::fs::read_to_string(p).ok()))
        .and_then(|text| serde_json::from_str(&text).ok())
        .unwrap_or_default()
}

/// Writes through a temporary file, so a crash never leaves half a file.
pub fn save(progress: &Progress) {
    let dir = data_dir();
    if std::fs::create_dir_all(&dir).is_err() {
        return;
    }
    let Ok(text) = serde_json::to_string_pretty(progress) else {
        return;
    };
    let tmp = dir.join(format!("{FILE}.tmp"));
    if std::fs::write(&tmp, text).is_ok() {
        let _ = std::fs::rename(&tmp, file());
    }
}

#[cfg(test)]
mod tests {
    use super::Progress;

    #[test]
    fn zoom_steps_and_resets() {
        let mut p = Progress::default();
        p.step_zoom(1);
        assert_eq!(p.zoom(), 110);
        p.step_zoom(-1);
        p.step_zoom(-1);
        assert_eq!(p.zoom(), 90);
        p.step_zoom(0);
        assert_eq!((p.zoom(), p.zoom), (100, None));
        for _ in 0..20 {
            p.step_zoom(1);
        }
        assert_eq!(p.zoom(), 200);
    }
}

#[cfg(test)]
mod sidebar_tests {
    use super::{clamp_sidebar, SIDEBAR_MAX, SIDEBAR_MIN};

    #[test]
    fn the_left_panel_stays_in_range() {
        assert_eq!(clamp_sidebar(10.0), SIDEBAR_MIN);
        assert_eq!(clamp_sidebar(333.4), 333);
        assert_eq!(clamp_sidebar(5000.0), SIDEBAR_MAX);
    }
}
