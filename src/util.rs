use discord_sdk as ds;

use crate::APP_ID;

pub struct Client {
    pub discord: ds::Discord,
    pub user: ds::user::User,
    pub wheel: ds::wheel::Wheel,
}

pub async fn make_client(subs: ds::Subscriptions) -> Client {

    let (wheel, handler) = ds::wheel::Wheel::new(Box::new(|err| {
        log::error!("Encountered an error: {}", err);
    }));

    let mut user = wheel.user();

    let discord = ds::Discord::new(ds::DiscordApp::PlainId(APP_ID), subs, Box::new(handler))
        .expect("Unable to create discord client");

    log::info!("Waiting for handshake...");
    user.0.changed().await.unwrap();

    let user = match &*user.0.borrow() {
        ds::wheel::UserState::Connected(user) => user.clone(),
        ds::wheel::UserState::Disconnected(err) => panic!("failed to connect to Discord: {}", err),
    };

    log::info!("connected to Discord, local user is {:#?}", user);

    Client {
        discord,
        user,
        wheel,
    }
}