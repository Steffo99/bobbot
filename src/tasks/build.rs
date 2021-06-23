//! This module contains a task to build a new channel.

use serenity::model::prelude::*;
use serenity::prelude::*;
use crate::errors::*;
use crate::database::models::{MayHaveBeenCreatedByBob};
use crate::utils::permission_overwrites::get_built_channel_permows;


/// Build a new channel in the specified [`guild`]([Guild]) with the specified `name`.
///
/// The function optionally accepts a [`category`]([ChannelCategory]) and a `preset_name`:
/// - if a `category` is specified, the channel is created in it and inherits its [PermissionOverwrite]s.
/// - if a `preset_name` is specified, the preset with that name is loaded and used as a template for the channel,
///   inheriting the following properties:
///     - [PermissionOverwrite]s
///     - Bitrate (defaulting to 64 kbps)
///     - User limit (defaulting to None)
///
/// # Returns
///
/// - `Ok(msg)` if the channel creation was successful.
/// - `Err(_)` if something went wrong in the creation of the channel.
///
/// # To do
///
/// Presets aren't loaded yet.
///
pub async fn task_build(ctx: &Context, guild: &PartialGuild, name: &str, creator: &Member, category: &Option<ChannelCategory>, preset: &Option<&str>) -> BobResult<GuildChannel> {
    debug!(
        "Running task: build | In <G:{}>, build #{} in <C:{}> with preset {}",
        &guild.name,
        &name,
        &category.as_ref().map_or_else(|| "<no category>", |ok| ok.name()),
        &preset.clone().map_or_else(|| format!("<no preset>"), |ok| format!("'{}'", ok))
    );

    let permissions: Vec<PermissionOverwrite> = get_built_channel_permows(&ctx.cache, &category, creator.user.id.clone()).await;
    let bitrate: Option<u32> = None;
    let limit: Option<u32> = None;

    let created = guild.create_channel(&ctx.http, |c| {
        c.name(name.clone());
        c.kind(ChannelType::Voice);
        if let Some(cat) = category {
            c.category(cat.id.clone());
        }

        c.permissions(permissions);
        c.bitrate(bitrate.unwrap_or(64000));
        if let Some(limit) = limit {
            c.user_limit(limit);
        }

        c
    }).await.bob_catch(ErrorKind::Admin, "Failed to create channel")?;

    created.mark_as_created_by_bob()?;

    Ok(created)
}