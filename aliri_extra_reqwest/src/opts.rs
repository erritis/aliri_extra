use clap::{Parser, ValueEnum};

use aliri::jwt;
use aliri_tokens::{
    ClientId,
    ClientSecret
};

#[derive(Clone, Debug, PartialEq, Eq, ValueEnum)]
pub enum ContentTypeEnum {
    /// json content type
    Json,
    /// form content type
    Form
}


#[derive(Debug, Parser)]
pub struct AuthClientOpts {
    /// The issuing authority's token request URL
    #[clap(env)]
    pub token_url: reqwest::Url,

    /// The client ID of the client
    #[clap(env)]
    pub client_id: ClientId,

    /// The client secret used to identify the client to the issuing authority
    #[clap(env, hide_env_values = true)]
    pub client_secret: ClientSecret,

    /// The audience to request a token for
    #[clap(env)]
    pub audience: jwt::Audience,

    /// The local file used to cache credentials
    #[clap(
        env,
        name = "FILE",
        default_value = ".credentials.json"
    )]
    pub credentials_file: std::path::PathBuf,

    /// The lifetime of the access token
    #[clap(env, default_value = "300")]
    pub access_token_lifetime: u64,

    
    /// The content type when requesting the token
    #[clap(env, value_enum, default_value = "json")]
    pub reqwest_content_type: ContentTypeEnum,
}

#[derive(Clone, Debug, Parser)]
pub struct AuthOpts {
    /// host url
    #[clap(env)]
    pub host_url: String,
    /// Issuer
    #[clap(env)]
    pub realm: String,
}