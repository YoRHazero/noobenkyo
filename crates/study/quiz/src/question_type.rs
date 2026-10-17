use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, specta::Type)]
pub enum QuestionType {
    /// 例句题: the sentence with the target in kana; answer in Chinese.
    Sentence,
    /// 翻译题: the sentence with the target blank and the glosses; answer the dictionary-form kana.
    Translation,
    /// 读音题: the sentence with the target in kanji and the glosses; answer its kana.
    Reading,
    /// 综合题: glosses, writing and kana, some covered; self-rating only.
    Comprehensive,
}
