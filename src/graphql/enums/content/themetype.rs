use async_graphql::Enum;

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
#[graphql(remote = "crate::enums::content::themetype::ThemeType")]
pub enum ThemeType {
    /// Opening
    OP,
    /// Ending
    ED,
    /// Insert Song
    IN,
}
