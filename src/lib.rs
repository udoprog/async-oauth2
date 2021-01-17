//! [![Documentation](https://docs.rs/async-oauth2/badge.svg)](https://docs.rs/async-oauth2)
//! [![Crates](https://img.shields.io/crates/v/async-oauth2.svg)](https://crates.io/crates/async-oauth2)
//! [![Actions Status](https://github.com/udoprog/async-oauth2/workflows/Rust/badge.svg)](https://github.com/udoprog/async-oauth2/actions)
//!
//! An asynchronous OAuth2 flow implementation, trying to adhere as much as
//! possible to [RFC 6749].
//!
//! # Examples
//!
//! To see the library in action, you can go to one of our examples:
//!
//! - [Google]
//! - [Spotify]
//! - [Twitch]
//!
//! If you've checked out the project they can be run like this:
//!
//! ```sh
//! cargo run --manifest-path=examples/Cargo.toml --bin spotify --
//!     --client-id <client-id> --client-secret <client-secret>
//! cargo run --manifest-path=examples/Cargo.toml --bin google --
//!     --client-id <client-id> --client-secret <client-secret>
//! cargo run --manifest-path=examples/Cargo.toml --bin twitch --
//!     --client-id <client-id> --client-secret <client-secret>
//! ```
//!
//! > Note: You need to configure your client integration to permit redirects to
//! > `http://localhost:8080/api/auth/redirect` for these to work. How this is
//! > done depends on the integration used.
//!
//! ## Authorization Code Grant
//!
//! This is the most common OAuth2 flow.
//!
//! ```no_run
//! use oauth2::*;
//! use url::Url;
//!
//! pub struct ReceivedCode {
//!     pub code: AuthorizationCode,
//!     pub state: State,
//! }
//!
//! # async fn listen_for_code(port: u32) -> Result<ReceivedCode, Box<dyn std::error::Error>> { todo!() }
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let reqwest_client = reqwest::Client::new();
//!
//! // Create an OAuth2 client by specifying the client ID, client secret,
//! // authorization URL and token URL.
//! let mut client = Client::new(
//!     "client_id",
//!     Url::parse("http://authorize")?,
//!     Url::parse("http://token")?
//! );
//!
//! client.set_client_secret("client_secret");
//! // Set the URL the user will be redirected to after the authorization
//! // process.
//! client.set_redirect_url(Url::parse("http://redirect")?);
//! // Set the desired scopes.
//! client.add_scope("read");
//! client.add_scope("write");
//!
//! // Generate the full authorization URL.
//! let state = State::new_random();
//! let auth_url = client.authorize_url(&state);
//!
//! // This is the URL you should redirect the user to, in order to trigger the
//! // authorization process.
//! println!("Browse to: {}", auth_url);
//!
//! // Once the user has been redirected to the redirect URL, you'll have the
//! // access code. For security reasons, your code should verify that the
//! // `state` parameter returned by the server matches `state`.
//! let received: ReceivedCode = listen_for_code(8080).await?;
//!
//! if received.state != state {
//!    panic!("CSRF token mismatch :(");
//! }
//!
//! // Now you can trade it for an access token.
//! let token = client.exchange_code(received.code)
//!     .with_client(&reqwest_client)
//!     .execute::<StandardToken>()
//!     .await?;
//!
//! # Ok(())
//! # }
//! ```
//!
//! ## Implicit Grant
//!
//! This flow fetches an access token directly from the authorization endpoint.
//!
//! Be sure to understand the security implications of this flow before using
//! it. In most cases the Authorization Code Grant flow above is preferred to
//! the Implicit Grant flow.
//!
//! ```no_run
//! use oauth2::*;
//! use url::Url;
//!
//! pub struct ReceivedCode {
//!     pub code: AuthorizationCode,
//!     pub state: State,
//! }
//!
//! # async fn get_code() -> Result<ReceivedCode, Box<dyn std::error::Error>> { todo!() }
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let mut client = Client::new(
//!     "client_id",
//!     Url::parse("http://authorize")?,
//!     Url::parse("http://token")?
//! );
//!
//! client.set_client_secret("client_secret");
//!
//! // Generate the full authorization URL.
//! let state = State::new_random();
//! let auth_url = client.authorize_url_implicit(&state);
//!
//! // This is the URL you should redirect the user to, in order to trigger the
//! // authorization process.
//! println!("Browse to: {}", auth_url);
//!
//! // Once the user has been redirected to the redirect URL, you'll have the
//! // access code. For security reasons, your code should verify that the
//! // `state` parameter returned by the server matches `state`.
//! let received: ReceivedCode = get_code().await?;
//!
//! if received.state != state {
//!     panic!("CSRF token mismatch :(");
//! }
//!
//! # Ok(()) }
//! ```
//!
//! ## Resource Owner Password Credentials Grant
//!
//! You can ask for a *password* access token by calling the
//! `Client::exchange_password` method, while including the username and
//! password.
//!
//! ```no_run
//! use oauth2::*;
//! use url::Url;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let reqwest_client = reqwest::Client::new();
//!
//! let mut client = Client::new(
//!     "client_id",
//!     Url::parse("http://authorize")?,
//!     Url::parse("http://token")?
//! );
//!
//! client.set_client_secret("client_secret");
//! client.add_scope("read");
//!
//! let token = client
//!     .exchange_password("user", "pass")
//!     .with_client(&reqwest_client)
//!     .execute::<StandardToken>()
//!     .await?;
//!
//! # Ok(()) }
//! ```
//!
//! ## Client Credentials Grant
//!
//! You can ask for a *client credentials* access token by calling the
//! `Client::exchange_client_credentials` method.
//!
//! ```no_run
//! use oauth2::*;
//! use url::Url;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let reqwest_client = reqwest::Client::new();
//! let mut client = Client::new(
//!     "client_id",
//!     Url::parse("http://authorize")?,
//!     Url::parse("http://token")?
//! );
//!
//! client.set_client_secret("client_secret");
//! client.add_scope("read");
//!
//! let token_result = client.exchange_client_credentials()
//!     .with_client(&reqwest_client)
//!     .execute::<StandardToken>();
//!
//! # Ok(()) }
//! ```
//!
//! ## Relationship to oauth2-rs
//!
//! This is a fork of [oauth2-rs].
//!
//! The main differences are:
//! * Removal of unnecessary type parameters on Client ([see discussion here]).
//! * Only support one client implementation ([reqwest]).
//! * Remove most newtypes except `Scope` and the secret ones since they made the API harder to use.
//!
//! [RFC 6749]: https://tools.ietf.org/html/rfc6749
//! [Google]: https://github.com/udoprog/async-oauth2/blob/master/examples/src/bin/google.rs
//! [oauth2-rs]: https://github.com/ramosbugs/oauth2-rs
//! [reqwest]: https://docs.rs/reqwest
//! [see discussion here]: https://github.com/ramosbugs/oauth2-rs/issues/44#issuecomment-50158653
//! [Spotify]: https://github.com/udoprog/async-oauth2/blob/master/examples/src/bin/spotify.rs
//! [Twitch]: https://github.com/udoprog/async-oauth2/blob/master/examples/src/bin/twitch.rs

#![deny(missing_docs)]

use std::{borrow::Cow, error, fmt, time::Duration};

use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use thiserror::Error;
pub use url::Url;

/// Indicates whether requests to the authorization server should use basic authentication or
/// include the parameters in the request body for requests in which either is valid.
///
/// The default AuthType is *BasicAuth*, following the recommendation of
/// [Section 2.3.1 of RFC 6749](https://tools.ietf.org/html/rfc6749#section-2.3.1).
#[derive(Clone, Copy, Debug)]
pub enum AuthType {
    /// The client_id and client_secret will be included as part of the request body.
    RequestBody,
    /// The client_id and client_secret will be included using the basic auth authentication scheme.
    BasicAuth,
}

macro_rules! redacted_debug {
    ($name:ident) => {
        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, concat!(stringify!($name), "([redacted])"))
            }
        }
    };
}

