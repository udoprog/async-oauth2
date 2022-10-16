# async-oauth2

[<img alt="github" src="https://img.shields.io/badge/github-udoprog/async--oauth2-8da0cb?style=for-the-badge&logo=github" height="20">](https://github.com/udoprog/async-oauth2)
[<img alt="crates.io" src="https://img.shields.io/crates/v/async-oauth2.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/async-oauth2)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-async--oauth2-66c2a5?style=for-the-badge&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K" height="20">](https://docs.rs/async-oauth2)
[<img alt="build status" src="https://img.shields.io/github/workflow/status/udoprog/async-oauth2/CI/main?style=for-the-badge" height="20">](https://github.com/udoprog/async-oauth2/actions?query=branch%3Amain)

An asynchronous OAuth2 flow implementation, trying to adhere as much as
possible to [RFC 6749].

<br>

## Examples

To see the library in action, you can go to one of our examples:

- [Google]
- [Spotify]
- [Twitch]

If you've checked out the project they can be run like this:

```sh
cargo run --manifest-path=examples/Cargo.toml --bin spotify --
    --client-id <client-id> --client-secret <client-secret>
cargo run --manifest-path=examples/Cargo.toml --bin google --
    --client-id <client-id> --client-secret <client-secret>
cargo run --manifest-path=examples/Cargo.toml --bin twitch --
    --client-id <client-id> --client-secret <client-secret>
```

> Note: You need to configure your client integration to permit redirects to
> `http://localhost:8080/api/auth/redirect` for these to work. How this is
> done depends on the integration used.

<br>

## Authorization Code Grant

This is the most common OAuth2 flow.

```rust
use oauth2::*;
use url::Url;

pub struct ReceivedCode {
    pub code: AuthorizationCode,
    pub state: State,
}

let reqwest_client = reqwest::Client::new();

// Create an OAuth2 client by specifying the client ID, client secret,
// authorization URL and token URL.
let mut client = Client::new(
    "client_id",
    Url::parse("http://authorize")?,
    Url::parse("http://token")?
);

client.set_client_secret("client_secret");
// Set the URL the user will be redirected to after the authorization
// process.
client.set_redirect_url(Url::parse("http://redirect")?);
// Set the desired scopes.
client.add_scope("read");
client.add_scope("write");

// Generate the full authorization URL.
let state = State::new_random();
let auth_url = client.authorize_url(&state);

// This is the URL you should redirect the user to, in order to trigger the
// authorization process.
println!("Browse to: {}", auth_url);

// Once the user has been redirected to the redirect URL, you'll have the
// access code. For security reasons, your code should verify that the
// `state` parameter returned by the server matches `state`.
let received: ReceivedCode = listen_for_code(8080).await?;

if received.state != state {
   panic!("CSRF token mismatch :(");
}

// Now you can trade it for an access token.
let token = client.exchange_code(received.code)
    .with_client(&reqwest_client)
    .execute::<StandardToken>()
    .await?;

```

<br>

## Implicit Grant

This flow fetches an access token directly from the authorization endpoint.

Be sure to understand the security implications of this flow before using
it. In most cases the Authorization Code Grant flow above is preferred to
the Implicit Grant flow.

```rust
use oauth2::*;
use url::Url;

pub struct ReceivedCode {
    pub code: AuthorizationCode,
    pub state: State,
}

let mut client = Client::new(
    "client_id",
    Url::parse("http://authorize")?,
    Url::parse("http://token")?
);

client.set_client_secret("client_secret");

// Generate the full authorization URL.
let state = State::new_random();
let auth_url = client.authorize_url_implicit(&state);

// This is the URL you should redirect the user to, in order to trigger the
// authorization process.
println!("Browse to: {}", auth_url);

// Once the user has been redirected to the redirect URL, you'll have the
// access code. For security reasons, your code should verify that the
// `state` parameter returned by the server matches `state`.
let received: ReceivedCode = get_code().await?;

if received.state != state {
    panic!("CSRF token mismatch :(");
}

```

<br>

## Resource Owner Password Credentials Grant

You can ask for a *password* access token by calling the
`Client::exchange_password` method, while including the username and
password.

```rust
use oauth2::*;
use url::Url;

let reqwest_client = reqwest::Client::new();

let mut client = Client::new(
    "client_id",
    Url::parse("http://authorize")?,
    Url::parse("http://token")?
);

client.set_client_secret("client_secret");
client.add_scope("read");

let token = client
    .exchange_password("user", "pass")
    .with_client(&reqwest_client)
    .execute::<StandardToken>()
    .await?;

```

<br>

## Client Credentials Grant

You can ask for a *client credentials* access token by calling the
`Client::exchange_client_credentials` method.

```rust
use oauth2::*;
use url::Url;

let reqwest_client = reqwest::Client::new();
let mut client = Client::new(
    "client_id",
    Url::parse("http://authorize")?,
    Url::parse("http://token")?
);

client.set_client_secret("client_secret");
client.add_scope("read");

let token_result = client.exchange_client_credentials()
    .with_client(&reqwest_client)
    .execute::<StandardToken>();

```

<br>

## Relationship to oauth2-rs

This is a fork of [oauth2-rs].

The main differences are:
* Removal of unnecessary type parameters on Client ([see discussion here]).
* Only support one client implementation ([reqwest]).
* Remove most newtypes except `Scope` and the secret ones since they made the API harder to use.

[RFC 6749]: https://tools.ietf.org/html/rfc6749
[Google]: https://github.com/udoprog/async-oauth2/blob/master/examples/src/bin/google.rs
[oauth2-rs]: https://github.com/ramosbugs/oauth2-rs
[reqwest]: https://docs.rs/reqwest
[see discussion here]: https://github.com/ramosbugs/oauth2-rs/issues/44#issuecomment-50158653
[Spotify]: https://github.com/udoprog/async-oauth2/blob/master/examples/src/bin/spotify.rs
[Twitch]: https://github.com/udoprog/async-oauth2/blob/master/examples/src/bin/twitch.rs
