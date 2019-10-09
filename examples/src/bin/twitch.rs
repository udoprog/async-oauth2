//! Showcases how to define and use a nonstandard token type.
//!
//! Note: Twitch requires you to set `client_id` and `client_secret` as extra
//! parameters when performing the token exchange (see below).

use oauth2::{AccessToken, Client, RefreshToken, Scope, State, Token, TokenType, Url};
use oauth2_examples::{config_from_args, listen_for_code};
use std::{error::Error, time::Duration};

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct TwitchToken {
    access_token: AccessToken,
    token_type: TokenType,
    #[serde(skip_serializing_if = "Option::is_none")]
    expires_in: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    refresh_token: Option<RefreshToken>,
    #[serde(rename = "scope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    scopes: Option<Vec<Scope>>,
}

impl Token for TwitchToken {
    fn access_token(&self) -> &AccessToken {
        &self.access_token
    }

    fn token_type(&self) -> &TokenType {
        &self.token_type
    }

    fn expires_in(&self) -> Option<Duration> {
        self.expires_in.map(Duration::from_secs)
    }

    fn refresh_token(&self) -> Option<&RefreshToken> {
        self.refresh_token.as_ref()
    }

    fn scopes(&self) -> Option<&Vec<Scope>> {
        self.scopes.as_ref()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = config_from_args("Twitch Example")?;

    let reqwest_client = reqwest::Client::new();

    let auth_url = Url::parse("https://id.twitch.tv/oauth2/authorize")?;
    let token_url = Url::parse("https://id.twitch.tv/oauth2/token")?;
    let redirect_url = Url::parse("http://localhost:8080/api/auth/redirect")?;

    let mut client = Client::new(&config.client_id, auth_url, token_url);
    client.set_client_secret(&config.client_secret);
    client.set_redirect_url(redirect_url);

    let state = State::new_random();
    let auth_url = client.authorize_url(&state);

    println!("Browse to: {}", auth_url);

    let received = listen_for_code(8080).await?;

    if received.state != state {
        panic!("CSRF token mismatch :(");
    }

    let token = client
        .exchange_code(received.code)
        .param("client_id", &config.client_id)
        .param("client_secret", &config.client_secret)
        .with_client(&reqwest_client)
        .execute::<TwitchToken>()
        .await?;

    println!("Token: {:?}", token);
    Ok(())
}
