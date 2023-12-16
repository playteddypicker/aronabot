use serenity::{
    all::Member,
    async_trait,
    client::{Context, EventHandler},
    gateway::ActivityData,
    model::{
        application::Interaction,
        channel::{Message, Reaction},
        gateway::Ready,
        guild::Guild,
    },
};

use log::info;

use crate::events::{emoji_monitoring, member_verification};

pub struct DiscordEventHandler;

#[async_trait]
impl EventHandler for DiscordEventHandler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{}으로 로그인 완료!", ready.user.tag());

        ctx.set_activity(Some(ActivityData::playing("ㅁㄴㅇㄹ")));
    }

    //서버 입장 유저 인증기능
    async fn guild_member_addition(&self, ctx: Context, new_member: Member) {
        tokio::join!(member_verification::notice(&ctx, new_member));
    }

    //음성채널 모니터링 기능
    async fn reaction_add(&self, ctx: Context, add_reaction: Reaction) {
        tokio::join!(emoji_monitoring::addition_monitoring(&ctx, add_reaction));
    }
}
