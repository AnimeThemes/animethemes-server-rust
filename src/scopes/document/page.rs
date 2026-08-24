use sea_orm::{
    ColumnTrait, Condition, EntityTrait, ExprTrait, QueryFilter, QuerySelect, QueryTrait,
};
use sea_query::Expr;

use crate::{
    entities::{
        auth::role,
        document::{page, page_roles},
    },
    enums::document::pageroletype::PageRoleType,
};

pub fn public_pages(user_roles: &Vec<role::Model>) -> Condition {
    let user_role_ids: Vec<u64> = user_roles.iter().map(|role| role.id).collect();

    let viewer_roles = page_roles::Entity::find()
        .select_only()
        .column(page_roles::Column::PageId)
        .filter(
            Expr::col((page_roles::Entity, page_roles::Column::PageId))
                .equals((page::Entity, page::Column::Id)),
        )
        .filter(page_roles::Column::Type.eq(PageRoleType::Viewer))
        .into_query();

    let allowed_viewer_roles = page_roles::Entity::find()
        .select_only()
        .column(page_roles::Column::PageId)
        .filter(
            Expr::col((page_roles::Entity, page_roles::Column::PageId))
                .equals((page::Entity, page::Column::Id)),
        )
        .filter(page_roles::Column::Type.eq(PageRoleType::Viewer))
        .filter(page_roles::Column::RoleId.is_in(user_role_ids))
        .into_query();

    Condition::any()
        // Public pages
        .add(Expr::not(Expr::exists(viewer_roles)))
        // OR user has one of the viewer roles
        .add(Expr::exists(allowed_viewer_roles))
}