/// borrowed newtype plumbing
macro_rules! borrowed_newtype {
    ($name:ident, $borrowed:ty) => {
        impl std::ops::Deref for $name {
            type Target = $borrowed;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl<'a> Into<Cow<'a, $borrowed>> for &'a $name {
            fn into(self) -> Cow<'a, $borrowed> {
                Cow::Borrowed(&self.0)
            }
        }

        impl AsRef<$borrowed> for $name {
            fn as_ref(&self) -> &$borrowed {
                self
            }
        }
    };
}

/// newtype plumbing
macro_rules! newtype {
    ($name:ident, $owned:ty, $borrowed:ty) => {
        borrowed_newtype!($name, $borrowed);

        impl<'a> From<&'a $borrowed> for $name {
            fn from(value: &'a $borrowed) -> Self {
                Self(value.to_owned())
            }
        }

        impl From<$owned> for $name {
            fn from(value: $owned) -> Self {
                Self(value)
            }
        }

        impl<'a> From<&'a $owned> for $name {
            fn from(value: &'a $owned) -> Self {
                Self(value.to_owned())
            }
        }

        impl<'a> Into<$owned> for $name {
            fn into(self) -> $owned {
                self.0
            }
        }
    };
}

/// Access token scope, as defined by the authorization server.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct Scope(String);
newtype!(Scope, String, str);

/// Code Challenge used for [PKCE]((https://tools.ietf.org/html/rfc7636)) protection via the
/// `code_challenge` parameter.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct PkceCodeChallengeS256(String);
newtype!(PkceCodeChallengeS256, String, str);

