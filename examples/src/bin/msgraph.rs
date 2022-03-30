//! Showcases how to define and use a nonstandard token type.
//!
//! Note: MSGraph requires you to set `client_id` and `client_secret` as extra
//! parameters when performing the token exchange (see below).

use oauth2::{Client, StandardToken, Url};

use anyhow::{anyhow, Result};

pub struct ConfigMS {
    pub client_id: String,
    pub client_secret: String,
    /// Tennant ID, Required in the url by Microsoft.
    pub tenant_domain: String,
}

pub fn config_from_args_ms(name: &str) -> Result<ConfigMS> {
    let app = clap::Command::new(name)
        .about("Testing out OAuth 2.0 flows")
        .arg(
            clap::Arg::new("client-id")
                .takes_value(true)
                .long("client-id")
                .help("Client ID to use."),
        )
        .arg(
            clap::Arg::new("client-secret")
                .takes_value(true)
                .long("client-secret")
                .help("Client Secret to use."),
        )
        .arg(
            clap::Arg::new("tenant-domain")
                .takes_value(true)
                .long("tenant-domain")
                .help("Tenant domain to use."),
        );

    let m = app.get_matches();

    let client_id = m
        .value_of("client-id")
        .ok_or_else(|| anyhow!("missing: --client-id <argument>"))?
        .to_string();
    let client_secret = m
        .value_of("client-secret")
        .ok_or_else(|| anyhow!("missing: --client-secret <argument>"))?
        .to_string();

    let tenant_domain = m
        .value_of("tenant-domain")
        .ok_or_else(|| anyhow!("missing: --tenant-domain <argument>"))?
        .to_string();

    Ok(ConfigMS {
        client_id,
        client_secret,
        tenant_domain,
    })
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = config_from_args_ms("msgraph Example")?;

    let reqwest_client = reqwest::Client::new();

    let auth_url = Url::parse(
        format!(
            "https://login.microsoftonline.com/{}/oauth2/authorize",
            config.tenant_domain
        )
        .as_str(),
    )?;
    let token_url = Url::parse(
        format!(
            "https://login.microsoftonline.com/{}/oauth2/token",
            config.tenant_domain
        )
        .as_str(),
    )?;
    let redirect_url = Url::parse("https://login.microsoftonline.com/common/oauth2/nativeclient")?;
    //let refresh_token_url = Url::parse(format!("https://login.microsoftonline.com/{}/oauth2/token", config.tenant_domain).as_str())?;

    let mut client = Client::new(&config.client_id, auth_url, token_url);
    client.set_client_secret(&config.client_secret);
    client.set_redirect_url(redirect_url);

    client.add_scope("User.ReadAll");

    let token_result = client
        .exchange_client_credentials()
        .with_client(&reqwest_client)
        .execute::<StandardToken>()
        .await?;

    println!("Token: {:?}", token_result);
    Ok(())
}
