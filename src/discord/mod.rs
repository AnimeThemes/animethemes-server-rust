use loco_rs::Result;
use sea_orm::{EntityTrait, IdenStatic, Iterable, ModelTrait};
use serde::Serialize;
use serde_json::Value;

use crate::middlewares::current_user::CurrentUser;

pub mod playlist;
pub mod user;

const IGNORED_COLUMNS: &[&str] = &["updated_at", "created_at", "deleted_at"];

pub fn created_embed(user: Option<CurrentUser>) -> serenity::all::CreateEmbed {
    let embed = serenity::all::CreateEmbed::new().color(3066993);

    let embed = if let Some(user) = user {
        embed.footer(auth_footer(user))
    } else {
        embed
    };

    embed
}

pub fn updated_embed<E>(
    user: Option<CurrentUser>,
    old_model: &E::Model,
    new_model: &E::Model,
) -> Result<serenity::all::CreateEmbed>
where
    E: EntityTrait,
    E::Model: Serialize,
{
    let mut embed = serenity::all::CreateEmbed::new().color(16776960);

    add_updated_fields::<E>(&mut embed, old_model, new_model)?;

    let embed = if let Some(user) = user {
        embed.footer(auth_footer(user))
    } else {
        embed
    };

    Ok(embed)
}

pub fn deleted_embed(user: Option<CurrentUser>) -> serenity::all::CreateEmbed {
    let embed = serenity::all::CreateEmbed::new().color(15158332);

    let embed = if let Some(user) = user {
        embed.footer(auth_footer(user))
    } else {
        embed
    };

    embed
}

fn add_updated_fields<E>(
    embed: &mut serenity::all::CreateEmbed,
    old_model: &E::Model,
    new_model: &E::Model,
) -> serde_json::Result<()>
where
    E: EntityTrait,
    E::Model: Serialize,
{
    let old_json = serde_json::to_value(old_model)?;
    let new_json = serde_json::to_value(new_model)?;

    let mut builder = std::mem::take(embed);

    for column in E::Column::iter() {
        if IGNORED_COLUMNS.contains(&column.as_str()) {
            continue;
        }

        if old_model.get(column) == new_model.get(column) {
            continue;
        }

        builder = builder
            .field("Attribute", column.as_str(), true)
            .field("Old", get_attribute_value(column.as_str(), &old_json), true)
            .field("New", get_attribute_value(column.as_str(), &new_json), true);
    }

    *embed = builder;

    Ok(())
}

fn auth_footer(user: CurrentUser) -> serenity::all::CreateEmbedFooter {
    serenity::all::CreateEmbedFooter::new(user.user.name.to_owned())
        .icon_url(user.user.avatar_url().to_owned())
}

fn get_attribute_value(column: &str, json: &Value) -> String {
    json.get(column)
        .map_or("-".to_string(), |value| match value {
            Value::String(v) => v.clone(),
            Value::Null => "*(null)*".to_string(),
            other => other.to_string(),
        })
}