/// Code Challenge Method used for [PKCE]((https://tools.ietf.org/html/rfc7636)) protection
/// via the `code_challenge_method` parameter.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct PkceCodeChallengeMethod(String);
newtype!(PkceCodeChallengeMethod, String, str);

/// Client password issued to the client during the registration process described by
/// [Section 2.2](https://tools.ietf.org/html/rfc6749#section-2.2).
#[derive(Clone, Deserialize, Serialize)]
pub struct ClientSecret(String);
redacted_debug!(ClientSecret);
newtype!(ClientSecret, String, str);

/// Value used for [CSRF]((https://tools.ietf.org/html/rfc6749#section-10.12)) protection
/// via the `state` parameter.
#[must_use]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct State([u8; 16]);
redacted_debug!(State);
borrowed_newtype!(State, [u8]);

impl State {
    /// Generate a new random, base64-encoded 128-bit CSRF token.
    pub fn new_random() -> Self {
        let mut random_bytes = [0u8; 16];
        thread_rng().fill(&mut random_bytes);
        State(random_bytes)
    }

    /// Convert into base64.
    pub fn to_base64(&self) -> String {
        base64::encode_config(&self.0, base64::URL_SAFE_NO_PAD)
    }
}

impl serde::Serialize for State {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_base64().serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for State {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let bytes =
            base64::decode_config(&s, base64::URL_SAFE_NO_PAD).map_err(serde::de::Error::custom)?;
        let mut buf = [0u8; 16];
        buf.copy_from_slice(&bytes);
        Ok(Self(buf))
    }
}

/// Code Verifier used for [PKCE]((https://tools.ietf.org/html/rfc7636)) protection via the
/// `code_verifier` parameter. The value must have a minimum length of 43 characters and a
/// maximum length of 128 characters.  Each character must be ASCII alphanumeric or one of
/// the characters "-" / "." / "_" / "~".
#[derive(Deserialize, Serialize)]
pub struct PkceCodeVerifierS256(String);
newtype!(PkceCodeVerifierS256, String, str);

impl PkceCodeVerifierS256 {
    /// Generate a new random, base64-encoded code verifier.
    pub fn new_random() -> Self {
        PkceCodeVerifierS256::new_random_len(32)
    }

    /// Generate a new random, base64-encoded code verifier.
    ///
    /// # Arguments
    ///
    /// * `num_bytes` - Number of random bytes to generate, prior to base64-encoding.
    ///   The value must be in the range 32 to 96 inclusive in order to generate a verifier
    ///   with a suitable length.
    pub fn new_random_len(num_bytes: u32) -> Self {
        // The RFC specifies that the code verifier must have "a minimum length of 43
        // characters and a maximum length of 128 characters".
        // This implies 32-96 octets of random data to be base64 encoded.
        assert!(num_bytes >= 32 && num_bytes <= 96);
        let random_bytes: Vec<u8> = (0..num_bytes).map(|_| thread_rng().gen::<u8>()).collect();
        let code = base64::encode_config(&random_bytes, base64::URL_SAFE_NO_PAD);
        assert!(code.len() >= 43 && code.len() <= 128);
        PkceCodeVerifierS256(code)
    }

    /// Return the code challenge for the code verifier.
    pub fn code_challenge(&self) -> PkceCodeChallengeS256 {
        let digest = Sha256::digest(self.as_bytes());
        PkceCodeChallengeS256::from(base64::encode_config(&digest, base64::URL_SAFE_NO_PAD))
    }

    /// Return the code challenge method for this code verifier.
    pub fn code_challenge_method() -> PkceCodeChallengeMethod {
        PkceCodeChallengeMethod::from("S256".to_string())
    }

    /// Return the extension params used for authorize_url.
    pub fn authorize_url_params(&self) -> Vec<(&'static str, String)> {
        vec![
            (
                "code_challenge_method",
                PkceCodeVerifierS256::code_challenge_method().into(),
            ),
            ("code_challenge", self.code_challenge().into()),
        ]
    }
}

/// Authorization code returned from the authorization endpoint.
#[derive(Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AuthorizationCode(String);
redacted_debug!(AuthorizationCode);
newtype!(AuthorizationCode, String, str);

/// Refresh token used to obtain a new access token (if supported by the authorization server).
#[derive(Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RefreshToken(String);
redacted_debug!(RefreshToken);
newtype!(RefreshToken, String, str);

/// Access token returned by the token endpoint and used to access protected resources.
#[derive(Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AccessToken(String);
redacted_debug!(AccessToken);
newtype!(AccessToken, String, str);

/// Resource owner's password used directly as an authorization grant to obtain an access
/// token.
pub struct ResourceOwnerPassword(String);
newtype!(ResourceOwnerPassword, String, str);

