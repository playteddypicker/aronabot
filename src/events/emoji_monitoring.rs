use serenity::{
    builder::{CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter, CreateMessage, EditMessage},
    client::Context,
    model::{
        channel::{Message, Reaction, ReactionType},
        id::{ChannelId, GuildId, MessageId},
    },
};

use lazy_static::lazy_static;
use std::collections::HashMap;
use tokio::sync::Mutex;

use log::info;

const CUTLINE: usize = 3;
const AWARDS_CHANNEL_ID: u64 = 1185257580264693861;
const AWARDS_GUILD_ID: u64 = 841337761431814165;
const DEFAULT_PROFILE_URL: &str = "https://media.discordapp.net/attachments/1035386153668452383/1185581299168198697/discordblue.png";

lazy_static! {
    static ref MESSAGE_LIST: Mutex<HashMap<u64, ReactionMonitor>> = Mutex::new(HashMap::new());
}
//global hashmap에 넣는 메시지 개추 모니터 구조체
struct ReactionMonitor {
    star_cnt: usize,
    registered_message_id: Option<MessageId>,
    original_msg_channel_id: ChannelId,
}

pub async fn addition_monitoring(ctx: &Context, reaction: Reaction) {
    info!("reaction added {:?}", reaction.emoji);
    //개추이모지 이외에는 다 무시
    if !reaction.emoji.unicode_eq("⭐") {
        return;
    }

    //개추수개수 모니터링
    if let Some(cnt) = count_stars(ctx, &reaction).await {
        info!("{}", cnt);
        if cnt >= CUTLINE {
            let mut msg_list = MESSAGE_LIST.lock().await;
            let entried = msg_list
                .entry(reaction.message_id.get())
                .and_modify(|rm| rm.star_cnt = cnt)
                .or_insert(ReactionMonitor {
                    star_cnt: cnt,
                    registered_message_id: None,
                    original_msg_channel_id: reaction.channel_id,
                });

            let mut attachment = AttachmentType::Empty;

            let (content, author) = match reaction.message(&ctx.http).await {
                Ok(c) => {
                    if !c.attachments.is_empty() {
                        attachment = AttachmentType::File(c.attachments[0].url.clone());
                    }
                    (
                        c.content.clone(),
                        CreateEmbedAuthor::new(match c.author_nick(&ctx.http).await {
                            Some(n) => n,
                            None => c.author.name.clone(),
                        })
                        .icon_url(match c.author.avatar_url() {
                            Some(url) => url,
                            None => DEFAULT_PROFILE_URL.to_string(),
                        }), //아유씨발귀찬하
                    )
                }
                Err(_) => (
                    "메시지 불러오지 못함".to_string(),
                    CreateEmbedAuthor::new("알 수 없는 유저"),
                ),
            };

            let msg_template = msg_template(
                cnt,
                content,
                reaction
                    .message_id
                    .link(reaction.channel_id, reaction.guild_id),
                author,
                reaction.channel_id.get(),
                attachment,
            );

            match entried.registered_message_id {
                Some(msg_id) => {
                    if let Ok(mut original_msg) = ctx
                        .http
                        .get_message(ChannelId::from(AWARDS_GUILD_ID), msg_id)
                        .await
                    {
                        let _ = original_msg
                            .edit(&ctx.http, edit_msg_template(cnt, reaction.channel_id.get()))
                            .await;
                    }
                } //메시지 수정
                None => {
                    if let Ok(sended_msg) = ChannelId::from(AWARDS_CHANNEL_ID)
                        .send_message(&ctx.http, msg_template)
                        .await
                    {
                        entried.registered_message_id = Some(sended_msg.id);
                    }
                } // 메시지 등록
            }
        }
    }
}

async fn count_stars(ctx: &Context, reaction: &Reaction) -> Option<usize> {
    match reaction
        .users(&ctx.http, reaction.emoji.clone(), None, reaction.user_id)
        .await
    {
        Ok(users) => Some(users.len() + 1),
        Err(_) => None,
    }
}

enum AttachmentType {
    File(String),
    Empty,
}

fn msg_template(
    stars: usize,
    content: String,
    link: String,
    author: CreateEmbedAuthor,
    channel_id: u64,
    attachments: AttachmentType,
) -> CreateMessage {
    let mut embed = CreateEmbed::new()
        .author(author)
        .title("바로가기")
        .description(content)
        .url(link);

    match attachments {
        AttachmentType::File(imgurl) => embed = embed.attachment(imgurl),
        AttachmentType::Empty => (),
    }

    let star = match stars {
        3..=8 => "⭐️",
        9..=13 => "🌟",
        _ => "💫",
    };

    CreateMessage::new()
        .add_embed(embed)
        .content(format!("{}{} <#{}>", star, stars, channel_id))
}

fn edit_msg_template(stars: usize, channel_id: u64) -> EditMessage {
    let star = match stars {
        5..=9 => "⭐️",
        11..=15 => "🌟",
        _ => "💫",
    };

    EditMessage::new().content(format!("{}{} <#{}>", star, stars, channel_id))
}
