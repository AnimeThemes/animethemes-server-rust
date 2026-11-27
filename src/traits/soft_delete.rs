use sea_orm::{
    ColumnTrait, ConnectionTrait, EntityTrait, Iterable, ModelTrait, PrimaryKeyToColumn,
    QueryFilter,
};
use sea_query::Expr;

use crate::AppError;

pub trait SoftDeleteEntity: EntityTrait {
    fn deleted_at_column() -> Self::Column;
}

pub trait SoftDeleteModel: ModelTrait + Sync
where
    Self::Entity: SoftDeleteEntity,
{
    fn soft_delete<C>(&self, db: &C) -> impl Future<Output = Result<(), AppError>> + Send
    where
        C: ConnectionTrait,
    {
        async move {
            let mut query = Self::Entity::update_many()
                .col_expr(
                    Self::Entity::deleted_at_column(),
                    Expr::value(time::OffsetDateTime::now_utc()),
                )
                .filter(Self::Entity::deleted_at_column().is_null());

            for pk in <Self::Entity as EntityTrait>::PrimaryKey::iter() {
                let column = pk.into_column();

                query = query.filter(column.eq(self.get(column)));
            }

            query.exec(db).await?;

            Ok(())
        }
    }

    fn restore<C>(&self, db: &C) -> impl Future<Output = Result<(), AppError>> + Send
    where
        C: ConnectionTrait,
    {
        async move {
            let mut query = Self::Entity::update_many().col_expr(
                Self::Entity::deleted_at_column(),
                Expr::value(Option::<time::OffsetDateTime>::None),
            );

            for pk in <Self::Entity as EntityTrait>::PrimaryKey::iter() {
                let column = pk.into_column();

                query = query.filter(column.eq(self.get(column)));
            }

            query.exec(db).await?;

            Ok(())
        }
    }
}

impl<M> SoftDeleteModel for M
where
    M: ModelTrait + Sync,
    M::Entity: SoftDeleteEntity,
{
}
