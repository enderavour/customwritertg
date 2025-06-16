use futures_util::future::{select, Either};
use grammers_client::{Client, Config, InitParams, Update};
use grammers_session::Session;
use log;
use simple_logger::SimpleLogger;
use core::time;
use std::io;
use std::pin::pin;
use tokio::{runtime, task};
use std::thread;
use std::collections::HashMap;
use std::sync::LazyLock;
mod parse_settings;


type Res = Result<(), Box<dyn std::error::Error>>;

static TYP: char = '#'; // Insert typing symbol here

const SESSION: &str = "sess.session";
static SETTINGS: LazyLock<HashMap<String, String>> = LazyLock::new(|| {
    parse_settings::parse_settings().unwrap()
});

async fn handle_update(client: Client, update: Update) -> Res
{
    match update 
    {
        Update::NewMessage(message) if message.outgoing() => {
            if message.text().starts_with("!t") {
                let edited = message.text()[2..].to_string(); 
                let mut result = String::new();
            
                for c in edited.chars() {
                    result.push(c); 
                    client.edit_message(
                        message.chat(),
                        message.id(),
                        result.clone() + &TYP.to_string(), 
                    ).await?;
                    thread::sleep(time::Duration::from_secs(0.9 as u64));
                }
                client.edit_message(
                    message.chat(),
                    message.id(),
                    result,
                ).await?;
            }  
        }
        _ => {}
    }
    Ok(())
}

async fn async_main() -> Res
{
    SimpleLogger::new()
                .with_level(log::LevelFilter::Debug)
                .init()
                .unwrap();
    
    println!("Connecting...");

    let client = Client::connect(
        Config {
            session: Session::load_file_or_create(SESSION)?,
            api_id: (&*SETTINGS).get("API_ID").unwrap().parse::<i32>()?,
            api_hash: (&*SETTINGS).get("API_HASH").unwrap().to_string(),
            params: InitParams {
                catch_up: true,
                ..Default::default()
            }
        }
    ).await?;

    let mut st = String::new();

    if !client.is_authorized().await?
    {
        println!("Signing...");
        let tok = client.request_login_code((&*SETTINGS).get("PHONENUMBER").unwrap()).await?;
        println!("Enter code: ");
        io::stdin().read_line(&mut st).unwrap();
        let _ = match client.sign_in(&tok, st.as_str()).await
        {
            Ok(u) => u,
            Err(e) => {
                println!("Failed to sign: {}", e);
                return Err(e.into());
            }
        };
        println!("Signed in!");
    }

    println!("Waiting for messages...");

    loop {
        let update = {
            let exit = pin!(async { tokio::signal::ctrl_c().await });
            let upd = pin!(async { client.next_update().await });

            match select(exit, upd).await
            {
                Either::Left(_) => None,
                Either::Right((u, _)) => Some(u)
            }
        };

        let update = match update 
        {
           None | Some(Ok(None)) => break,
           Some(u) => u?.unwrap()     
        };

        let hnd = client.clone();
        task::spawn(async move {
            match handle_update(hnd, update).await
            {
                Ok(_) => {},
                Err(e) => println!("Error: {}", e)
            }
        });
    }

    println!("Saving session and exiting user...");
    client.session().save_to_file(SESSION)?;
    client.sign_out().await?;
    Ok(())
}

fn main() -> Res 
{
    runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .unwrap()
                    .block_on(async_main())?;
    Ok(())
}
