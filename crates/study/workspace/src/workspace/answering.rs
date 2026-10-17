//! Rating, skipping and reporting the current question.

use chrono::{DateTime, FixedOffset};
use study_answer::Rating;
use study_answer::rules::log_condition::should_log;
use study_quiz::rules::order::order_for;
use study_quiz::{QuestionType, QuizSettings};

use super::Workspace;
use crate::rules::exit_progress::{full_progress, skipped_progress};
use crate::rules::rating::apply_rating;
use crate::{
    AnswerRecord, RateOutcome, ReportedSentence, Settlement, WorkspaceError, WorkspaceSettings,
    Zone,
};

impl Workspace {
    /// Applies the self-rating. A sense that becomes full is left untouched
    /// here: the caller settles it, then removes it.
    pub fn rate(
        &mut self,
        zone: Zone,
        rating: Rating,
        quiz: &QuizSettings,
        settings: &WorkspaceSettings,
        now: DateTime<FixedOffset>,
    ) -> Result<RateOutcome, WorkspaceError> {
        let current = self
            .session
            .take_current(zone)
            .ok_or(WorkspaceError::NoQuestion)?;
        let index = self
            .position(zone, &current.key)
            .ok_or(WorkspaceError::NoQuestion)?;
        let item = &mut self.state.items[index];
        let prediction = current.submission.as_ref().and_then(|s| s.prediction);
        let shows_sentence = current.question_type != QuestionType::Comprehensive;

        let record = should_log(prediction, rating).then(|| AnswerRecord {
            time: now,
            key: item.key.clone(),
            question_type: current.question_type,
            sentence: current
                .sentence
                .and_then(|i| item.sentences.get(i))
                .filter(|_| shows_sentence)
                .map(|s| s.marked.clone()),
            answer: shows_sentence.then(|| {
                current
                    .submission
                    .as_ref()
                    .map(|s| s.input.clone())
                    .unwrap_or_default()
            }),
            prediction,
            rating,
        });

        let steps = u32::try_from(order_for(quiz, &item.key.writing).len()).unwrap_or(1);
        let rated = apply_rating(item.remaining, item.progress, rating, zone, steps);
        let settlement = if rated.remaining == 0 {
            Some(Settlement {
                key: item.key.clone(),
                progress: full_progress(rated.progress, settings.max_stage()),
            })
        } else {
            item.remaining = rated.remaining;
            item.progress = rated.progress;
            None
        };
        self.session
            .push_recent(zone, current.key, settings.recent_limit);
        Ok(RateOutcome { record, settlement })
    }

    /// Skips the current sense: stage N. The caller settles it, then removes it.
    pub fn skip(
        &mut self,
        zone: Zone,
        settings: &WorkspaceSettings,
    ) -> Result<Settlement, WorkspaceError> {
        let current = self
            .session
            .take_current(zone)
            .ok_or(WorkspaceError::NoQuestion)?;
        let index = self
            .position(zone, &current.key)
            .ok_or(WorkspaceError::NoQuestion)?;
        let progress = skipped_progress(self.state.items[index].progress, settings.max_stage());
        self.session
            .push_recent(zone, current.key.clone(), settings.recent_limit);
        Ok(Settlement {
            key: current.key,
            progress,
        })
    }

