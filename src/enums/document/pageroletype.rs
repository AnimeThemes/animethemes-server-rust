use async_graphql::Enum;
use sea_orm::entity::prelude::*;

#[derive(Enum, Debug, Copy, Clone, Eq, EnumIter, PartialEq, DeriveActiveEnum)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum PageRoleType {
    #[sea_orm(num_value = 0)]
    Viewer,

    #[sea_orm(num_value = 1)]
    Editor,
}
