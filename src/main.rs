use std::str::FromStr;
use dotenv::dotenv;
use std::env;
use serenity::{
    async_trait,
    framework::StandardFramework,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

struct Handler {
    user_ids: Vec<u64>,
}

impl Handler {
    fn new(user_ids: Vec<u64>) -> Self {
        Handler { user_ids }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        // println!("{}: {}", msg.author.name, msg.content);
        if msg.author.bot {
            return;
        }
        let bot_user_id = ctx.cache.current_user().id;

        // --- あいさつ ---
        // メッセージがボットのメンションを含むかどうかをチェックします
        if msg.mentions.iter().any(|user| user.id == bot_user_id) {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Hello from Rust!").await {
                println!("Error sending message: {:?}", why);
            }
        }

        // --- x.comをvxtwitter.comに置き換える ---
        // 特定のユーザーIDを確認
        if self.user_ids.contains(&msg.author.id.get()) {
            // 特定のユーザーIDに置き換えてください
            if msg.content.contains("x.com") {
                // ニックネームまたはユーザー名を取得
                let display_name = if let Some(guild_id) = msg.guild_id {
                    match guild_id.member(&ctx.http, msg.author.id).await {
                        Ok(member) => member.nick.unwrap_or_else(|| {
                            msg.author
                                .global_name
                                .clone()
                                .unwrap_or_else(|| msg.author.name.clone())
                        }),
                        Err(error) => {
                            println!("Error retrieving member info: {:?}", error);
                            msg.author
                                .global_name
                                .clone()
                                .unwrap_or_else(|| msg.author.name.clone())
                        }
                    }
                } else {
                    println!("Error retrieving guild info");
                    msg.author
                        .global_name
                        .clone()
                        .unwrap_or_else(|| msg.author.name.clone())
                };

                // 置換処理を含む新しいメッセージ内容を作成
                let replaced_content = msg
                    .content
                    .replace("twitter.com", "vxtwitter.com")
                    .replace("x.com", "vxtwitter.com");
                let new_content = format!("From: {}\n{}", display_name, replaced_content);

                // 新しいメッセージを送信
                if let Err(why) = msg.channel_id.say(&ctx.http, &new_content).await {
                    println!("Error sending message: {:?}", why);
                }

                // 元のメッセージを削除
                if let Err(why) = msg.delete(&ctx.http).await {
                    println!("Error deleting message: {:?}", why);
                }
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = env::var("BOT_TOKEN")
        .expect("Expected a token in the environment");

    let user_ids = env::var("USER_IDS")
        .expect("Expected USER_IDS in the environment")
        .split(',')
        .map(|id| u64::from_str(id.trim()).expect("Invalid ID"))
        .collect::<Vec<u64>>();

    let framework = StandardFramework::new();

    let handler = Handler::new(user_ids);

    // 必要なIntentを設定
    let intents = 
        GatewayIntents::GUILD_MESSAGES |
        GatewayIntents::MESSAGE_CONTENT |
        GatewayIntents::DIRECT_MESSAGES;

    let mut client = Client::builder(&token, intents)
        .event_handler(handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