/// Stores the configuration for an OAuth2 client.
#[derive(Clone, Debug)]
pub struct Client {
    client_id: String,
    client_secret: Option<ClientSecret>,
    auth_url: Url,
    auth_type: AuthType,
    token_url: Url,
    scopes: Vec<Scope>,
    redirect_url: Option<Url>,
}

impl Client {
    /// Initializes an OAuth2 client with the fields common to most OAuth2 flows.
    ///
    /// # Arguments
    ///
    /// * `client_id` -  Client ID
    /// * `auth_url` -  Authorization endpoint: used by the client to obtain authorization from
    ///   the resource owner via user-agent redirection. This URL is used in all standard OAuth2
    ///   flows except the [Resource Owner Password Credentials
    ///   Grant](https://tools.ietf.org/html/rfc6749#section-4.3) and the
    ///   [Client Credentials Grant](https://tools.ietf.org/html/rfc6749#section-4.4).
    /// * `token_url` - Token endpoint: used by the client to exchange an authorization grant
    ///   (code) for an access token, typically with client authentication. This URL is used in
    ///   all standard OAuth2 flows except the
    ///   [Implicit Grant](https://tools.ietf.org/html/rfc6749#section-4.2). If this value is set
    ///   to `None`, the `exchange_*` methods will return `Err(RequestTokenError::Other(_))`.
    pub fn new(client_id: impl AsRef<str>, auth_url: Url, token_url: Url) -> Self {
        Client {
            client_id: client_id.as_ref().to_string(),
            client_secret: None,
            auth_url,
            auth_type: AuthType::BasicAuth,
            token_url,
            scopes: Vec::new(),
            redirect_url: None,
        }
    }

    /// Configure the client secret to use.
    pub fn set_client_secret(&mut self, client_secret: impl Into<ClientSecret>) {
        self.client_secret = Some(client_secret.into());
    }

    /// Appends a new scope to the authorization URL.
    pub fn add_scope(&mut self, scope: impl Into<Scope>) {
        self.scopes.push(scope.into());
    }

    /// Configures the type of client authentication used for communicating with the authorization
    /// server.
    ///
    /// The default is to use HTTP Basic authentication, as recommended in
    /// [Section 2.3.1 of RFC 6749](https://tools.ietf.org/html/rfc6749#section-2.3.1).
    pub fn set_auth_type(&mut self, auth_type: AuthType) {
        self.auth_type = auth_type;
    }

    /// Sets the the redirect URL used by the authorization endpoint.
    pub fn set_redirect_url(&mut self, redirect_url: Url) {
        self.redirect_url = Some(redirect_url);
    }

    /// Produces the full authorization URL used by the
    /// [Authorization Code Grant](https://tools.ietf.org/html/rfc6749#section-4.1)
    /// flow, which is the most common OAuth2 flow.
    ///
    /// # Arguments
    ///
    /// * `state` - A state value to include in the request. The authorization
    ///   server includes this value when redirecting the user-agent back to the
    ///   client.
    ///
    /// # Security Warning
    ///
    /// Callers should use a fresh, unpredictable `state` for each authorization
    /// request and verify that this value matches the `state` parameter passed
    /// by the authorization server to the redirect URI. Doing so mitigates
    /// [Cross-Site Request Forgery](https://tools.ietf.org/html/rfc6749#section-10.12)
    /// attacks.
    pub fn authorize_url(&self, state: &State) -> Url {
        self.authorize_url_impl("code", state)
    }

    /// Produces the full authorization URL used by the
    /// [Implicit Grant](https://tools.ietf.org/html/rfc6749#section-4.2) flow.
    ///
    /// # Arguments
    ///
    /// * `state` - A state value to include in the request. The authorization
    ///   server includes this value when redirecting the user-agent back to the
    ///   client.
    ///
    /// # Security Warning
    ///
    /// Callers should use a fresh, unpredictable `state` for each authorization request and verify
    /// that this value matches the `state` parameter passed by the authorization server to the
    /// redirect URI. Doing so mitigates
    /// [Cross-Site Request Forgery](https://tools.ietf.org/html/rfc6749#section-10.12)
    ///  attacks.
    pub fn authorize_url_implicit(&self, state: &State) -> Url {
        self.authorize_url_impl("token", state)
    }

    fn authorize_url_impl(&self, response_type: &str, state: &State) -> Url {
        let scopes = self
            .scopes
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
            .join(" ");

        let mut url = self.auth_url.clone();

        {
            let mut query = url.query_pairs_mut();

            query.append_pair("response_type", response_type);
            query.append_pair("client_id", &self.client_id);

            if let Some(ref redirect_url) = self.redirect_url {
                query.append_pair("redirect_uri", redirect_url.as_str());
            }

            if !scopes.is_empty() {
                query.append_pair("scope", &scopes);
            }

            query.append_pair("state", &state.to_base64());
        }

        url
    }

