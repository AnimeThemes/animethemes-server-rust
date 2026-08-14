use chrono::Utc;
use sea_orm::entity::prelude::*;

use crate::entities::{
    SoftDeleteEntity,
    content::{performance, synonym},
};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "artists")]
pub struct Model {
    #[sea_orm(primary_key, column_name = "artist_id")]
    pub id: u64,
    pub slug: String,
    pub name: String,
    pub name_native: Option<String>,
    pub information: Option<String>,
    #[sea_orm(column_type = "Timestamp")]
    pub created_at: Option<chrono::DateTime<Utc>>,
    #[sea_orm(column_type = "Timestamp")]
    pub updated_at: Option<chrono::DateTime<Utc>>,
    #[sea_orm(column_type = "Timestamp")]
    pub deleted_at: Option<chrono::DateTime<Utc>>,

    #[sea_orm(
        has_many,
        // Unsupported
        //on_condition = r#"synonym::Column::SynonymableType.eq("artist")"#
    )]
    pub synonyms: HasMany<synonym::Entity>,

    #[sea_orm(has_many, relation_enum = "Performances", via_rel = "Artist")]
    pub performances: HasMany<performance::Entity>,

    #[sea_orm(has_many, relation_enum = "MemberPerformances", via_rel = "Member")]
    pub member_performances: HasMany<performance::Entity>,

    #[sea_orm(self_ref, via = "artist_members", from = "Artist", to = "Member")]
    pub members: HasMany<Entity>,

    #[sea_orm(self_ref, via = "artist_members", reverse)]
    pub groups: HasMany<Entity>,
}

impl SoftDeleteEntity for Entity {
    fn deleted_at_column() -> Self::Column {
        Column::DeletedAt
    }
}

// pub struct ArtistPerformances;

// impl Linked for ArtistPerformances {
//     type FromEntity = Entity;
//     type ToEntity = performance::Entity;

//     fn link(&self) -> Vec<LinkDef> {
//         vec![Relation::Performances.def().into()]
//     }
// }

// pub struct MemberPerformances;

// impl Linked for MemberPerformances {
//     type FromEntity = Entity;
//     type ToEntity = performance::Entity;

//     fn link(&self) -> Vec<LinkDef> {
//         vec![Relation::MemberPerformances.def().into()]
//     }
// }

impl ActiveModelBehavior for ActiveModel {}
