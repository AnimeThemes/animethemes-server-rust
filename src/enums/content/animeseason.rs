use async_graphql::Enum;
use sea_orm::entity::prelude::*;

use crate::enums::LocalizedEnum;

#[derive(Enum, Debug, Copy, Clone, Eq, EnumIter, PartialEq, DeriveActiveEnum)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum AnimeSeason {
    #[sea_orm(num_value = 0)]
    Winter,

    #[sea_orm(num_value = 1)]
    Spring,

    #[sea_orm(num_value = 2)]
    Summer,

    #[sea_orm(num_value = 3)]
    Fall,
}

impl LocalizedEnum for AnimeSeason {
    fn localize(&self) -> &str {
        match self {
            AnimeSeason::Winter => "Winter",
            AnimeSeason::Spring => "Spring",
            AnimeSeason::Summer => "Summer",
            AnimeSeason::Fall => "Fall",
        }
    }
}
