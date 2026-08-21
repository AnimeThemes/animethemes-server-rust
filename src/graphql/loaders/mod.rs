use std::{collections::HashMap, hash::Hash};

pub mod admin;
pub mod auth;
pub mod content;
pub mod document;
pub mod list;
pub mod loaders;
pub mod user;

pub fn group_by_query<'a, K, Q>(
    keys: &'a [K],
    get_query: impl Fn(&K) -> Q,
) -> HashMap<Q, Vec<&'a K>>
where
    Q: Eq + Hash,
{
    let mut groups = HashMap::new();

    for key in keys {
        groups
            .entry(get_query(key))
            .or_insert_with(Vec::new)
            .push(key);
    }

    groups
}
