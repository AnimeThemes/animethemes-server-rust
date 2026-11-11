use chrono::Utc;
use sea_orm::Condition;
use sea_orm::entity::prelude::*;

use crate::entities::auth::user;
use crate::entities::content::entry;
use crate::traits::sortable::Sortable;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "favorites")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub favoriteable_type: FavoriteableType,
    pub favoriteable_id: u64,
    pub user_id: u64,
    pub position: i32,
    #[sea_orm(column_type = "Timestamp")]
    pub created_at: chrono::DateTime<Utc>,

    #[sea_orm(belongs_to, from = "favoriteable_id", to = "id")]
    pub favoriteable: BelongsTo<entry::Entity>,
    #[sea_orm(belongs_to, from = "user_id", to = "id")]
    pub user: BelongsTo<user::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, Copy, Clone, Eq, EnumIter, PartialEq, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::N(255))")]
pub enum FavoriteableType {
    #[sea_orm(string_value = "entry")]
    Entry,
}

impl Sortable for Model {
    fn order_column() -> <Self::Entity as EntityTrait>::Column {
        self::Column::Position
    }

    fn sort_scope(&self) -> sea_orm::Condition {
        Condition::all()
            .add(self::Column::UserId.eq(self.user_id))
            .add(self::Column::FavoriteableType.eq(self.favoriteable_type))
    }
}
