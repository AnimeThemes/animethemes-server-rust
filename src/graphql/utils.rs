use std::fmt::Display;

use async_graphql::{
    Context, OutputType, Result,
    connection::{Connection, CursorType, Edge, EmptyFields, query},
};
use axum::Error;
use chrono::{DateTime, Utc};
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, QuerySelect, Select,
    Value,
};

use crate::graphql::inputs::pagination_input::PaginationInput;

pub fn format_datetime(dt: Option<&DateTime<Utc>>, format: &str) -> Option<String> {
    Some(dt?.format(format).to_string())
}

const DEFAULT_PAGE_SIZE: usize = 15;

pub async fn cursor_paginate<E, C, Node, Cursor, GetCursor>(
    builder: Select<E>,
    ctx: &Context<'_>,
    cursor_column: C,
    pagination: Option<PaginationInput>,
    get_cursor: GetCursor,
) -> Result<Connection<Cursor, Node, EmptyFields, EmptyFields>>
where
    E: EntityTrait,
    C: ColumnTrait + Copy + Send + Sync,
    Node: OutputType + From<E::Model>,
    Cursor: CursorType + Into<Value> + Send + Sync,
    <Cursor as CursorType>::Error: Display + Send + Sync + 'static,
    GetCursor: Fn(&E::Model) -> Cursor + Send + Sync,
{
    let db = ctx.data::<DatabaseConnection>()?;

    let pagination = pagination.unwrap_or_default();

    query(
        pagination.after,
        pagination.before,
        pagination.first,
        pagination.last,
        move |after: Option<Cursor>,
              before: Option<Cursor>,
              first: Option<usize>,
              last: Option<usize>| async move {
            let mut builder = builder;

            let has_after = after.is_some();
            let has_before = before.is_some();
            let is_backward = last.is_some();

            let limit = first.or(last).unwrap_or(DEFAULT_PAGE_SIZE);

            if let Some(cursor) = after {
                builder = builder.filter(cursor_column.gt(cursor));
            }

            if let Some(cursor) = before {
                builder = builder.filter(cursor_column.lt(cursor));
            }

            builder = if is_backward {
                builder.order_by_desc(cursor_column)
            } else {
                builder.order_by_asc(cursor_column)
            };

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
                let cursor = get_cursor(&model);
                let node = Node::from(model);

                Edge::new(cursor, node)
            }));

            Ok::<_, Error>(connection)
        },
    )
    .await
}
