use async_graphql::{
    Context, OutputType, Result,
    connection::{Connection, Edge, EmptyFields, OpaqueCursor, query},
};
use axum::Error;
use sea_orm::{
    ColumnTrait, Condition, DatabaseConnection, EntityTrait, ModelTrait, Order, QueryFilter,
    QueryOrder, QuerySelect, Select, Value,
};
use serde::{Deserialize, Serialize};

use crate::graphql::inputs::pagination_input::PaginationInput;

const DEFAULT_PAGE_SIZE: usize = 15;

#[derive(Debug, Clone)]
pub struct CursorSort<C> {
    pub column: C,
    pub order: Order,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationCursor {
    pub values: Vec<Value>,
}

pub async fn cursor_paginate<E, Node>(
    builder: Select<E>,
    ctx: &Context<'_>,
    sorts: Vec<CursorSort<E::Column>>,
    pagination: Option<PaginationInput>,
) -> Result<Connection<OpaqueCursor<PaginationCursor>, Node, EmptyFields, EmptyFields>>
where
    E: EntityTrait,
    E::Column: Copy,
    Node: OutputType + From<E::Model>,
{
    let db = ctx.data::<DatabaseConnection>()?;
    let pagination = pagination.unwrap_or_default();

    query(
        pagination.after,
        pagination.before,
        pagination.first,
        pagination.last,
        move |after: Option<OpaqueCursor<PaginationCursor>>,
              before: Option<OpaqueCursor<PaginationCursor>>,
              first: Option<usize>,
              last: Option<usize>| async move {
            let has_after = after.is_some();
            let has_before = before.is_some();
            let is_backward = last.is_some();

            let limit = first.or(last).unwrap_or(DEFAULT_PAGE_SIZE);

            let mut builder = builder;

            let build_condition = |values: &[Value], is_after: bool| -> Result<Condition, Error> {
                if values.len() != sorts.len() {
                    return Err(Error::new("Invalid pagination cursor"));
                }

                let mut condition = Condition::any();

                for i in 0..sorts.len() {
                    let mut branch = Condition::all();

                    for j in 0..i {
                        branch = branch.add(ColumnTrait::eq(&sorts[j].column, values[j].clone()));
                    }

                    let comparison = match (&sorts[i].order, is_after) {
                        (Order::Asc, true) => ColumnTrait::gt(&sorts[i].column, values[i].clone()),
                        (Order::Desc, true) => ColumnTrait::lt(&sorts[i].column, values[i].clone()),
                        (Order::Asc, false) => ColumnTrait::lt(&sorts[i].column, values[i].clone()),
                        (Order::Desc, false) => {
                            ColumnTrait::gt(&sorts[i].column, values[i].clone())
                        }
                        _ => {
                            return Err(Error::new("Unsupported sort order"));
                        }
                    };

                    branch = branch.add(comparison);
                    condition = condition.add(branch);
                }

                Ok(condition)
            };

            if let Some(OpaqueCursor(after)) = after {
                builder = builder.filter(build_condition(&after.values, true)?);
            }

            if let Some(OpaqueCursor(before)) = before {
                builder = builder.filter(build_condition(&before.values, false)?);
            }

            QueryOrder::query(&mut builder).clear_order_by();

            for sort in &sorts {
                let order = match (&sort.order, is_backward) {
                    (Order::Asc, false) => Order::Asc,
                    (Order::Desc, false) => Order::Desc,

                    (Order::Asc, true) => Order::Desc,
                    (Order::Desc, true) => Order::Asc,

                    _ => {
                        return Err(Error::new("Unsupported sort order"));
                    }
                };

                builder = builder.order_by(sort.column, order);
            }

            let mut models = builder
                .limit(limit as u64 + 1)
                .all(db)
                .await
                .map_err(|error| Error::new(error.to_string()))?;

            let has_extra_item = models.len() > limit;

            if has_extra_item {
                models.pop();
            }

            if is_backward {
                models.reverse();
            }

            let has_previous_page = if is_backward {
                has_extra_item
            } else {
                has_after
            };

            let has_next_page = if is_backward {
                has_before
            } else {
                has_extra_item
            };

            let mut connection = Connection::new(has_previous_page, has_next_page);

            connection.edges.extend(models.into_iter().map(|model| {
                let values = sorts.iter().map(|sort| model.get(sort.column)).collect();

                Edge::new(OpaqueCursor(PaginationCursor { values }), Node::from(model))
            }));

            Ok::<_, Error>(connection)
        },
    )
    .await
}