    /// Exchanges a code produced by a successful authorization process with an access token.
    ///
    /// Acquires ownership of the `code` because authorization codes may only be used to retrieve
    /// an access token from the authorization server.
    ///
    /// See https://tools.ietf.org/html/rfc6749#section-4.1.3
    pub fn exchange_code(&self, code: impl Into<AuthorizationCode>) -> Request<'_> {
        let code = code.into();

        self.request_token()
            .param("grant_type", "authorization_code")
            .param("code", code.to_string())
    }

    /// Requests an access token for the *password* grant type.
    ///
    /// See https://tools.ietf.org/html/rfc6749#section-4.3.2
    pub fn exchange_password<'a>(
        &'a self,
        username: impl AsRef<str>,
        password: impl AsRef<str>,
    ) -> Request<'a> {
        let username = username.as_ref();
        let password = password.as_ref();

        let mut builder = self
            .request_token()
            .param("grant_type", "password")
            .param("username", username.to_string())
            .param("password", password.to_string());

        // Generate the space-delimited scopes String before initializing params so that it has
        // a long enough lifetime.
        if !self.scopes.is_empty() {
            let scopes = self
                .scopes
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .join(" ");

            builder = builder.param("scope", scopes);
        }

        builder
    }

    /// Requests an access token for the *client credentials* grant type.
    ///
    /// See https://tools.ietf.org/html/rfc6749#section-4.4.2
    pub fn exchange_client_credentials(&self) -> Request<'_> {
        let mut builder = self
            .request_token()
            .param("grant_type", "client_credentials");

        // Generate the space-delimited scopes String before initializing params so that it has
        // a long enough lifetime.
        if !self.scopes.is_empty() {
            let scopes = self
                .scopes
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .join(" ");

            builder = builder.param("scopes", scopes);
        }

        builder
    }

    /// Exchanges a refresh token for an access token
    ///
    /// See https://tools.ietf.org/html/rfc6749#section-6
    pub fn exchange_refresh_token(&self, refresh_token: &RefreshToken) -> Request<'_> {
        self.request_token()
            .param("grant_type", "refresh_token")
            .param("refresh_token", refresh_token.to_string())
    }

    /// Construct a request builder for the token URL.
    fn request_token(&self) -> Request<'_> {
        Request {
            token_url: &self.token_url,
            auth_type: self.auth_type,
            client_id: &self.client_id,
            client_secret: self.client_secret.as_ref(),
            redirect_url: self.redirect_url.as_ref(),
            params: vec![],
        }
    }
}

/// A request wrapped in a client, ready to be executed.
pub struct ClientRequest<'a, 'client> {
    request: Request<'a>,
    client: &'client reqwest::Client,
}

impl<'a, 'b> ClientRequest<'a, 'b> {
    /// Execute the token request.
    pub async fn execute<T>(self) -> Result<T, RequestTokenError>
    where
        T: Token,
    {
        use self::RequestTokenError::*;
        use reqwest::{header, Method};

        let token_url = self.request.token_url;

        let mut request = self.client.request(Method::POST, &token_url.to_string());

        // Section 5.1 of RFC 6749 (https://tools.ietf.org/html/rfc6749#section-5.1) only permits
        // JSON responses for this request. Some providers such as GitHub have off-spec behavior
        // and not only support different response formats, but have non-JSON defaults. Explicitly
        // request JSON here.
        request = request.header(
            header::ACCEPT,
            header::HeaderValue::from_static(CONTENT_TYPE_JSON),
        );

        let request = {
            let mut form = url::form_urlencoded::Serializer::new(String::new());

            // FIXME: add support for auth extensions? e.g., client_secret_jwt and private_key_jwt
            match self.request.auth_type {
                AuthType::RequestBody => {
                    form.append_pair("client_id", self.request.client_id);

                    if let Some(client_secret) = self.request.client_secret {
                        form.append_pair("client_secret", client_secret);
                    }
                }
                AuthType::BasicAuth => {
                    // Section 2.3.1 of RFC 6749 requires separately url-encoding the id and secret
                    // before using them as HTTP Basic auth username and password. Note that this is
                    // not standard for ordinary Basic auth, so curl won't do it for us.
                    let username = url_encode(self.request.client_id);

                    let password = match self.request.client_secret {
                        Some(client_secret) => Some(url_encode(client_secret)),
                        None => None,
                    };

                    request = request.basic_auth(&username, password.as_ref());
                }
            }

            for (key, value) in self.request.params {
                form.append_pair(key.as_ref(), value.as_ref());
            }

            if let Some(ref redirect_url) = self.request.redirect_url {
                form.append_pair("redirect_uri", redirect_url.as_str());
            }

            request = request.header(
                header::CONTENT_TYPE,
                header::HeaderValue::from_static("application/x-www-form-urlencoded"),
            );

            request.body(form.finish().into_bytes())
        };

        let res = request
            .send()
            .await
            .map_err(|error| ReqwestError { error })?;

        let status = res.status();

        let body = res.bytes().await.map_err(|error| ReqwestError { error })?;

        if body.is_empty() {
            return Err(EmptyResponse { status });
        }

        if !status.is_success() {
            let error = match serde_json::from_slice::<self::ErrorResponse>(body.as_ref()) {
                Ok(error) => error,
                Err(error) => {
                    return Err(BadResponse {
                        status,
                        error,
                        body,
                    });
                }
            };

            return Err(RequestTokenError::ErrorResponse { status, error });
        }

        return serde_json::from_slice(body.as_ref()).map_err(|error| BadResponse {
            status,
            error,
            body,
        });

        fn url_encode(s: &str) -> String {
            url::form_urlencoded::byte_serialize(s.as_bytes()).collect::<String>()
        }

        const CONTENT_TYPE_JSON: &str = "application/json";
    }
}

