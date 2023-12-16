use serenity::{
    all::{Member, UserId},
    builder::{
        CreateActionRow, CreateButton, CreateEmbed, CreateEmbedAuthor,
        CreateInteractionResponseMessage, CreateMessage, EditMessage, CreateInteractionResponse
    },
    client::Context,
    futures::StreamExt,
    model::{application::ButtonStyle, id::ChannelId},
};

use std::time::Duration;

use log::error;

pub async fn notice(ctx: &Context, new_member: Member) {
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
                "시간 내에 인증 버튼을 클릭하지 못했습니다. 문제가 있으면 TeddyPicker#2048로 DM주세요."
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
    id: UserId,
    profile_url: String,
    guild_name: String,
}

impl VerifyingMessageBuilder {
    fn new(new_member: &Member, gname: String) -> Self {
        Self {
            tag: new_member.user.tag(),
            id: new_member.user.id,
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
                CreateEmbedAuthor::new(format!("{}님이 들어오셨습니다.", &self.tag))
                    .icon_url(&self.profile_url),
            )
            .description(
                format!(
                    "<@{}>, {}에 오신 것을 환영합니다.",
                    &self.id, &self.guild_name
                ) + "\n"
                    + "이 서버는 도배 방지를 위해 인증 시스템을 도입하고 있습니다."
                    + "\n"
                    + "밑에 버튼을 눌러서 입장을 완료해주세요.",
            )
    }

    fn welcome_message(&self) -> CreateMessage {
        CreateMessage::new()
            .embed(self.welcome_embed(0x1F80F8))
            .components(vec![CreateActionRow::Buttons(vec![CreateButton::new(
                "verify",
            )
            .label("로봇이 아닙니다")
            .style(ButtonStyle::Primary)])])
    }

    fn warning_message(&self) -> EditMessage {
        EditMessage::new()
            .content(
                format!("<@{}>, 아직 인증이 되지 않았습니다.", &self.id)
                    + "\n"
                    + "5분 내에 버튼을 누르지 않을 시 자동으로 추방되므로 주의해주세요."
                    + "\n"
                    + "버튼이 눌리지 않는다면 <@653157614452211712> 에게 DM주세요.",
            )
            .embed(self.welcome_embed(0xF77F00))
    }

    fn kicked_message(&self) -> EditMessage {
        EditMessage::new()
            .content(format!("<@{}> 추방되었습니다.", &self.id))
            .embed(
                CreateEmbed::new()
                    .color(0xFFADAD)
                    .author(CreateEmbedAuthor::new("인증 실패됨").icon_url(&self.profile_url))
                    .description(format!("<@{}>님이 인증을 실패했습니다.", &self.id)),
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
                        CreateEmbedAuthor::new(format!("{}님이 들어오셨습니다.", &self.tag))
                        .icon_url(&self.profile_url))
                    .description(format!("<@{}>님의 인증이 완료되었습니다.", &self.id))
            )
            .components(vec![])
    }
}
