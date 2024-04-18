use crate::entities::{self, channel, chat, inout, user};
use chrono::{Datelike, Utc};
use poise::serenity_prelude::{self as serenity};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel,
    ModelTrait, Order, QueryFilter, QueryOrder,
};

pub async fn get_user_model(
    member: &serenity::Member,
    cn: &DatabaseConnection,
) -> Option<entities::user::Model> {
    match entities::prelude::User::find()
        .filter(user::Column::DiscordId.contains(member.user.id.to_string()))
        .one(cn)
        .await
    {
        Ok(ou) => match ou {
            Some(u) => Some(u),
            None => {
                let nuser = entities::user::ActiveModel {
                    id: ActiveValue::NotSet,
                    discord_id: ActiveValue::Set(member.user.id.to_string()),
                    name: ActiveValue::Set(member.user.name.to_string()),
                    nick_name: ActiveValue::Set(match &member.nick {
                        Some(nk) => Some(nk.to_string()),
                        None => None,
                    }),
                    created: ActiveValue::Set(Utc::now().naive_utc()),
                };
                match nuser.insert(cn).await {
                    Ok(u) => Some(u),
                    Err(_) => None,
                }
            }
        },
        Err(_) => None,
    }
}

pub async fn get_channel_model(
    channel: &serenity::GuildChannel,
    cn: &DatabaseConnection,
) -> Option<entities::channel::Model> {
    match entities::prelude::Channel::find()
        .filter(channel::Column::DiscordId.contains(channel.id.to_string()))
        .one(cn)
        .await
    {
        Ok(ou) => match ou {
            Some(u) => Some(u),
            None => {
                let nchannel = entities::channel::ActiveModel {
                    id: ActiveValue::NotSet,
                    discord_id: ActiveValue::Set(channel.id.to_string()),
                    name: ActiveValue::Set(channel.to_string()),
                    created: ActiveValue::Set(Utc::now().naive_utc()),
                    r#type: ActiveValue::Set(channel.kind.name().to_string()),
                };
                match nchannel.insert(cn).await {
                    Ok(u) => Some(u),
                    Err(_) => None,
                }
            }
        },
        Err(_) => None,
    }
}

pub async fn get_chat_model_thismonth(
    member: &serenity::Member,
    channel: &serenity::GuildChannel,
    cn: &DatabaseConnection,
) -> Option<entities::chat::Model> {
    let now = Utc::now();
    let userm = get_user_model(member, cn).await.unwrap();
    let channelm = get_channel_model(channel, cn).await.unwrap();
    match entities::prelude::Chat::find()
        .filter(chat::Column::User.eq(userm.id))
        .filter(chat::Column::Channel.eq(channelm.id))
        .filter(chat::Column::Created.between(now.with_day(1).unwrap(), now))
        .one(cn)
        .await
    {
        Ok(oc) => match oc {
            Some(c) => Some(c),
            None => {
                let nchat = entities::chat::ActiveModel {
                    id: sea_orm::NotSet,
                    user: ActiveValue::Set(userm.id),
                    channel: ActiveValue::Set(channelm.id),
                    count: ActiveValue::Set(0),
                    mentioned: ActiveValue::Set(0),
                    created: sea_orm::NotSet,
                };
                match nchat.insert(cn).await {
                    Ok(c) => Some(c),
                    Err(_) => None,
                }
            }
        },
        Err(_) => None,
    }
}

pub async fn get_inout_model_typein(
    member: &serenity::Member,
    channel: &serenity::GuildChannel,
    cn: &DatabaseConnection,
) -> Option<entities::inout::Model> {
    let userm = get_user_model(member, cn).await.unwrap();
    let channelm = get_channel_model(channel, cn).await.unwrap();
    match entities::prelude::Inout::find()
        .filter(inout::Column::User.eq(userm.id))
        .filter(inout::Column::Out.is_null())
        .order_by(inout::Column::Id, Order::Desc)
        .one(cn)
        .await
    {
        Ok(oinn) => {
            match oinn {
                Some(inn) => {
                    inn.delete(cn).await.unwrap();
                }
                None => {}
            };
            let ninout = entities::inout::ActiveModel {
                id: ActiveValue::NotSet,
                user: ActiveValue::Set(userm.id),
                channel: ActiveValue::Set(channelm.id),
                r#in: ActiveValue::NotSet,
                out: ActiveValue::NotSet,
            };
            match ninout.insert(cn).await {
                Ok(c) => Some(c),
                Err(_) => None,
            }
        }
        Err(_) => None,
    }
}

pub async fn get_inout_model_typeout(
    member: &serenity::Member,
    cn: &DatabaseConnection,
) -> Option<entities::inout::Model> {
    let userm = get_user_model(member, cn).await.unwrap();
    match entities::prelude::Inout::find()
        .filter(inout::Column::User.eq(userm.id))
        .filter(inout::Column::Out.is_null())
        .order_by(inout::Column::Id, Order::Desc)
        .one(cn)
        .await
    {
        Ok(oinn) => match oinn {
            Some(inn) => {
                let now = Utc::now();
                let mut act_inn = inn.clone().into_active_model();
                act_inn.out = ActiveValue::Set(Some(now.naive_utc()));
                match act_inn.update(cn).await {
                    _ => {}
                };
                Some(inn)
            }
            None => None,
        },
        Err(_) => None,
    }
}