    /// Reports the current sentence as wrong. The caller deletes it, then
    /// removes the sense without settling.
    pub fn report_sentence(&mut self, zone: Zone) -> Result<ReportedSentence, WorkspaceError> {
        let current = self
            .session
            .current(zone)
            .ok_or(WorkspaceError::NoQuestion)?;
        let index = self
            .position(zone, &current.key)
            .ok_or(WorkspaceError::NoQuestion)?;
        let sentence = current
            .sentence
            .and_then(|i| self.state.items[index].sentences.get(i))
            .ok_or(WorkspaceError::NoSentence)?;
        let reported = ReportedSentence {
            key: current.key.clone(),
            marked: sentence.marked.clone(),
        };
        self.session.take_current(zone);
        Ok(reported)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::fixtures::{key, now, rng};
    use crate::test_support::{snapshot, today};
    use crate::{Progress, WorkspaceState};

    fn single(zone: Zone, quiz: &QuizSettings) -> Workspace {
        let mut workspace = Workspace::new(WorkspaceState::default());
        workspace.admit(
            zone,
            vec![snapshot(
                "諦める",
                Progress {
                    stage: 2,
                    passes: 1,
                    lapses: 0,
                },
            )],
            quiz,
            today(),
        );
        workspace
    }

    fn ask(workspace: &mut Workspace, zone: Zone, quiz: &QuizSettings) {
        workspace
            .current_question(zone, quiz, &WorkspaceSettings::default(), &mut rng())
            .unwrap();
    }

    #[test]
    fn known_in_review_becomes_full_without_touching_the_item() {
        let quiz = QuizSettings::default();
        let mut workspace = single(Zone::Review, &quiz);
        ask(&mut workspace, Zone::Review, &quiz);
        let outcome = workspace
            .rate(
                Zone::Review,
                Rating::Known,
                &quiz,
                &WorkspaceSettings::default(),
                now(),
            )
            .unwrap();
        assert_eq!(
            outcome,
            RateOutcome {
                record: None,
                settlement: Some(Settlement {
                    key: key("諦める"),
                    progress: Progress {
                        stage: 3,
                        passes: 2,
                        lapses: 0
                    }
                }),
            }
        );
        assert_eq!(workspace.state().items[0].remaining, 1);
    }

    #[test]
    fn unknown_in_review_restarts_and_logs() {
        let quiz = QuizSettings::default();
        let mut workspace = single(Zone::Review, &quiz);
        ask(&mut workspace, Zone::Review, &quiz);
        workspace.submit(Zone::Review, String::new()).unwrap();
        let outcome = workspace
            .rate(
                Zone::Review,
                Rating::Unknown,
                &quiz,
                &WorkspaceSettings::default(),
                now(),
            )
            .unwrap();
        assert!(outcome.settlement.is_none());
        let record = outcome.record.unwrap();
        assert_eq!(record.question_type, QuestionType::Comprehensive);
        assert_eq!((record.sentence, record.answer), (None, None));
        let item = &workspace.state().items[0];
        assert_eq!(item.remaining, 4);
        assert_eq!(
            item.progress,
            Progress {
                stage: 0,
                passes: 1,
                lapses: 1
            }
        );
    }

    #[test]
    fn logs_overridden_prediction_with_sentence_and_input() {
        let quiz = QuizSettings {
            kanji_order: vec![QuestionType::Reading],
            ..QuizSettings::default()
        };
        let mut workspace = single(Zone::Learning, &quiz);
        ask(&mut workspace, Zone::Learning, &quiz);
        workspace.submit(Zone::Learning, "ちがう".into()).unwrap();
        let outcome = workspace
            .rate(
                Zone::Learning,
                Rating::Known,
                &quiz,
                &WorkspaceSettings::default(),
                now(),
            )
            .unwrap();
        let record = outcome.record.unwrap();
        assert_eq!(record.prediction, Some(Rating::Unknown));
        assert_eq!(record.sentence.as_deref(), Some("[[諦める]]だ"));
        assert_eq!(record.answer.as_deref(), Some("ちがう"));
        assert!(outcome.settlement.is_some());
    }

    #[test]
    fn rating_without_question_fails() {
        let quiz = QuizSettings::default();
        let mut workspace = single(Zone::Review, &quiz);
        let error = workspace.rate(
            Zone::Review,
            Rating::Known,
            &quiz,
            &WorkspaceSettings::default(),
            now(),
        );
        assert_eq!(error.unwrap_err(), WorkspaceError::NoQuestion);
    }

    #[test]
    fn skip_settles_at_max_stage() {
        let quiz = QuizSettings::default();
        let mut workspace = single(Zone::Learning, &quiz);
        ask(&mut workspace, Zone::Learning, &quiz);
        let settlement = workspace
            .skip(Zone::Learning, &WorkspaceSettings::default())
            .unwrap();
        assert_eq!(settlement.progress.stage, 7);
    }

    #[test]
    fn report_returns_the_shown_sentence() {
        let quiz = QuizSettings::default();
        let mut workspace = single(Zone::Learning, &quiz);
        ask(&mut workspace, Zone::Learning, &quiz);
        let reported = workspace.report_sentence(Zone::Learning).unwrap();
        assert_eq!(
            reported,
            ReportedSentence {
                key: key("諦める"),
                marked: "[[諦める]]だ".into()
            }
        );
        assert_eq!(
            workspace.report_sentence(Zone::Learning).unwrap_err(),
            WorkspaceError::NoQuestion
        );
    }
}
