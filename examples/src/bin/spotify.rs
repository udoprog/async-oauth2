use oauth2::{Client, StandardToken, State, Url};
use oauth2_examples::{config_from_args, listen_for_code};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = config_from_args("Spotify Example")?;

    let reqwest_client = reqwest::Client::new();

    let auth_url = Url::parse("https://accounts.spotify.com/authorize")?;
    let token_url = Url::parse("https://accounts.spotify.com/api/token")?;
    let redirect_url = Url::parse("http://localhost:8080/api/auth/redirect")?;

    let mut client = Client::new(config.client_id, auth_url, token_url);
    client.set_client_secret(config.client_secret);
    client.add_scope("user-read-email");
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
        .with_client(&reqwest_client)
        .execute::<StandardToken>()
        .await?;

    println!("Token: {:?}", token);
    Ok(())
}
