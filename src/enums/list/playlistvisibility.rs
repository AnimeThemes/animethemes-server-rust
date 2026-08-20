use async_graphql::Enum;
use sea_orm::{DeriveActiveEnum, EnumIter};

use crate::enums::LocalizedEnum;

#[derive(Enum, Debug, Copy, Clone, Eq, EnumIter, PartialEq, DeriveActiveEnum)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum PlaylistVisibility {
    #[sea_orm(num_value = 0)]
    Public,

    #[sea_orm(num_value = 1)]
    Private,

    #[sea_orm(num_value = 2)]
    Unlisted,
}

impl LocalizedEnum for PlaylistVisibility {
    fn localize(&self) -> &str {
        match self {
            PlaylistVisibility::Public => "Public",
            PlaylistVisibility::Private => "Private",
            PlaylistVisibility::Unlisted => "Unlisted",
        }
    }
}
