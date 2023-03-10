use serenity::{
    all::Member,
    builder::{
        CreateActionRow, CreateButton, CreateEmbed, CreateEmbedAuthor,
        CreateInteractionResponseMessage, CreateMessage, EditMessage, CreateInteractionResponse
    },
    client::Context,
    futures::StreamExt,
    model::{application::ButtonStyle, id::ChannelId},
};

use std::time::Duration;
use std::num::NonZeroU64;

use log::error;

pub async fn notice(ctx: &Context, mut new_member: Member) {
    if new_member.user.bot { return; }
    let system_channel = ChannelId::new(889812490538602506);

    let member_message = VerifyingMessageBuilder::new(
        &new_member,
        new_member
            .guild_id
            .name(&ctx.cache)
            .unwrap_or("TEDDYPICKER".to_string()),
    );

    let mut verify_message = system_channel
        .send_message(&ctx.http, member_message.welcome_message())
        .await
        .expect("sending message error: ");

    let mut interaction_stream = verify_message
        .await_component_interaction(ctx)
        .timeout(Duration::from_secs(60 * 5))
        .filter(move |f| {
            f.message.id == verify_message.id
                && f.member.as_ref().unwrap().user.id == new_member.user.id
        })
        .stream();
    
    if let Some(btn) = interaction_stream.next().await {
        if let Err(why) = btn.create_response(&ctx.http, CreateInteractionResponse::UpdateMessage(
            member_message.verified_message()
        )).await {
            error!("couldn't edit welcome message. why: {:?}", why);
        }

        if let Err(why) = new_member.add_role(&ctx.http, 920466119846944859).await {
            error!("couldn't add default role. why: {:?}", why);
        }
    } else {
        if let Err(why) = verify_message.edit(&ctx.http, member_message.warning_message()).await {
            error!("couldn't edit welcome message. why: {:?}", why);
        }

        let mut interaction_stream = verify_message
            .await_component_interaction(ctx)
            .timeout(Duration::from_secs(60 * 5))
            .filter(move |f| {
                f.message.id == verify_message.id
                    && f.member.as_ref().unwrap().user.id == new_member.user.id
            })
            .stream();

        if let Some(btn) = interaction_stream.next().await {
            if let Err(why) = btn.create_response(&ctx.http, CreateInteractionResponse::UpdateMessage(
                member_message.verified_message()
            )).await {
                error!("couldn't edit welcome message. why: {:?}", why);
            }

            if let Err(why) = new_member.add_role(&ctx.http, 920466119846944859).await {
                error!("couldn't add default role. why: {:?}", why);
            }
        } else {
        //user not pressed button in time.
            if let Err(why) = new_member.kick_with_reason(
                &ctx.http, 
                "?????? ?????? ?????? ????????? ???????????? ???????????????. ????????? ????????? TeddyPicker#2048??? DM?????????."
                ).await {
                error!("couldn't kick member. why: {:?}", why);
            }
                
            if let Err(why) = verify_message.edit(&ctx.http, member_message.kicked_message()).await {
                error!("couldn't edit welcome message. why: {:?}", why);
            }
        }
    }
}

#[derive(Clone)]
struct VerifyingMessageBuilder {
    tag: String,
    id: NonZeroU64,
    profile_url: String,
    guild_name: String,
}

impl VerifyingMessageBuilder {
    fn new(new_member: &Member, gname: String) -> Self {
        Self {
            tag: new_member.user.tag(),
            id: new_member.user.id.0,
            profile_url: new_member.avatar_url().unwrap_or(
                new_member.user.avatar_url().unwrap_or(
                    "https://i.pinimg.com/564x/7d/14/30/7d1430db3eb67d239f445af37991cc93.jpg"
                        .to_string(),
                ),
            ),
            guild_name: gname,
        }
    }

    fn welcome_embed(&self, color: u64) -> CreateEmbed {
        CreateEmbed::new()
            .color(color)
            .author(
                CreateEmbedAuthor::new(format!("{}?????? ?????????????????????.", &self.tag))
                    .icon_url(&self.profile_url),
            )
            .description(
                format!(
                    "<@{}>, {}??? ?????? ?????? ???????????????.",
                    &self.id, &self.guild_name
                ) + "\n"
                    + "??? ????????? ?????? ????????? ?????? ?????? ???????????? ???????????? ????????????."
                    + "\n"
                    + "?????? ????????? ????????? ????????? ??????????????????.",
            )
    }

    fn welcome_message(&self) -> CreateMessage {
        CreateMessage::new()
            .content(format!("<@{}>", &self.id))
            .embed(self.welcome_embed(0x1F80F8))
            .components(vec![CreateActionRow::Buttons(vec![CreateButton::new(
                "verify",
            )
            .label("????????? ????????????")
            .style(ButtonStyle::Primary)])])
    }

    fn warning_message(&self) -> EditMessage {
        EditMessage::new()
            .content(
                format!("<@{}>, ?????? ????????? ?????? ???????????????.", &self.id)
                    + "\n"
                    + "5??? ?????? ????????? ????????? ?????? ??? ???????????? ??????????????? ??????????????????."
                    + "\n"
                    + "????????? ????????? ???????????? <@653157614452211712> ?????? DM?????????.",
            )
            .embed(self.welcome_embed(0xF77F00))
    }

    fn kicked_message(&self) -> EditMessage {
        EditMessage::new()
            .content(format!("<@{}> ?????????????????????.", &self.id))
            .embed(
                CreateEmbed::new()
                    .color(0xFFADAD)
                    .author(CreateEmbedAuthor::new("?????? ?????????").icon_url(&self.profile_url))
                    .description(format!("<@{}>?????? ????????? ??????????????????.", &self.id)),
            )
            .components(vec![])
    }

    fn verified_message(&self) -> CreateInteractionResponseMessage {
        CreateInteractionResponseMessage::new()
            .content("")
            .embed(
                CreateEmbed::new()
                    .color(0x1F80F8)
                    .author(
                        CreateEmbedAuthor::new(format!("{}?????? ?????????????????????.", &self.tag))
                        .icon_url(&self.profile_url))
                    .description(format!("<@{}>?????? ????????? ?????????????????????.", &self.id))
            )
            .components(vec![])
    }
}
