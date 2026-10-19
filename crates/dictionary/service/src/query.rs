//! Reading the dictionary: counts, pages of rows, one sense in full, and
//! senses with their progress.

use dictionary_word::store::query::{filter_senses, search, sense_detail, stats};

use crate::{
    ContentFilter, DictionaryError, DictionaryService, DictionaryStats, SearchPage, SenseDetail,
    SenseKey, SenseOrder, SenseProgress,
};

impl DictionaryService {
    pub async fn stats(&self) -> Result<DictionaryStats, DictionaryError> {
        Ok(stats::count_stats(&self.db).await?)
    }

    /// Rows matching the filter, `limit` of them starting at `offset`; a row
    /// is what one exported line would be.
    pub async fn search(
        &self,
        filter: &ContentFilter,
        offset: u32,
        limit: u32,
    ) -> Result<SearchPage, DictionaryError> {
        Ok(search::search(&self.db, filter, offset, limit).await?)
    }

    /// Everything about one sense; `None` if it is gone.
    pub async fn sense_detail(
        &self,
        key: &SenseKey,
    ) -> Result<Option<SenseDetail>, DictionaryError> {
        Ok(sense_detail::sense_detail(&self.db, key).await?)
    }

    /// Senses matching the filter with their progress, except the excluded
    /// ones, in the given order, at most `limit` of them.
    pub async fn filter_senses(
        &self,
        filter: &ContentFilter,
        exclude: &[SenseKey],
        order: SenseOrder,
        limit: Option<u32>,
    ) -> Result<Vec<SenseProgress>, DictionaryError> {
        Ok(filter_senses::filter_senses(&self.db, filter, exclude, order, limit).await?)
    }
}
