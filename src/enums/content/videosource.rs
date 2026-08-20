use async_graphql::Enum;
use sea_orm::entity::prelude::*;

use crate::enums::LocalizedEnum;

#[derive(Enum, Debug, Copy, Clone, Eq, EnumIter, PartialEq, DeriveActiveEnum)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum VideoSource {
    #[sea_orm(num_value = 0)]
    WEB,

    #[sea_orm(num_value = 1)]
    RAW,

    #[sea_orm(num_value = 2)]
    BD,

    #[sea_orm(num_value = 3)]
    DVD,

    #[sea_orm(num_value = 4)]
    VHS,

    #[sea_orm(num_value = 5)]
    LD,
}

impl LocalizedEnum for VideoSource {
    fn localize(&self) -> &str {
        match self {
            VideoSource::WEB => "WEB",
            VideoSource::RAW => "RAW",
            VideoSource::BD => "BD",
            VideoSource::DVD => "DVD",
            VideoSource::VHS => "VHS",
            VideoSource::LD => "LD",
        }
    }
}
