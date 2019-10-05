use hyper::{body::Body, server, Request, Response};
use futures::{prelude::*, task::{Context, Poll}, channel::oneshot};
use tower_service::Service;
use std::net::SocketAddr;
use failure::{format_err, Error};
use serde::Deserialize;
use oauth2::{AuthorizationCode, State};

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
    type Error = Error;
    type Future = future::BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _: &mut Context) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        if let Ok(code) = serde_urlencoded::from_str::<ReceivedCode>(req.uri().query().unwrap_or("")) {
            if let Some(channel) = self.channel.take() {
                let _ = channel.send(code);
            }
        }

        Box::pin(future::ok(Response::new(Body::empty())))
    }
}

pub struct MakeSvc(Option<Server>);

impl<T> Service<T> for MakeSvc {
    type Response = Server;
    type Error = std::io::Error;
    type Future = future::Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Ok(()).into()
    }

    fn call(&mut self, _: T) -> Self::Future {
        future::ok(self.0.take().expect("server to not have been setup"))
    }
}

/// Get configuration from arguments.
pub fn config_from_args(name: &str) -> Result<Config, Error> {
    let app = clap::App::new(name)
        .about("Testing out OAuth 2.0 flows")
        .arg(
            clap::Arg::with_name("client-id")
                .takes_value(true)
                .long("client-id")
                .help("Client ID to use."),
        )
        .arg(
            clap::Arg::with_name("client-secret")
                .takes_value(true)
                .long("client-secret")
                .help("Client Secret to use."),
        );

    let m = app.get_matches();

    let client_id = m.value_of("client-id").ok_or_else(|| format_err!("missing: --client-id <argument>"))?.to_string();
    let client_secret = m.value_of("client-secret").ok_or_else(|| format_err!("missing: --client-secret <argument>"))?.to_string();

    Ok(Config {
        client_id,
        client_secret,
    })
}

/// Listen for a code at the specified port.
pub async fn listen_for_code(port: u32) -> Result<ReceivedCode, Error> {
    let bind = format!("127.0.0.1:{}", port);
    log::info!("Listening on: http://{}", bind);

    let addr: SocketAddr = str::parse(&bind)?;

    let (tx, rx) = oneshot::channel::<ReceivedCode>();

    let server = Server {
        channel: Some(tx),
    };

    let mut server_future = server::Server::bind(&addr).serve(MakeSvc(Some(server))).fuse();
    let mut rx = rx.fuse();

    futures::select! {
        _ = server_future => panic!("server exited for some reason"),
        received = rx => Ok(received?),
    }
}