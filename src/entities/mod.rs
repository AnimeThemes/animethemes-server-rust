use std::{env, sync::OnceLock};

use hash_ids::HashIds;
use sea_orm::EntityTrait;

pub mod admin;
pub mod auth;
pub mod content;
pub mod document;
pub mod list;
pub mod user;

pub trait SoftDeleteEntity: EntityTrait {
    fn deleted_at_column() -> Self::Column;
}

pub trait HasHashId {
    fn hashids(&self) -> Vec<u64>;

    fn encode_hashid(&self) -> String {
        static HASHIDS: OnceLock<HashIds> = OnceLock::new();

        let hashids = HASHIDS.get_or_init(|| {
            HashIds::builder()
                .with_salt(
                    env::var("HASHIDS_SALT")
                        .expect("HASHIDS_SALT must be set in .env")
                        .as_str(),
                )
                .finish()
        });

        hashids.encode(&self.hashids())
    }
}
