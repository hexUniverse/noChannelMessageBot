use std::env;
use std::pin::pin;
use std::time::Duration;

use dotenv;
use futures_util::future::{select, Either};
use grammers_client::{Client, Config, InitParams, Update};
use grammers_session::Session;
use grammers_tl_types as tl;
use grammers_tl_types::enums;
use grammers_tl_types::enums::ChatFull;
use grammers_tl_types::types;

use tokio::task;

const SESSION_FILE: &str = "bot.session";

async fn handle_update(client: Client, update: Update) -> Result<(), Box<dyn std::error::Error>> {
    match update {
        Update::NewMessage(message) if !message.outgoing() => {
            let chat = message.chat();
            if message.sender().is_some() != false { return Ok(()) };
            match message.sender().unwrap() {
                grammers_client::types::Chat::Channel(target) => {
                    let fullchat: types::messages::ChatFull = client
                        .invoke(&tl::functions::channels::GetFullChannel {
                            channel: enums::InputChannel::Channel(types::InputChannel {
                                channel_id: chat.id(),
                                access_hash: chat.pack().access_hash.unwrap(),
                            }),
                        })
                        .await
                        .unwrap()
                        .into();
                    match fullchat.full_chat {
                        ChatFull::Full(_) => {}
                        ChatFull::ChannelFull(channelfull) => {
                            if channelfull.linked_chat_id.unwrap() == target.id() {
                                return Ok(());
                            }
                        }
                    }
                    let _e = message.delete().await.map_err(|e| "");
                    match client
                        .set_banned_rights(&chat, &target)
                        .view_messages(false)
                        .send_messages(false)
                        .duration(Duration::from_secs(1))
                        .await
                    {
                        Ok(_) => {
                            message.reply("本群組不允許頻道發言。").await?;
                        }
                        Err(e) => {
                            client
                                .invoke(&tl::functions::channels::LeaveChannel {
                                    channel: enums::InputChannel::Channel(types::InputChannel {
                                        channel_id: chat.id(),
                                        access_hash: chat.pack().access_hash.unwrap(),
                                    }),
                                })
                                .await?;
                        }
                    }
                }
                _ => {}
            }
        }
        _ => {}
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::from_filename(".env").expect("Failed to load .env file");
    let api_id = env::var("API_ID")
        .expect("Not Found Env")
        .parse()
        .expect("TG_ID invalid");
    let api_hash = env::var("API_HASH").expect("Not Found Env").to_string();
    let token = env::var("BOT_TOKEN").expect("Not Found Env");
    println!("Connecting to Telegram...");
    let client = Client::connect(Config {
        session: Session::load_file_or_create(SESSION_FILE).unwrap(),
        api_id,
        api_hash: api_hash.clone(),
        params: InitParams {
            // Fetch the updates we missed while we were offline
            catch_up: true,
            ..Default::default()
        },
    })
    .await
    .unwrap();
    println!("Connected!");

    if !client.is_authorized().await.unwrap() {
        println!("Signing in...");
        client.bot_sign_in(&token).await.unwrap();
        client.session().save_to_file(SESSION_FILE).unwrap();
        println!("Signed in!");
    }
    let me = client.get_me().await.unwrap();
    println!("{:?}", me.full_name());
    println!("Waiting for messages...");

    loop {
        let update = {
            let exit = pin!(async { tokio::signal::ctrl_c().await });
            let upd = pin!(async { client.next_update().await });

            match select(exit, upd).await {
                Either::Left(_) => None,
                Either::Right((u, _)) => Some(u),
            }
        };

        let update = match update {
            None | Some(Ok(None)) => break,
            Some(u) => u.unwrap().unwrap(),
        };

        let handle = client.clone();
        task::spawn(async move {
            match handle_update(handle, update).await {
                Ok(_) => {}
                Err(e) => eprintln!("Error handling updates!: {}", e),
            }
        });
    }

    println!("Saving session file and exiting...");
    client.session().save_to_file(SESSION_FILE).unwrap();
    Ok(())
}
