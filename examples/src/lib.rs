use std::future::ready;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use anyhow::{anyhow, Result};
use hyper::{body::Body, server, service, Request, Response};
use oauth2::{AuthorizationCode, State};
use serde::Deserialize;
use std::net::SocketAddr;
use tokio::sync::oneshot;
use tower_service::Service;

type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + Sync + 'a>>;

pub struct Config {
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Deserialize)]
pub struct ReceivedCode {
    pub code: AuthorizationCode,
    pub state: State,
}

/// Interface to the server.
pub struct Server {
    channel: Option<oneshot::Sender<ReceivedCode>>,
}

impl Service<Request<Body>> for Server {
    type Response = Response<Body>;
    type Error = anyhow::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _: &mut Context) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        if let Ok(code) =
            serde_urlencoded::from_str::<ReceivedCode>(req.uri().query().unwrap_or(""))
        {
            if let Some(channel) = self.channel.take() {
                let _ = channel.send(code);
            }
        }

        Box::pin(ready(Ok(Response::new(Body::empty()))))
    }
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
    let bind = format!("127.0.0.1:{}", port);
    log::info!("Listening on: http://{}", bind);

    let addr: SocketAddr = str::parse(&bind)?;

    let (tx, rx) = oneshot::channel::<ReceivedCode>();

    let mut channel = Some(tx);

    let server_future = server::Server::bind(&addr).serve(service::make_service_fn(move |_| {
        let channel = channel.take().expect("channel is not available");
        let mut server = Server {
            channel: Some(channel),
        };
        let service = service::service_fn(move |req| server.call(req));

        async move { Ok::<_, hyper::Error>(service) }
    }));

    tokio::select! {
        _ = server_future => panic!("server exited for some reason"),
        received = rx => Ok(received?),
    }
}
