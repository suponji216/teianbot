use crate::{commands::body::Data, db::get_chat_model_thismonth};
use poise::serenity_prelude::{self as serenity, Member, Message, PartialMember};
use sea_orm::{ActiveModelTrait, IntoActiveModel};

pub async fn update_chatcount(ctx: &serenity::Context, data: &Data, new_message: &Message) {
    if !new_message.author.bot {
        let gc: serenity::GuildChannel = ctx
            .cache
            .guild(new_message.guild_id.unwrap())
            .unwrap()
            .channels
            .get(&new_message.channel_id)
            .unwrap()
            .clone();
        let m: Member = <Option<Box<PartialMember>> as Clone>::clone(&new_message.member)
            .unwrap()
            .as_ref()
            .clone()
            .into();
        match get_chat_model_thismonth(&m, &gc, &data.connection).await {
            Some(chat) => {
                let mut mchat = chat.clone().into_active_model();
                mchat.count = sea_orm::Set(chat.count + 1);
                match mchat.update(&data.connection).await {
                    _ => {}
                };
            }
            None => {}
        }
        for user in new_message.mentions.clone() {
            let mm: Member = <Option<Box<PartialMember>> as Clone>::clone(&user.member)
                .unwrap()
                .as_ref()
                .clone()
                .into();
            match get_chat_model_thismonth(&mm, &gc, &data.connection).await {
                Some(chat) => {
                    let mut mchat = chat.clone().into_active_model();
                    mchat.mentioned = sea_orm::Set(chat.mentioned + 1);
                    match mchat.update(&data.connection).await {
                        _ => {}
                    };
                }
                None => {}
            }
        }
    };
}
