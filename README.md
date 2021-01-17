# async-oauth2

[![Documentation](https://docs.rs/async-oauth2/badge.svg)](https://docs.rs/async-oauth2)
[![Crates](https://img.shields.io/crates/v/async-oauth2.svg)](https://crates.io/crates/async-oauth2)
[![Actions Status](https://github.com/udoprog/async-oauth2/workflows/Rust/badge.svg)](https://github.com/udoprog/async-oauth2/actions)

An asynchronous OAuth2 flow implementation, trying to adhere as much as
possible to [RFC 6749].

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

### Authorization Code Grant

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

### Implicit Grant

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

### Resource Owner Password Credentials Grant

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

### Client Credentials Grant

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

### Relationship to oauth2-rs

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

License: MIT/Apache-2.0
