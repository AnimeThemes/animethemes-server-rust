use crate::middlewares::current_user::CurrentUser;

pub mod playlist;

pub fn created_embed(user: Option<CurrentUser>) -> serenity::all::CreateEmbed {
    let embed = serenity::all::CreateEmbed::new().color(3066993);

    let embed = if let Some(user) = user {
        embed.footer(auth_footer(user))
    } else {
        embed
    };

    embed
}

fn auth_footer(user: CurrentUser) -> serenity::all::CreateEmbedFooter {
    serenity::all::CreateEmbedFooter::new(user.user.name.to_owned())
        .icon_url(user.user.avatar_url().to_owned())
}
