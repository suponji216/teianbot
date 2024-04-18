use crate::{commands::body::Data, db::*};
use poise::serenity_prelude::{self as serenity, VoiceState};

pub async fn update_inout(
    ctx: &serenity::Context,
    data: &Data,
    old: &Option<VoiceState>,
    new: &VoiceState,
) {
    match &new.member {
        Some(m) => {
            if !m.user.bot {
                if old.is_none() && new.channel_id.is_some() {
                    let gc: serenity::GuildChannel = ctx
                        .cache
                        .guild(new.guild_id.unwrap())
                        .unwrap()
                        .channels
                        .get(&new.channel_id.unwrap())
                        .unwrap()
                        .clone();
                    match get_inout_model_typein(&m, &gc, &data.connection).await {
                        _ => {}
                    };
                }
                if old.is_some() && new.channel_id.is_none() {
                    match get_inout_model_typeout(&m, &data.connection).await {
                        _ => {}
                    };
                }
                if old.is_some() && new.channel_id.is_some() {
                    if old.clone().unwrap().channel_id != new.channel_id {
                        match get_inout_model_typeout(&m, &data.connection).await {
                            _ => {}
                        };
                        let gc: serenity::GuildChannel = ctx
                            .cache
                            .guild(new.guild_id.unwrap())
                            .unwrap()
                            .channels
                            .get(&new.channel_id.unwrap())
                            .unwrap()
                            .clone();
                        match get_inout_model_typein(&m, &gc, &data.connection).await {
                            _ => {}
                        };
                    }
                }
            };
        }
        None => panic!(),
    }
}
