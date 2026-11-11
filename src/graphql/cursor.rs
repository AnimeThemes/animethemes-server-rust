use async_graphql::connection::{Connection, Edge, EmptyFields, OpaqueCursor, query};
use async_graphql::{Context, OutputType, Result};
use axum::Error;
use sea_orm::{
    ColumnTrait, Condition, DatabaseConnection, EntityTrait, ExprTrait, ModelTrait, Order,
    QueryFilter, QueryOrder, QuerySelect, Select, Value,
};
use sea_query::{Expr, Func};
use serde::{Deserialize, Serialize};

use crate::graphql::inputs::pagination_input::PaginationInput;

const DEFAULT_PAGE_SIZE: usize = 15;

#[derive(Debug, Clone)]
pub enum CursorSortKey<C> {
    Column(C),
    Random { id: C },
}

#[derive(Debug, Clone)]
pub struct CursorSort<C> {
    pub key: CursorSortKey<C>,
    pub order: Order,
}

fn random_expr(value: Expr, seed: u64) -> Expr {
    Func::md5(Expr::cust_with_exprs(
        "CONCAT(?, ':', ?)",
        [value, Expr::val(seed.to_string())],
    ))
    .into()
}

impl<C: ColumnTrait + Copy> CursorSort<C> {
    fn expression(&self, seed: u64) -> Expr {
        match &self.key {
            CursorSortKey::Column(column) => (*column).into_expr(),

            CursorSortKey::Random { id } => random_expr((*id).into_expr(), seed),
        }
    }

    fn cursor_expression(&self, value: Value, seed: u64) -> Expr {
        match &self.key {
            CursorSortKey::Column(_) => Expr::val(value),

            CursorSortKey::Random { .. } => random_expr(Expr::val(value), seed),
        }
    }

    fn source_column(&self) -> C {
        match &self.key {
            CursorSortKey::Column(column) => *column,
            CursorSortKey::Random { id } => *id,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationCursor {
    pub values: Vec<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seed: Option<u64>,
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

            if sorts.is_empty() {
                return Err(Error::new("At least one cursor sort is required"));
            }

            let has_random_sort = sorts
                .iter()
                .any(|sort| matches!(&sort.key, CursorSortKey::Random { .. }));

            let after_seed = after.as_ref().and_then(|OpaqueCursor(cursor)| cursor.seed);
            let before_seed = before.as_ref().and_then(|OpaqueCursor(cursor)| cursor.seed);

            if has_random_sort {
                if (has_after && after_seed.is_none()) || (has_before && before_seed.is_none()) {
                    return Err(Error::new(
                        "After and before are prohibited when using Random sort",
                    ));
                }

                if let (Some(a), Some(b)) = (after_seed, before_seed) {
                    if a != b {
                        return Err(Error::new("Pagination cursors have different seeds"));
                    }
                }
            }

            let seed = if has_random_sort {
                after_seed
                    .or(before_seed)
                    .unwrap_or_else(rand::random::<u64>)
            } else {
                0
            };

            let cursor_seed = has_random_sort.then_some(seed);

            let mut builder = builder;

            let build_condition = |values: &[Value], is_after: bool| -> Result<Condition, Error> {
                if values.len() != sorts.len() {
                    return Err(Error::new("Invalid pagination cursor"));
                }

                let mut condition = Condition::any();

                for i in 0..sorts.len() {
                    let mut branch = Condition::all();

                    for j in 0..i {
                        let left = sorts[j].expression(seed);

                        let right = sorts[j].cursor_expression(values[j].clone(), seed);

                        branch = branch.add(left.eq(right));
                    }

                    let left = sorts[i].expression(seed);

                    let right = sorts[i].cursor_expression(values[i].clone(), seed);

                    let comparison = match (&sorts[i].order, is_after) {
                        (Order::Asc, true) => left.gt(right),
                        (Order::Desc, true) => left.lt(right),
                        (Order::Asc, false) => left.lt(right),
                        (Order::Desc, false) => left.gt(right),
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

                builder = builder.order_by(sort.expression(seed), order);
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
                let values = sorts
                    .iter()
                    .map(|sort| model.get(sort.source_column()))
                    .collect();

                Edge::new(
                    OpaqueCursor(PaginationCursor {
                        values,
                        seed: cursor_seed,
                    }),
                    Node::from(model),
                )
            }));

            Ok::<_, Error>(connection)
        },
    )
    .await
}
