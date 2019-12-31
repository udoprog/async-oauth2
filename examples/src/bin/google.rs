use oauth2::{Client, StandardToken, State, Url};
use oauth2_examples::{config_from_args, listen_for_code};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = config_from_args("Google Example")?;

    let reqwest_client = reqwest::Client::new();

    let auth_url = Url::parse("https://accounts.google.com/o/oauth2/v2/auth")?;
    let token_url = Url::parse("https://www.googleapis.com/oauth2/v4/token")?;
    let redirect_url = Url::parse("http://localhost:8080/api/auth/redirect")?;

    let mut client = Client::new(config.client_id, auth_url, token_url);
    client.set_client_secret(config.client_secret);
    client.add_scope("https://www.googleapis.com/auth/youtube.readonly");
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
