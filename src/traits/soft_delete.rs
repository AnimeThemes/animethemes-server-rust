use sea_orm::EntityTrait;

pub trait SoftDeleteEntity: EntityTrait {
    fn deleted_at_column() -> Self::Column;
}
