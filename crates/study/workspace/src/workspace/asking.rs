//! Choosing and presenting questions.

use rand::Rng;
use rand::seq::IndexedRandom;
use study_answer::rules::expected::expected_answer;
use study_answer::rules::predict::predict;
use study_quiz::rules::order::{order_for, question_type_for};
use study_quiz::rules::prompt::{SenseFace, build_prompt};
use study_quiz::{QuizSettings, SentenceDisplay};

use super::Workspace;
use crate::rules::pick::pick_index;
use crate::rules::recent_window::recent_window;
use crate::session::{Current, Submission};
use crate::{
    Item, Question, QuestionView, Reveal, RevealView, SentenceToRender, WorkspaceError,
    WorkspaceSettings, Zone,
};

impl Workspace {
    /// The zone's current question, choosing one if there is none. `None` when the zone is empty.
    pub fn current_question(
        &mut self,
        zone: Zone,
        quiz: &QuizSettings,
        settings: &WorkspaceSettings,
        rng: &mut impl Rng,
    ) -> Option<Question> {
        let still_present = self
            .session
            .current(zone)
            .is_some_and(|current| self.position(zone, &current.key).is_some());
        if !still_present {
            let current = self.choose(zone, quiz, settings, rng)?;
            self.session.set_current(zone, current);
        }
        let current = self.session.current(zone)?;
        let item = &self.state.items[self.position(zone, &current.key)?];
        Some(question_of(item, current, quiz))
    }

    /// Records the input of the current question and reveals the answer.
    pub fn submit(&mut self, zone: Zone, input: String) -> Result<Reveal, WorkspaceError> {
        let current = self
            .session
            .current(zone)
            .ok_or(WorkspaceError::NoQuestion)?;
        let index = self
            .position(zone, &current.key)
            .ok_or(WorkspaceError::NoQuestion)?;
        let item = &self.state.items[index];
        let sentence = current.sentence.and_then(|i| item.sentences.get(i));
        let expected = expected_answer(current.question_type, &item.key.kana);
        let prediction = predict(&input, expected.as_deref());
        let reveal = Reveal {
            view: RevealView {
                writing: item.key.writing.clone(),
                kana: item.key.kana.clone(),
                glosses: item.glosses.clone(),
                translation: sentence.and_then(|s| s.translation.clone()),
                expected,
                input: input.clone(),
                prediction,
            },
            sentence: sentence.map(|s| SentenceToRender {
                marked: s.marked.clone(),
                display: SentenceDisplay::KanjiOnly,
            }),
        };
        if let Some(current) = self.session.current_mut(zone) {
            current.submission = Some(Submission { input, prediction });
        }
        Ok(reveal)
    }

    /// Picks a sense by remaining steps, skipping the last k asked, and a random sentence.
    fn choose(
        &self,
        zone: Zone,
        quiz: &QuizSettings,
        settings: &WorkspaceSettings,
        rng: &mut impl Rng,
    ) -> Option<Current> {
        let items: Vec<&Item> = self.state.items_in(zone).collect();
        let excluded = self
            .session
            .recent(zone, recent_window(settings.recent_limit, items.len()));
        let eligible: Vec<&Item> = items
            .into_iter()
            .filter(|item| !excluded.contains(&&item.key))
            .collect();
        let weights: Vec<u32> = eligible.iter().map(|item| item.remaining.max(1)).collect();
        let item = eligible[pick_index(&weights, rng)?];
        let order = order_for(quiz, &item.key.writing);
        let indices: Vec<usize> = (0..item.sentences.len()).collect();
        Some(Current {
            key: item.key.clone(),
            question_type: question_type_for(order, item.remaining),
            sentence: indices.choose(rng).copied(),
            submission: None,
        })
    }
}

