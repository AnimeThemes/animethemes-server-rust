use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, ConnectionTrait, DbErr, EntityTrait, ExprTrait,
    IntoActiveModel, ModelTrait, QueryFilter, QueryOrder, QuerySelect, TransactionSession,
    TransactionTrait, sea_query::Expr,
};

pub trait Sortable:
    ModelTrait + Clone + Sync + IntoActiveModel<<Self::Entity as EntityTrait>::ActiveModel>
where
    Self::Entity: EntityTrait<Model = Self>,
    <Self::Entity as EntityTrait>::ActiveModel: Send,
{
    fn order_column() -> <Self::Entity as EntityTrait>::Column;

    fn sort_scope(&self) -> Condition {
        Condition::all()
    }

    fn current_order(&self) -> i32 {
        self.get(Self::order_column()).unwrap::<i32>()
    }

    fn set_sort_order<'a, C>(
        &'a self,
        db: &'a C,
        position: i32,
    ) -> impl Future<Output = Result<Self, DbErr>> + Send + 'a
    where
        C: ConnectionTrait + Sync + 'a,
    {
        async move {
            let mut active_model = self.clone().into_active_model();

            active_model.set(Self::order_column(), position.into());

            Ok(active_model.update(db).await?)
        }
    }

    fn move_to<'a, C>(
        &'a self,
        db: &'a C,
        position: i32,
    ) -> impl Future<Output = Result<Self, DbErr>> + Send + 'a
    where
        C: ConnectionTrait + TransactionTrait + Sync + 'a,
        <C as TransactionTrait>::Transaction: Send,
    {
        async move {
            let current = self.current_order();

            if current == position {
                return Ok(self.clone());
            }

            let txn = db.begin().await?;

            let column = Self::order_column();

            if position < current {
                Self::Entity::update_many()
                    .col_expr(column, Expr::col(column).add(1))
                    .filter(self.sort_scope())
                    .filter(column.gte(position))
                    .filter(column.lt(current))
                    .exec(db)
                    .await?;
            } else {
                Self::Entity::update_many()
                    .col_expr(column, Expr::col(column).sub(1))
                    .filter(self.sort_scope())
                    .filter(column.gt(current))
                    .filter(column.lte(position))
                    .exec(db)
                    .await?;
            }

            let updated = self.set_sort_order(db, position).await?;

            txn.commit().await?;

            Ok(updated)
        }
    }

    fn move_to_start<'a, C>(
        &'a self,
        db: &'a C,
    ) -> impl Future<Output = Result<Self, DbErr>> + Send + 'a
    where
        C: ConnectionTrait + TransactionTrait + Sync + 'a,
        <C as TransactionTrait>::Transaction: Send,
    {
        async move { Ok(self.move_to(db, 1).await?) }
    }

    fn move_to_end<'a, C>(
        &'a self,
        db: &'a C,
    ) -> impl Future<Output = Result<Self, DbErr>> + Send + 'a
    where
        C: ConnectionTrait + TransactionTrait + Sync + 'a,
        <C as TransactionTrait>::Transaction: Send,
    {
        async move {
            let column = Self::order_column();

            let max_order: Option<i32> = Self::Entity::find()
                .filter(self.sort_scope())
                .select_only()
                .column(column)
                .order_by_desc(column)
                .into_tuple()
                .one(db)
                .await?;

            let Some(max_order) = max_order else {
                return Ok(self.clone());
            };

            Ok(self.move_to(db, max_order).await?)
        }
    }

    fn reorder_after_delete<'a, C>(
        &'a self,
        db: &'a C,
    ) -> impl Future<Output = Result<(), DbErr>> + Send + 'a
    where
        C: ConnectionTrait + Sync + 'a,
    {
        async move {
            let column = Self::order_column();
            let current = self.current_order();

            Self::Entity::update_many()
                .col_expr(column, Expr::col(column).sub(1))
                .filter(self.sort_scope())
                .filter(column.gt(current))
                .exec(db)
                .await?;

            Ok(())
        }
    }
}
