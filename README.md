# async-oauth2

[![Documentation](https://docs.rs/async-oauth2/badge.svg)](https://docs.rs/async-oauth2)
[![Crates](https://img.shields.io/crates/v/async-oauth2.svg)](https://crates.io/crates/async-oauth2)
[![Actions Status](https://github.com/udoprog/async-oauth2/workflows/Rust/badge.svg)](https://github.com/udoprog/async-oauth2/actions)

An async/await implementation of OAuth2 for Rust.

Documentation is available on [docs.rs](https://docs.rs/crate/async-oauth2), or you can check out the [working examples](https://github.com/udoprog/async-oauth2/tree/master/examples).

## Examples

Note: `oauth2_examples` below is a crate written to run the examples, you should
integrate it into whatever HTTP framework you are using.

```rust
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
```

If you want to run some of our [pre-baked examples](https://github.com/udoprog/async-oauth2/tree/master/examples), you need to register an application that has a redirect URL of `http://localhost:8080/api/auth/redirect`, then you can execute a flow like this:

```
cargo run --manifest-path=examples/Cargo.toml --bin spotify --client-id <client-id> --client-secret <client-secret>
cargo run --manifest-path=examples/Cargo.toml --bin google --client-id <client-id> --client-secret <client-secret>
cargo run --manifest-path=examples/Cargo.toml --bin twitch --client-id <client-id> --client-secret <client-secret>
```

## Relationship to oauth2-rs

This is a fork of [`oauth2-rs`](https://github.com/ramosbugs/oauth2-rs).

The main differences are:
* Removal of unnecessary type parameters on Client ([see discussion here]).
* Only support one client implementation (reqwest).
* Remove most newtypes except `Scope` and the secret ones since they made the API harder to use.

[see discussion here]: https://github.com/ramosbugs/oauth2-rs/issues/44#issuecomment-50158653
