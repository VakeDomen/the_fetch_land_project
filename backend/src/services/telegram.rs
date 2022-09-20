use std::{error::Error, env};

use teloxide::{Bot, requests::{RequesterExt, Requester}};

pub async fn notify_via_telegram(message: String, sender: String) -> Result<(), Box<dyn Error + Send + Sync>> {
    let chat_id = env::var("TELEGRAM_NOTIFICATION_CHAT_ID").expect("Missing the TELEGRAM_NOTIFICATION_CHAT_ID environment variable.");
    tokio::task::spawn(async move {
        match Bot::from_env().auto_send().send_message(
            chat_id,
            format!("{}:\n\t{}", message, sender)
        ).await {
            Ok(e) => println!("{:?}", e),
            Err(e) => println!("{:?}", e),
        };
    });
    Ok(())
}