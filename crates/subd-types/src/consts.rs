use once_cell::sync::OnceCell;
use twitch_api2::twitch_oauth2::{AccessToken, RefreshToken};

/// twitch_bot_oauth is the authentication for the bot that will respond to messages in chat and
/// whispers (TODO). It can possibly be your account, but in general that will be pretty confusing
/// so you should probably use a new and separate bot account
pub fn get_twitch_bot_oauth() -> String {
    static TWITCH_BOT_OAUTH: OnceCell<String> = OnceCell::new();
    TWITCH_BOT_OAUTH
        .get_or_init(|| {
            dotenv::var("SUBD_TWITCH_BOT_OAUTH")
                .expect("SUBD_TWITCH_BOT_OAUTH must be present in .env file")
                .replace("oauth:", "")
                .to_string()
        })
        .clone()
}

/// Prefer get_twitch_broadcaster_oauth() over this functions. Since you can just leak
/// strings so easily (and we don't want that). Unfortunately, pubsub seems to think
/// that we should pass strings directly? As far as I can tell.
pub fn get_twitch_broadcaster_raw() -> String {
    static TWITCH_BROADCASTER_OAUTH: OnceCell<String> = OnceCell::new();
    TWITCH_BROADCASTER_OAUTH
        .get_or_init(|| {
            dotenv::var("SUBD_TWITCH_BROADCASTER_OAUTH")
                .expect("$SUBD_TWITCH_BROADCASTER_OAUTH must be set")
                .replace("oauth:", "")
                .to_string()
        })
        .clone()
}

pub fn get_twitch_broadcaster_oauth() -> AccessToken {
    static TWITCH_BROADCASTER_OAUTH: OnceCell<AccessToken> = OnceCell::new();
    TWITCH_BROADCASTER_OAUTH
        .get_or_init(|| {
            AccessToken::new(
                dotenv::var("SUBD_TWITCH_BROADCASTER_OAUTH")
                    .expect("$SUBD_TWITCH_BROADCASTER_OAUTH must be set")
                    .replace("oauth:", "")
                    .to_string(),
            )
        })
        .clone()
}

pub fn get_twitch_broadcaster_refresh() -> Option<RefreshToken> {
    static TWITCH_BROADCASTER_REFRESH: OnceCell<Option<RefreshToken>> = OnceCell::new();
    TWITCH_BROADCASTER_REFRESH
        .get_or_init(|| match dotenv::var("SUBD_TWITCH_BOT_REFRESH").ok() {
            Some(token) => Some(RefreshToken::new(token)),
            None => None,
        })
        .clone()
}

pub fn get_github_broadcaster_token() -> String {
    // TODO: Lazy
    // TODO: Should return an option probably and just quit from github functions if you don't have
    // it. As many tokens as possible should not be required (will make things a lot easier later
    // if we have this strategy from the start).
    String::from("token ")
        + &std::env::var("SUBD_GITHUB_TOKEN").expect("Should have GITHUB_ACCESS token")
}

pub fn get_database_url() -> String {
    dotenv::var("DATABASE_URL").expect("DATABASE_URL to exist")
}
