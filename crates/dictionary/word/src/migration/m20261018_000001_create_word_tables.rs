use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Word::Table)
                    .col(
                        ColumnDef::new(Word::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Word::Writing).text().not_null())
                    .col(ColumnDef::new(Word::Origin).text().not_null())
                    .col(ColumnDef::new(Word::Stratum).text().not_null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("word_writing_origin")
                    .table(Word::Table)
                    .col(Word::Writing)
                    .col(Word::Origin)
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Reading::Table)
                    .col(
                        ColumnDef::new(Reading::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Reading::WordId).integer().not_null())
                    .col(ColumnDef::new(Reading::Kana).text().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Reading::Table, Reading::WordId)
                            .to(Word::Table, Word::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("reading_word_id_kana")
                    .table(Reading::Table)
                    .col(Reading::WordId)
                    .col(Reading::Kana)
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Sense::Table)
                    .col(
                        ColumnDef::new(Sense::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Sense::ReadingId).integer().not_null())
                    .col(ColumnDef::new(Sense::PrimaryGloss).text().not_null())
                    .col(ColumnDef::new(Sense::SecondaryGlosses).json().not_null())
                    .col(ColumnDef::new(Sense::Stage).integer().not_null())
                    .col(ColumnDef::new(Sense::Passes).integer().not_null())
                    .col(ColumnDef::new(Sense::Lapses).integer().not_null())
                    .col(
                        ColumnDef::new(Sense::ModifiedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Sense::Table, Sense::ReadingId)
                            .to(Reading::Table, Reading::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("sense_reading_id_primary_gloss")
                    .table(Sense::Table)
                    .col(Sense::ReadingId)
                    .col(Sense::PrimaryGloss)
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Label::Table)
                    .col(ColumnDef::new(Label::SenseId).integer().not_null())
                    .col(ColumnDef::new(Label::Text).text().not_null())
                    .primary_key(Index::create().col(Label::SenseId).col(Label::Text))
                    .foreign_key(
                        ForeignKey::create()
                            .from(Label::Table, Label::SenseId)
                            .to(Sense::Table, Sense::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("label_text")
                    .table(Label::Table)
                    .col(Label::Text)
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Sentence::Table)
                    .col(
                        ColumnDef::new(Sentence::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Sentence::SenseId).integer().not_null())
                    .col(ColumnDef::new(Sentence::Marked).text().not_null())
                    .col(ColumnDef::new(Sentence::Translation).text().null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Sentence::Table, Sentence::SenseId)
                            .to(Sense::Table, Sense::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("sentence_sense_id")
                    .table(Sentence::Table)
                    .col(Sentence::SenseId)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Word {
    Table,
    Id,
    Writing,
    Origin,
    Stratum,
}

#[derive(DeriveIden)]
enum Reading {
    Table,
    Id,
    WordId,
    Kana,
}

#[derive(DeriveIden)]
enum Sense {
    Table,
    Id,
    ReadingId,
    PrimaryGloss,
    SecondaryGlosses,
    Stage,
    Passes,
    Lapses,
    ModifiedAt,
}

#[derive(DeriveIden)]
enum Label {
    Table,
    SenseId,
    Text,
}

#[derive(DeriveIden)]
enum Sentence {
    Table,
    Id,
    SenseId,
    Marked,
    Translation,
}
