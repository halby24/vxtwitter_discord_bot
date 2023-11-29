use dotenv::dotenv;
use std::env;
use serenity::{
    async_trait,
    framework::StandardFramework,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use regex::Regex;

fn replace_and_extract_links(text: &str) -> String {
    let re = Regex::new(r"https?://(?:www\.)?(x\.com|twitter\.com)/\S*").unwrap();
    let links: Vec<String> = re.find_iter(text)
        .map(|mat| mat.as_str()
            .replace("twitter.com", "vxtwitter.com")
            .replace("x.com", "vxtwitter.com"))
        .collect();
    
    // 置き換えられたリンクを改行で結合
    links.join("\n")
}

struct Handler;

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
        // メッセージがx.com, twitter.comを含むかどうかをチェック
        if msg.content.contains("https://x.com") || msg.content.contains("https://twitter.com") {
            // 置換処理を含む新しいメッセージ内容を作成
            let links = replace_and_extract_links(&msg.content);

            // 新しいメッセージを送信
            if let Err(why) = msg.channel_id.say(&ctx.http, &links).await {
                println!("Error sending message: {:?}", why);
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

    let framework = StandardFramework::new();

    // 必要なIntentを設定
    let intents = 
        GatewayIntents::GUILD_MESSAGES |
        GatewayIntents::MESSAGE_CONTENT |
        GatewayIntents::DIRECT_MESSAGES;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