/// A token request that is in progress.
pub struct Request<'a> {
    token_url: &'a Url,
    auth_type: AuthType,
    client_id: &'a str,
    client_secret: Option<&'a ClientSecret>,
    /// Configured redirect URL.
    redirect_url: Option<&'a Url>,
    /// Extra parameters.
    params: Vec<(Cow<'a, str>, Cow<'a, str>)>,
}

impl<'a> Request<'a> {
    /// Set an additional request param.
    pub fn param(mut self, key: impl Into<Cow<'a, str>>, value: impl Into<Cow<'a, str>>) -> Self {
        self.params.push((key.into(), value.into()));
        self
    }

    /// Wrap the request in a client.
    pub fn with_client<'client>(
        self,
        client: &'client reqwest::Client,
    ) -> ClientRequest<'a, 'client> {
        ClientRequest {
            client,
            request: self,
        }
    }
}

/// Basic OAuth2 authorization token types.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum TokenType {
    /// Bearer token
    /// ([OAuth 2.0 Bearer Tokens - RFC 6750](https://tools.ietf.org/html/rfc6750)).
    Bearer,
    /// MAC ([OAuth 2.0 Message Authentication Code (MAC)
    /// Tokens](https://tools.ietf.org/html/draft-ietf-oauth-v2-http-mac-05)).
    Mac,
}

impl<'de> serde::de::Deserialize<'de> for TokenType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?.to_lowercase();

        return match value.as_str() {
            "bearer" => Ok(TokenType::Bearer),
            "mac" => Ok(TokenType::Mac),
            other => Err(serde::de::Error::custom(UnknownVariantError(
                other.to_string(),
            ))),
        };

        #[derive(Debug)]
        struct UnknownVariantError(String);

        impl error::Error for UnknownVariantError {}

        impl fmt::Display for UnknownVariantError {
            fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
                write!(fmt, "unsupported variant: {}", self.0)
            }
        }
    }
}

/// Common methods shared by all OAuth2 token implementations.
///
/// The methods in this trait are defined in
/// [Section 5.1 of RFC 6749](https://tools.ietf.org/html/rfc6749#section-5.1). This trait exists
/// separately from the `StandardToken` struct to support customization by clients,
/// such as supporting interoperability with non-standards-complaint OAuth2 providers.
pub trait Token
where
    Self: for<'a> serde::de::Deserialize<'a>,
{
    /// REQUIRED. The access token issued by the authorization server.
    fn access_token(&self) -> &AccessToken;

    /// REQUIRED. The type of the token issued as described in
    /// [Section 7.1](https://tools.ietf.org/html/rfc6749#section-7.1).
    /// Value is case insensitive and deserialized to the generic `TokenType` parameter.
    fn token_type(&self) -> &TokenType;

    /// RECOMMENDED. The lifetime in seconds of the access token. For example, the value 3600
    /// denotes that the access token will expire in one hour from the time the response was
    /// generated. If omitted, the authorization server SHOULD provide the expiration time via
    /// other means or document the default value.
    fn expires_in(&self) -> Option<Duration>;

    /// OPTIONAL. The refresh token, which can be used to obtain new access tokens using the same
    /// authorization grant as described in
    /// [Section 6](https://tools.ietf.org/html/rfc6749#section-6).
    fn refresh_token(&self) -> Option<&RefreshToken>;

    /// OPTIONAL, if identical to the scope requested by the client; otherwise, REQUIRED. The
    /// scipe of the access token as described by
    /// [Section 3.3](https://tools.ietf.org/html/rfc6749#section-3.3). If included in the response,
    /// this space-delimited field is parsed into a `Vec` of individual scopes. If omitted from
    /// the response, this field is `None`.
    fn scopes(&self) -> Option<&Vec<Scope>>;
}

