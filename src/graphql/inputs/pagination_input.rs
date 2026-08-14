use async_graphql::InputObject;

#[derive(InputObject, Default)]
pub struct PaginationInput {
    pub after: Option<String>,
    pub before: Option<String>,
    pub first: Option<i32>,
    pub last: Option<i32>,
}
