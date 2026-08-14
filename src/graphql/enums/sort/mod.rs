use sea_orm::{EntityTrait, Select};

pub mod content;
pub mod document;
pub mod list;

pub trait GraphQLSort<E: EntityTrait> {
    fn apply_sort(&self, query: Select<E>) -> Select<E>;
}
