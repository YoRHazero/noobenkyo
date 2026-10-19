use sea_orm::sea_query::Query;
use sea_orm::{ColumnTrait, Condition};

use super::text_condition::sense_matches;
use super::{Completeness, ContentFilter, ProgressState};
use crate::entity::{sense, sentence};

/// What a sense row must meet, on sense joined with reading and word.
/// `None` when the filter allows no sense at all.
pub fn sense_condition(filter: &ContentFilter) -> Option<Condition> {
    let with = filter.allows(Completeness::WithSentence);
    let without = filter.allows(Completeness::NoSentence);
    if !with && !without {
        return None;
    }
    let mut condition = Condition::all();
    if with != without {
        let has_sentences = Query::select()
            .column(sentence::Column::SenseId)
            .from(sentence::Entity)
            .to_owned();
        condition = condition.add(if with {
            sense::Column::Id.in_subquery(has_sentences)
        } else {
            sense::Column::Id.not_in_subquery(has_sentences)
        });
    }
    condition = match filter.progress {
        ProgressState::Any => condition,
        ProgressState::Untouched => condition
            .add(sense::Column::Passes.eq(0))
            .add(sense::Column::Stage.eq(0)),
        ProgressState::Touched => condition.add(
            Condition::any()
                .add(sense::Column::Passes.ne(0))
                .add(sense::Column::Stage.ne(0)),
        ),
    };
    if let Some(text) = filter.text() {
        condition = condition.add(sense_matches(text));
    }
    Some(condition)
}
