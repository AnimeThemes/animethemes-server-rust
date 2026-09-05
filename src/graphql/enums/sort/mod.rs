use sea_orm::{EntityTrait, Select};

use crate::graphql::cursor::CursorSort;

pub mod content;
pub mod document;
pub mod list;
pub mod user;

pub trait GraphQLSort {
    type Entity: EntityTrait;

    fn cursor_sort(&self) -> Option<CursorSort<<Self::Entity as EntityTrait>::Column>>;
    fn apply_sort(&self, query: Select<Self::Entity>) -> Select<Self::Entity>;
}
