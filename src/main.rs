use std::pin::pin;
use grammers_client::{Client, Config, InitParams, Update};
use grammers_tl_types as tl;
use grammers_session::Session;
use tokio::{task};
use futures_util::future::{select, Either};
use dotenv;
use std::env;
use std::time::Duration;
use grammers_client::types::{Chat};

const SESSION_FILE: &str = "bot.session";
const CHANNEL: i32 = env::var("TG_CHANNEL").expect("Not Found Env").parse().expect("CHANNEL invalid");
const TOPIC: i32 = env::var("TG_TOPIC").expect("Not Found Env").parse().expect("TOPIC invalid");



async fn handle_update(client: Client, update: Update) -> Result<(), Box<dyn std::error::Error>> {
    match update {
        Update::NewMessage(message) if !message.outgoing() => {
            let chat = message.chat();
            match message.sender().unwrap() {
                Chat::Channel(target) => {
                    message.delete().await.unwrap();
                    match client
                        .set_banned_rights(&chat, &target)
                        .view_messages(false)
                        .send_messages(false)
                        .duration(Duration::from_secs(1))
                        .await {
                        Ok(_) => {
                            message.reply("本群組不允許頻道發言。").await?;
                        }
                        Err(_) => {
                            todo!("leave channel")
                            // client.invoke(
                            // &tl::functions::channels::LeaveChannel {
                            //     channel: chat::Channel,
                            // }).await?;
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
    let api_id = env::var("API_ID").expect("Not Found Env").parse().expect("TG_ID invalid");
    let api_hash = env::var("API_HASH").expect("Not Found Env").to_string();
    println!("API_ID: {}, API_HASH: {}", &api_id, &api_hash);
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
        .await.unwrap();
    println!("Connected!");

    if !client.is_authorized().await.unwrap() {
        println!("Signing in...");
        client.bot_sign_in(&token).await.unwrap();
        client.session().save_to_file(SESSION_FILE).unwrap();
        println!("Signed in!");
    }
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