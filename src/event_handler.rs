use serenity::{
    all::Member,
    async_trait,
    client::{Context, EventHandler},
    gateway::ActivityData,
    model::{application::Interaction, channel::Message, gateway::Ready, guild::Guild},
};

use log::info;

use crate::events::member_verification;

pub struct DiscordEventHandler;

#[async_trait]
impl EventHandler for DiscordEventHandler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{}으로 로그인 완료!", ready.user.tag());

        ctx.set_activity(Some(ActivityData::playing("ㅁㄴㅇㄹ")));
    }

    async fn guild_member_addition(&self, ctx: Context, new_member: Member) {
        tokio::join!(member_verification::notice(&ctx, new_member));
    }
}
