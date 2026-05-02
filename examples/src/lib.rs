use core::pin::pin;

use std::sync::Arc;

use anyhow::{anyhow, Result};
use axum::extract::State;
use axum::routing::post;
use axum::Form;
use axum::Router;
use serde::Deserialize;
use tokio::sync::oneshot;
use tokio::sync::oneshot::Sender;
use tokio::sync::Mutex;

pub struct Config {
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Deserialize)]
pub struct ReceivedCode {
    pub code: oauth2::AuthorizationCode,
    pub state: oauth2::State,
}

/// Interface to the server.
pub struct Server {
    channel: Mutex<Option<Sender<ReceivedCode>>>,
}

/// Get configuration from arguments.
pub fn config_from_args(name: &'static str) -> Result<Config> {
    let app = clap::Command::new(name)
        .about("Testing out OAuth 2.0 flows")
        .arg(
            clap::Arg::new("client-id")
                .long("client-id")
                .help("Client ID to use."),
        )
        .arg(
            clap::Arg::new("client-secret")
                .long("client-secret")
                .help("Client Secret to use."),
        );

    let m = app.get_matches();

    let client_id = m
        .get_one::<String>("client-id")
        .ok_or_else(|| anyhow!("missing: --client-id <argument>"))?
        .to_owned();

    let client_secret = m
        .get_one::<String>("client-secret")
        .ok_or_else(|| anyhow!("missing: --client-secret <argument>"))?
        .to_owned();

    Ok(Config {
        client_id,
        client_secret,
    })
}

/// Listen for a code at the specified port.
pub async fn listen_for_code(port: u32) -> Result<ReceivedCode> {
    let bind = format!("127.0.0.1:{port}");
    log::info!("Listening on: http://{bind}");

    let (tx, rx) = oneshot::channel::<ReceivedCode>();

    let server = Arc::new(Server {
        channel: Mutex::new(Some(tx)),
    });

    let app = Router::new()
        .fallback(post(receive_code))
        .with_state(server);

    let listener = tokio::net::TcpListener::bind(bind).await?;

    let server = axum::serve(listener, app);

    let mut server = pin!(async move { server.await });

    tokio::select! {
        _ = server.as_mut() => Err(anyhow!("server exited for some reason")),
        code = rx => match code {
            Ok(code) => Ok(code),
            Err(error) => Err(anyhow!("failed to receive code: {error}")),
        },
    }
}

#[axum::debug_handler]
async fn receive_code(State(server): State<Arc<Server>>, Form(code): Form<ReceivedCode>) {
    if let Some(tx) = server.channel.lock().await.take() {
        _ = tx.send(code);
    }
}
