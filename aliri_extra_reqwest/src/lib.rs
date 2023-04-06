mod opts;

use aliri_clock::DurationSecs;
use aliri_reqwest::AccessTokenMiddleware;
use aliri_tokens::{sources, TokenLifetimeConfig, TokenWatcher, jitter::NullJitter, backoff::ErrorBackoffConfig};

use clap::Parser;
use opts::{AuthClientOpts, AuthOpts, ContentTypeEnum};

use reqwest::Client;
use reqwest_middleware::{ClientWithMiddleware, ClientBuilder};
use predicates::prelude::predicate;



impl From<ContentTypeEnum> for sources::oauth2::ContentType {
    fn from(content_type: ContentTypeEnum) -> Self {
        match content_type {
            ContentTypeEnum::Json => sources::oauth2::ContentType::Json,
            ContentTypeEnum::Form => sources::oauth2::ContentType::Form
        }
    }
}



pub async fn auth_client() -> ClientWithMiddleware {

    let opts = AuthClientOpts::parse();

    let client = reqwest::Client::builder()
    .https_only(false)
    .build()
    .unwrap();

    let credentials = sources::oauth2::dto::ClientCredentialsWithAudience {
        credentials: sources::oauth2::dto::ClientCredentials {
            client_id: opts.client_id,
            client_secret: opts.client_secret,
        }
        .into(),
        audience: opts.audience,
    };

    let fallback = sources::oauth2::ClientCredentialsTokenSource::new(
        client,
        opts.token_url,
        credentials,
        TokenLifetimeConfig::new(0.75, DurationSecs(opts.access_token_lifetime)),
        opts.reqwest_content_type.into()
    );

    let file_source = sources::file::FileTokenSource::new(opts.credentials_file);

    let token_source = sources::cache::CachedTokenSource::new(fallback)
    .with_cache("file", file_source)
    ;

    let token_watcher = TokenWatcher::spawn_from_token_source(
        token_source,
        NullJitter,
        ErrorBackoffConfig::default()
    ).await.unwrap();

    
    let client = ClientBuilder::new(Client::default())
    .with(AccessTokenMiddleware::new(token_watcher).with_predicate(predicate::always()))
    .build();

    client
}

#[derive(Clone, Debug)]
pub struct AuthClient {
    pub client: ClientWithMiddleware,
    pub host_url: String,
    pub realm: String
}


pub async fn auth_client_with_sso_info() -> AuthClient {

    let opts = AuthOpts::parse();

    let client = auth_client().await;

    AuthClient {
        client,
        host_url: opts.host_url,
        realm: opts.realm
    }
}