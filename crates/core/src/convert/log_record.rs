use dictionary::{LogPrediction, LogQuestionType, LogRating, LogRecord, LogSense};
use study::{AnswerRecord, QuestionType, Rating};

pub fn to_dictionary(record: AnswerRecord) -> LogRecord {
    LogRecord {
        time: record.time,
        sense: LogSense {
            writing: record.key.writing,
            origin: record.key.origin,
            kana: record.key.kana,
            primary_gloss: record.key.primary_gloss,
        },
        question_type: question_type(record.question_type),
        sentence: record.sentence,
        answer: record.answer,
        prediction: prediction(record.prediction),
        similarity: None,
        embedding_model: None,
        rating: rating(record.rating),
    }
}

fn question_type(value: QuestionType) -> LogQuestionType {
    match value {
        QuestionType::Sentence => LogQuestionType::Sentence,
        QuestionType::Translation => LogQuestionType::Translation,
        QuestionType::Reading => LogQuestionType::Reading,
        QuestionType::Comprehensive => LogQuestionType::Comprehensive,
    }
}

fn prediction(value: Option<Rating>) -> LogPrediction {
    match value {
        Some(Rating::Known) => LogPrediction::Known,
        Some(Rating::Vague) => LogPrediction::Vague,
        Some(Rating::Unknown) => LogPrediction::Unknown,
        None => LogPrediction::Absent,
    }
}

fn rating(value: Rating) -> LogRating {
    match value {
        Rating::Known => LogRating::Known,
        Rating::Vague => LogRating::Vague,
        Rating::Unknown => LogRating::Unknown,
    }
}
