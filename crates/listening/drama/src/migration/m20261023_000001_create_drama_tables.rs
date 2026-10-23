use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Character::Table)
                    .col(
                        ColumnDef::new(Character::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Character::Name).text().not_null())
                    .col(ColumnDef::new(Character::Kana).text().not_null())
                    .col(ColumnDef::new(Character::Prompt).text().null())
                    .col(ColumnDef::new(Character::Gender).text().not_null())
                    .col(ColumnDef::new(Character::Age).integer().not_null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("character_name")
                    .table(Character::Table)
                    .col(Character::Name)
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Reference::Table)
                    .col(ColumnDef::new(Reference::CharacterId).integer().not_null())
                    .col(ColumnDef::new(Reference::Language).text().not_null())
                    .col(ColumnDef::new(Reference::Audio).blob().not_null())
                    .col(ColumnDef::new(Reference::Text).text().not_null())
                    .primary_key(
                        Index::create()
                            .col(Reference::CharacterId)
                            .col(Reference::Language),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Reference::Table, Reference::CharacterId)
                            .to(Character::Table, Character::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(World::Table)
                    .col(
                        ColumnDef::new(World::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(World::Name).text().not_null())
                    .col(ColumnDef::new(World::Setting).text().not_null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("world_name")
                    .table(World::Table)
                    .col(World::Name)
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Cast::Table)
                    .col(ColumnDef::new(Cast::WorldId).integer().not_null())
                    .col(ColumnDef::new(Cast::CharacterId).integer().not_null())
                    .col(ColumnDef::new(Cast::Role).text().not_null())
                    .col(ColumnDef::new(Cast::Description).text().null())
                    .primary_key(Index::create().col(Cast::WorldId).col(Cast::CharacterId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(Cast::Table, Cast::WorldId)
                            .to(World::Table, World::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Cast::Table, Cast::CharacterId)
                            .to(Character::Table, Character::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("cast_character_id")
                    .table(Cast::Table)
                    .col(Cast::CharacterId)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("cast_one_narrator")
                    .table(Cast::Table)
                    .col(Cast::WorldId)
                    .unique()
                    .and_where(Expr::col(Cast::Role).eq("narrator"))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Episode::Table)
                    .col(
                        ColumnDef::new(Episode::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Episode::Uuid).uuid().not_null())
                    .col(ColumnDef::new(Episode::WorldId).integer().not_null())
                    .col(ColumnDef::new(Episode::Number).double().not_null())
                    .col(ColumnDef::new(Episode::Title).text().not_null())
                    .col(ColumnDef::new(Episode::Summary).text().not_null())
                    .col(
                        ColumnDef::new(Episode::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Episode::Table, Episode::WorldId)
                            .to(World::Table, World::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("episode_uuid")
                    .table(Episode::Table)
                    .col(Episode::Uuid)
                    .unique()
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("episode_world_id")
                    .table(Episode::Table)
                    .col(Episode::WorldId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Line::Table)
                    .col(
                        ColumnDef::new(Line::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Line::EpisodeId).integer().not_null())
                    .col(ColumnDef::new(Line::Speaker).text().not_null())
                    .col(ColumnDef::new(Line::Voice).text().not_null())
                    .col(ColumnDef::new(Line::Ja).text().not_null())
                    .col(ColumnDef::new(Line::Zh).text().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Line::Table, Line::EpisodeId)
                            .to(Episode::Table, Episode::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("line_episode_id")
                    .table(Line::Table)
                    .col(Line::EpisodeId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(LineAudio::Table)
                    .col(ColumnDef::new(LineAudio::LineId).integer().not_null())
                    .col(ColumnDef::new(LineAudio::Language).text().not_null())
                    .col(ColumnDef::new(LineAudio::Audio).blob().not_null())
                    .primary_key(
                        Index::create()
                            .col(LineAudio::LineId)
                            .col(LineAudio::Language),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(LineAudio::Table, LineAudio::LineId)
                            .to(Line::Table, Line::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Character {
    Table,
    Id,
    Name,
    Kana,
    Prompt,
    Gender,
    Age,
}

#[derive(DeriveIden)]
enum Reference {
    Table,
    CharacterId,
    Language,
    Audio,
    Text,
}

#[derive(DeriveIden)]
enum World {
    Table,
    Id,
    Name,
    Setting,
}

#[derive(DeriveIden)]
enum Cast {
    Table,
    WorldId,
    CharacterId,
    Role,
    Description,
}

#[derive(DeriveIden)]
enum Episode {
    Table,
    Id,
    Uuid,
    WorldId,
    Number,
    Title,
    Summary,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Line {
    Table,
    Id,
    EpisodeId,
    Speaker,
    Voice,
    Ja,
    Zh,
}

#[derive(DeriveIden)]
enum LineAudio {
    Table,
    LineId,
    Language,
    Audio,
}
