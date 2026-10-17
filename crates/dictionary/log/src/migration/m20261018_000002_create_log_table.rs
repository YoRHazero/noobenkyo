use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Log::Table)
                    .col(
                        ColumnDef::new(Log::Time)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Log::Writing).text().not_null())
                    .col(ColumnDef::new(Log::Origin).text().not_null())
                    .col(ColumnDef::new(Log::Kana).text().not_null())
                    .col(ColumnDef::new(Log::PrimaryGloss).text().not_null())
                    .col(ColumnDef::new(Log::QuestionType).text().not_null())
                    .col(ColumnDef::new(Log::Sentence).text().null())
                    .col(ColumnDef::new(Log::Answer).text().null())
                    .col(ColumnDef::new(Log::Prediction).text().not_null())
                    .col(ColumnDef::new(Log::Similarity).double().null())
                    .col(ColumnDef::new(Log::EmbeddingModel).text().null())
                    .col(ColumnDef::new(Log::Rating).text().not_null())
                    .primary_key(
                        Index::create()
                            .col(Log::Time)
                            .col(Log::Writing)
                            .col(Log::Origin)
                            .col(Log::Kana)
                            .col(Log::PrimaryGloss)
                            .col(Log::QuestionType),
                    )
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Log {
    Table,
    Time,
    Writing,
    Origin,
    Kana,
    PrimaryGloss,
    QuestionType,
    Sentence,
    Answer,
    Prediction,
    Similarity,
    EmbeddingModel,
    Rating,
}
