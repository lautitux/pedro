use std::{error::Error, time::Duration};

use discord_sdk as ds;
use ds::activity::ActivityBuilder;
use rand::seq::SliceRandom;
mod util;

pub const APP_ID: ds::AppId = 1234314382721286265;

const STATES: &'static [&'static str] = &["Revolviendo 🗑️", "Bailando 💃", "Racooning 🦝"];
const PEDRO_MAX_INCLUSIVE_INDEX: usize = 10;

pub fn make_activity(state: &str, index: usize) -> ActivityBuilder {
    ds::activity::ActivityBuilder::default()
        .details("🦝🔄🦝🔄🦝".to_string())
        .state(state.to_string())
        .assets(
            ds::activity::Assets::default()
                .large(format!("pedro{}", index), Some("Pedro".to_owned())),
        )
        .button(ds::activity::Button {
            label: "Baila Pedro!".to_owned(),
            url: "https://www.youtube.com/watch?v=F2YpXC1itEE".to_owned(),
        })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let client = util::make_client(ds::Subscriptions::ACTIVITY).await;

    let mut activity_events = client.wheel.activity();

    tokio::task::spawn(async move {
        while let Ok(activity_event) = activity_events.0.recv().await {
            log::info!("Received activity event:\n{:?}", activity_event);
        }
    });

    let mut pedro_index = 0;
    let mut rng = rand::thread_rng();

    let (tx, mut rx) = tokio::sync::mpsc::channel::<bool>(3);

    tokio::spawn(async move {
        // Close on Enter
        let mut r = String::new();
        let _ = std::io::stdin().read_line(&mut r);
        log::info!("Closing may take up to 8 secs....");
        tx.send(true).await.unwrap();
    });

    loop {
        if let Ok(_) = rx.try_recv() {
            break;
        }

        log::info!(
            "Updated activity: {:?}",
            client
                .discord
                .update_activity(make_activity(
                    STATES.choose(&mut rng).expect("Not an empty slice"),
                    pedro_index
                ))
                .await
        );

        if pedro_index < PEDRO_MAX_INCLUSIVE_INDEX {
            pedro_index += 1;
        } else {
            pedro_index = 0;
        }
        tokio::time::sleep(Duration::from_secs(8)).await;
    }

    log::info!(
        "Cleared activity: {:?}",
        client.discord.clear_activity().await
    );

    client.discord.disconnect().await;

    Ok(())
}
