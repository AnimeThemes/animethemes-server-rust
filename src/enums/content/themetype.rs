use async_graphql::Enum;
use sea_orm::entity::prelude::*;

use crate::enums::LocalizedEnum;

#[derive(Enum, Debug, Copy, Clone, Eq, EnumIter, PartialEq, DeriveActiveEnum)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum ThemeType {
    /// Opening
    #[sea_orm(num_value = 0)]
    OP,

    /// Ending
    #[sea_orm(num_value = 1)]
    ED,

    /// Insert Song
    #[sea_orm(num_value = 2)]
    IN,
}

impl LocalizedEnum for ThemeType {
    fn localize(&self) -> &str {
        match self {
            ThemeType::OP => "OP",
            ThemeType::ED => "ED",
            ThemeType::IN => "IN",
        }
    }
}
