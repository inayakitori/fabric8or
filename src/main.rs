use std::any::Any;
use std::env;
use std::fmt::format;
use std::ops::Deref;
use std::sync::Arc;

use serenity::async_trait;
use serenity::builder::CreateThread;
use serenity::model::channel::{Channel, ChannelType, GuildChannel, Message};
use serenity::model::channel::Channel::Guild;
use serenity::model::gateway::Ready;
use serenity::model::id::{ChannelId, MessageId};
use serenity::prelude::*;
use serenity::utils::MessageBuilder;

struct Handler{
    channels: Vec<ChannelInfo>,
}


#[derive(Clone, Default, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
struct Channels {
    channels: Vec<ChannelInfo>
}

impl Channels{

    fn contains(&self, channel: ChannelId) -> bool{
        self.channels.clone().iter().any(|channel_info| channel_info.channel == channel)
    }

    async fn add_channel(&mut self, channel: ChannelId, message: Message, ctx: &Context) -> Result<(), ()> {

        if let Guild(channel) = channel {
            if channel.is_text_based(){
                if let OK()

            }
        }

        Err(())
    }

    async fn add_message(&mut self, channel: ChannelId, message: Message, ctx: &Context) -> Result<(),()>{
        if let Ok(notice_message) = channel.say(ctx, message.content).await {
            self.channels.push(
                ChannelInfo{
                    channel: channel.id,
                    notice_message: notice_message.id
                }
            );
            Ok(())
        }
        Err(())
    }

}

#[derive(Copy, Clone, Default, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
struct ChannelInfo{
    channel: ChannelId,
    notice_message: MessageId,
}


// 326417787968

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {

        //notic message

        //thread management
        if !msg.author.bot &&
            msg.channel(&ctx.http)
            .await.expect("couldn't get channel")
            .guild().expect("couldn't get guild channel")
            .kind == ChannelType::Text{
            if msg.attachments.is_empty() {
                //do not allow messages without files
                if let Err(why) = msg.delete(&ctx.http).await {
                    println!("Error deleting message: {:?}", why);
                }
            } else {
                //attach a thread to messages with files
                msg.channel_id.create_public_thread(&ctx.http, msg.id, |create_thread| -> &mut CreateThread {
                    create_thread
                        .name(format!("{} {}", msg.author.name, &msg.id.0))
                        .kind(ChannelType::PublicThread)
                        .auto_archive_duration(60)
                }).await.unwrap();
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client =
        Client::builder(&token, intents).event_handler(Handler).await.expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}