/// Standard OAuth2 token response.
///
/// This struct includes the fields defined in
/// [Section 5.1 of RFC 6749](https://tools.ietf.org/html/rfc6749#section-5.1), as well as
/// extensions defined by the `EF` type parameter.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct StandardToken {
    access_token: AccessToken,
    token_type: TokenType,
    #[serde(skip_serializing_if = "Option::is_none")]
    expires_in: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    refresh_token: Option<RefreshToken>,
    #[serde(rename = "scope")]
    #[serde(deserialize_with = "helpers::deserialize_space_delimited_vec")]
    #[serde(serialize_with = "helpers::serialize_space_delimited_vec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    scopes: Option<Vec<Scope>>,
}

impl Token for StandardToken {
    /// REQUIRED. The access token issued by the authorization server.
    fn access_token(&self) -> &AccessToken {
        &self.access_token
    }

    /// REQUIRED. The type of the token issued as described in
    /// [Section 7.1](https://tools.ietf.org/html/rfc6749#section-7.1).
    /// Value is case insensitive and deserialized to the generic `TokenType` parameter.
    fn token_type(&self) -> &TokenType {
        &self.token_type
    }

    /// RECOMMENDED. The lifetime in seconds of the access token. For example, the value 3600
    /// denotes that the access token will expire in one hour from the time the response was
    /// generated. If omitted, the authorization server SHOULD provide the expiration time via
    /// other means or document the default value.
    fn expires_in(&self) -> Option<Duration> {
        self.expires_in.map(Duration::from_secs)
    }

    /// OPTIONAL. The refresh token, which can be used to obtain new access tokens using the same
    /// authorization grant as described in
    /// [Section 6](https://tools.ietf.org/html/rfc6749#section-6).
    fn refresh_token(&self) -> Option<&RefreshToken> {
        self.refresh_token.as_ref()
    }

    /// OPTIONAL, if identical to the scope requested by the client; otherwise, REQUIRED. The
    /// scipe of the access token as described by
    /// [Section 3.3](https://tools.ietf.org/html/rfc6749#section-3.3). If included in the response,
    /// this space-delimited field is parsed into a `Vec` of individual scopes. If omitted from
    /// the response, this field is `None`.
    fn scopes(&self) -> Option<&Vec<Scope>> {
        self.scopes.as_ref()
    }
}

/// These error types are defined in
/// [Section 5.2 of RFC 6749](https://tools.ietf.org/html/rfc6749#section-5.2).
#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ErrorField {
    /// The request is missing a required parameter, includes an unsupported parameter value
    /// (other than grant type), repeats a parameter, includes multiple credentials, utilizes
    /// more than one mechanism for authenticating the client, or is otherwise malformed.
    InvalidRequest,
    /// Client authentication failed (e.g., unknown client, no client authentication included,
    /// or unsupported authentication method).
    InvalidClient,
    /// The provided authorization grant (e.g., authorization code, resource owner credentials)
    /// or refresh token is invalid, expired, revoked, does not match the redirection URI used
    /// in the authorization request, or was issued to another client.
    InvalidGrant,
    /// The authenticated client is not authorized to use this authorization grant type.
    UnauthorizedClient,
    /// The authorization grant type is not supported by the authorization server.
    UnsupportedGrantType,
    /// The requested scope is invalid, unknown, malformed, or exceeds the scope granted by the
    /// resource owner.
    InvalidScope,
    /// Other error type.
    Other(String),
}

impl fmt::Display for ErrorField {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        use self::ErrorField::*;

        match *self {
            InvalidRequest => "invalid_request".fmt(fmt),
            InvalidClient => "invalid_client".fmt(fmt),
            InvalidGrant => "invalid_grant".fmt(fmt),
            UnauthorizedClient => "unauthorized_client".fmt(fmt),
            UnsupportedGrantType => "unsupported_grant_type".fmt(fmt),
            InvalidScope => "invalid_scope".fmt(fmt),
            Other(ref value) => value.fmt(fmt),
        }
    }
}

/// Error response returned by server after requesting an access token.
///
/// The fields in this structure are defined in
/// [Section 5.2 of RFC 6749](https://tools.ietf.org/html/rfc6749#section-5.2).
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ErrorResponse {
    /// A single ASCII error code.
    pub error: ErrorField,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Human-readable ASCII text providing additional information, used to assist
    /// the client developer in understanding the error that occurred.
    pub error_description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A URI identifying a human-readable web page with information about the error,
    /// used to provide the client developer with additional information about the error.
    pub error_uri: Option<String>,
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut formatted = self.error.to_string();

        if let Some(error_description) = self.error_description.as_ref() {
            formatted.push_str(": ");
            formatted.push_str(error_description);
        }

        if let Some(error_uri) = self.error_uri.as_ref() {
            formatted.push_str(" / See ");
            formatted.push_str(error_uri);
        }

        write!(f, "{}", formatted)
    }
}

impl error::Error for ErrorResponse {}

