use chrono::{DateTime, FixedOffset};
use serde::Serialize;
use uuid::Uuid;

use crate::LineView;

/// An episode in a world's list.
#[derive(Clone, Debug, PartialEq, Serialize, specta::Type)]
pub struct EpisodeSummary {
    pub uuid: Uuid,
    pub number: f64,
    pub title: String,
    pub summary: String,
    pub created_at: DateTime<FixedOffset>,
    pub lines: u32,
    /// Characters of every line's Japanese and Chinese as stored, reading
    /// marks included: what the script context would hand over.
    pub char_count: u32,
    /// Lines without Japanese audio.
    pub missing_ja: u32,
    /// Lines without Chinese audio.
    pub missing_zh: u32,
}

/// One episode with its lines.
#[derive(Clone, Debug, PartialEq, Serialize, specta::Type)]
pub struct EpisodeDetail {
    pub uuid: Uuid,
    pub world: String,
    pub number: f64,
    pub title: String,
    pub summary: String,
    pub created_at: DateTime<FixedOffset>,
    pub lines: Vec<LineView>,
}