fn question_of(item: &Item, current: &Current, quiz: &QuizSettings) -> Question {
    let steps = u32::try_from(order_for(quiz, &item.key.writing).len()).unwrap_or(1);
    let face = SenseFace {
        writing: &item.key.writing,
        kana: &item.key.kana,
        glosses: &item.glosses,
    };
    let prompt = build_prompt(current.question_type, &face, quiz.comprehensive);
    let sentence = current.sentence.and_then(|i| item.sentences.get(i));
    Question {
        view: QuestionView {
            zone: item.zone,
            question_type: current.question_type,
            glosses: prompt.glosses,
            card: prompt.card,
            answer_input: prompt.answer_input,
            remaining: item.remaining.min(steps),
            steps,
        },
        sentence: prompt
            .sentence_display
            .zip(sentence)
            .map(|(display, s)| SentenceToRender {
                marked: s.marked.clone(),
                display,
            }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::fixtures::rng;
    use crate::test_support::{snapshot, today};
    use crate::{Progress, WorkspaceState};
    use study_answer::Rating;
    use study_quiz::QuestionType;

    fn workspace_with(writings: &[&str], zone: Zone) -> Workspace {
        let mut workspace = Workspace::new(WorkspaceState::default());
        let snapshots = writings
            .iter()
            .map(|w| snapshot(w, Progress::default()))
            .collect();
        workspace.admit(zone, snapshots, &QuizSettings::default(), today());
        workspace
    }

    #[test]
    fn learning_starts_with_the_first_question_type() {
        let mut workspace = workspace_with(&["諦める"], Zone::Learning);
        let question = workspace
            .current_question(
                Zone::Learning,
                &QuizSettings::default(),
                &WorkspaceSettings::default(),
                &mut rng(),
            )
            .unwrap();
        assert_eq!(question.view.question_type, QuestionType::Sentence);
        assert_eq!((question.view.remaining, question.view.steps), (4, 4));
        assert_eq!(
            question.sentence.unwrap().display,
            SentenceDisplay::KanaOnly
        );
    }

    #[test]
    fn review_starts_with_the_last_question_type() {
        let mut workspace = workspace_with(&["諦める"], Zone::Review);
        let question = workspace
            .current_question(
                Zone::Review,
                &QuizSettings::default(),
                &WorkspaceSettings::default(),
                &mut rng(),
            )
            .unwrap();
        assert_eq!(question.view.question_type, QuestionType::Comprehensive);
        assert!(question.sentence.is_none());
        assert!(question.view.card.is_some());
    }

    #[test]
    fn keeps_the_same_question_until_answered() {
        let mut workspace = workspace_with(&["a", "b", "c"], Zone::Learning);
        let (quiz, settings) = (QuizSettings::default(), WorkspaceSettings::default());
        let mut rng = rng();
        let first = workspace.current_question(Zone::Learning, &quiz, &settings, &mut rng);
        for _ in 0..5 {
            assert_eq!(
                workspace.current_question(Zone::Learning, &quiz, &settings, &mut rng),
                first
            );
        }
    }

    #[test]
    fn empty_zone_has_no_question() {
        let mut workspace = Workspace::new(WorkspaceState::default());
        assert!(
            workspace
                .current_question(
                    Zone::Review,
                    &QuizSettings::default(),
                    &WorkspaceSettings::default(),
                    &mut rng()
                )
                .is_none()
        );
    }

    #[test]
    fn does_not_repeat_recent_senses() {
        let mut workspace = workspace_with(&["a", "b", "c"], Zone::Learning);
        let (quiz, settings) = (QuizSettings::default(), WorkspaceSettings::default());
        let mut rng = rng();
        let mut asked = Vec::new();
        for _ in 0..30 {
            let question = workspace
                .current_question(Zone::Learning, &quiz, &settings, &mut rng)
                .unwrap();
            let key = workspace
                .session
                .current(Zone::Learning)
                .unwrap()
                .key
                .clone();
            assert!(
                !asked.iter().rev().take(2).any(|k| k == &key),
                "{question:?}"
            );
            asked.push(key);
            workspace.submit(Zone::Learning, String::new()).unwrap();
            workspace
                .rate(
                    Zone::Learning,
                    Rating::Vague,
                    &quiz,
                    &settings,
                    crate::rules::fixtures::now(),
                )
                .unwrap();
        }
    }

    #[test]
    fn submit_predicts_kana_answers() {
        let mut workspace = workspace_with(&["諦める"], Zone::Review);
        let quiz = QuizSettings {
            kanji_order: vec![QuestionType::Translation],
            ..QuizSettings::default()
        };
        workspace
            .current_question(
                Zone::Review,
                &quiz,
                &WorkspaceSettings::default(),
                &mut rng(),
            )
            .unwrap();
        let reveal = workspace.submit(Zone::Review, "カナ".into()).unwrap();
        assert_eq!(reveal.view.expected.as_deref(), Some("かな"));
        assert_eq!(reveal.view.prediction, Some(Rating::Known));
        assert_eq!(reveal.sentence.unwrap().display, SentenceDisplay::KanjiOnly);
    }

    #[test]
    fn submit_without_question_fails() {
        let mut workspace = Workspace::new(WorkspaceState::default());
        assert_eq!(
            workspace.submit(Zone::Review, String::new()).unwrap_err(),
            WorkspaceError::NoQuestion
        );
    }
}