/// Errors when creating new clients.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum NewClientError {
    /// Error creating underlying reqwest client.
    #[error("Failed to construct client")]
    Reqwest(#[source] reqwest::Error),
}

impl From<reqwest::Error> for NewClientError {
    fn from(error: reqwest::Error) -> Self {
        Self::Reqwest(error)
    }
}

/// Error encountered while requesting access token.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum RequestTokenError {
    /// A client error that occured.
    #[error("reqwest error")]
    ReqwestError {
        /// Original request error.
        #[source]
        error: reqwest::Error,
    },
    /// Failed to parse server response. Parse errors may occur while parsing either successful
    /// or error responses.
    #[error("malformed server response: {status}")]
    BadResponse {
        /// The status code associated with the response.
        status: http::status::StatusCode,
        /// The body that couldn't be deserialized.
        body: bytes::Bytes,
        /// Deserialization error.
        #[source]
        error: serde_json::error::Error,
    },
    /// Response with non-successful status code and a body that could be
    /// successfully deserialized as an [ErrorResponse].
    #[error("request resulted in error response: {status}")]
    ErrorResponse {
        /// The status code associated with the response.
        status: http::status::StatusCode,
        /// The deserialized response.
        #[source]
        error: ErrorResponse,
    },
    /// Server response was empty.
    #[error("request resulted in empty response: {status}")]
    EmptyResponse {
        /// The status code associated with the empty response.
        status: http::status::StatusCode,
    },
}

impl RequestTokenError {
    /// Access the status code of the error if available.
    pub fn status(&self) -> Option<http::status::StatusCode> {
        match *self {
            Self::ReqwestError { ref error, .. } => error.status(),
            Self::BadResponse { status, .. } => Some(status),
            Self::ErrorResponse { status, .. } => Some(status),
            Self::EmptyResponse { status, .. } => Some(status),
        }
    }

    /// The original response body if available.
    pub fn body(&self) -> Option<&bytes::Bytes> {
        match *self {
            Self::BadResponse { ref body, .. } => Some(body),
            _ => None,
        }
    }
}

/// Helper methods used by OAuth2 implementations/extensions.
pub mod helpers {
    use serde::{Deserialize, Deserializer, Serializer};
    use url::Url;

    /// Serde space-delimited string deserializer for a `Vec<String>`.
    ///
    /// This function splits a JSON string at each space character into a `Vec<String>` .
    ///
    /// # Example
    ///
    /// In example below, the JSON value `{"items": "foo bar baz"}` would deserialize to:
    ///
    /// ```
    /// # struct GroceryBasket {
    /// #     items: Vec<String>,
    /// # }
    /// # fn main() {
    /// GroceryBasket {
    ///     items: vec!["foo".to_string(), "bar".to_string(), "baz".to_string()]
    /// };
    /// # }
    /// ```
    ///
    /// Note: this example does not compile automatically due to
    /// [Rust issue #29286](https://github.com/rust-lang/rust/issues/29286).
    ///
    /// ```
    /// # /*
    /// use serde::Deserialize;
    ///
    /// #[derive(Deserialize)]
    /// struct GroceryBasket {
    ///     #[serde(deserialize_with = "helpers::deserialize_space_delimited_vec")]
    ///     items: Vec<String>,
    /// }
    /// # */
    /// ```
    pub fn deserialize_space_delimited_vec<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where
        T: Default + Deserialize<'de>,
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        use serde_json::Value;

        if let Some(space_delimited) = Option::<String>::deserialize(deserializer)? {
            let entries = space_delimited
                .split(' ')
                .map(|s| Value::String(s.to_string()))
                .collect();
            return T::deserialize(Value::Array(entries)).map_err(Error::custom);
        }

        // If the JSON value is null, use the default value.
        Ok(T::default())
    }

    /// Serde space-delimited string serializer for an `Option<Vec<String>>`.
    ///
    /// This function serializes a string vector into a single space-delimited string.
    /// If `string_vec_opt` is `None`, the function serializes it as `None` (e.g., `null`
    /// in the case of JSON serialization).
    pub fn serialize_space_delimited_vec<T, S>(
        vec_opt: &Option<Vec<T>>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        T: AsRef<str>,
        S: Serializer,
    {
        if let Some(ref vec) = *vec_opt {
            let space_delimited = vec.iter().map(|s| s.as_ref()).collect::<Vec<_>>().join(" ");
            serializer.serialize_str(&space_delimited)
        } else {
            serializer.serialize_none()
        }
    }

    /// Serde string deserializer for a `Url`.
    pub fn deserialize_url<'de, D>(deserializer: D) -> Result<Url, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        let url_str = String::deserialize(deserializer)?;
        Url::parse(url_str.as_ref()).map_err(Error::custom)
    }

    /// Serde string serializer for a `Url`.
    pub fn serialize_url<S>(url: &Url, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(url.as_str())
    }
}
