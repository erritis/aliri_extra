/*!
aliri_claims is a crate with ready-made claims structures for deserialization and subsequent use in the project.

# installation

Install using cargo:

```no_run,ignore
cargo install aliri_claims
```

# Usage

## Actix Web

```no_run
use actix_web::{middleware::Logger, App, HttpResponse, HttpServer, web, get};
use actix_cors::Cors;

use aliri::jwt;
use aliri_oauth2::Authority;
use aliri_actix::scope_policy;
use aliri_claims::AdvancedClaims;

scope_policy!(Profile / ProfileScope(AdvancedClaims); "profile");


#[get("/userinfo")]
pub async fn userinfo(user: Profile) -> HttpResponse {
    let issuer = user.claims().issuer();
    let sub = user.claims().subject();
    let picture = user.claims().picture();
    HttpResponse::Ok().json(user.claims())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    let issuer = std::env::var("ISSUER").expect("ISSUER must be set");

    let validator = jwt::CoreValidator::default()
            //.require_issuer(jwt::Issuer::new(issuer.clone()))
            //.add_allowed_audience(jwt::Audience::new(issuer.clone()))
            ;
    // To get jwks_uri look in {issuer}/.well-known/openid-configuration
    let jwks_url = format!("{}/protocol/openid-connect/certs", issuer);

    let authority = Authority::new_from_url(jwks_url, validator)
    .await.expect("AUTHORITY didn't create");


    HttpServer::new(move || {
        let cors = Cors::default().allow_any_origin().allow_any_header().allow_any_method().supports_credentials();

        App::new()
           
            .wrap(Logger::default())
            .app_data(authority.clone())
            .wrap(cors)
            .service(userinfo)
            
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
```
*/
#![warn(missing_docs)]

use std::ops::Deref;

use aliri::jwt;
use aliri_oauth2::oauth2 as scope;
use aliri_clock::UnixTime;

use openidconnect::{core::CoreGenderClaim, IdTokenClaims, EmptyAdditionalClaims};
use serde::{Serialize, Deserialize};

/// Extended claims structure from openidconnect
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AdvancedClaims {
    /// Structure from openidconnect
    #[serde(flatten)]
    pub basic: IdTokenClaims<EmptyAdditionalClaims, CoreGenderClaim>,
    /// Issuer
    pub iss: jwt::Issuer,
    /// Audiences
    pub aud: jwt::Audiences,
    /// Subject
    pub sub: jwt::Subject,
    /// (Expiration time) claim
    pub exp: Option<UnixTime>,
    /// (Not Before) Claim The "nbf" claim identifies the time before which the JWT MUST NOT be accepted for processing.
    pub nbf: Option<UnixTime>,
    /// List of all scopes
    pub scope: scope::Scope,
}


impl Deref for AdvancedClaims {
    type Target = IdTokenClaims<EmptyAdditionalClaims, CoreGenderClaim>;

    fn deref(&self) -> &Self::Target {
        &self.basic
    }
}

impl jwt::CoreClaims for AdvancedClaims {
    fn nbf(&self) -> Option<UnixTime> { self.nbf }
    fn exp(&self) -> Option<UnixTime> { self.exp }
    fn aud(&self) -> &jwt::Audiences { &self.aud }
    fn iss(&self) -> Option<&jwt::IssuerRef> { Some(&self.iss) }
    fn sub(&self) -> Option<&jwt::SubjectRef> { Some(&self.sub) }
}

impl scope::HasScope for AdvancedClaims {
    fn scope(&self) -> &scope::Scope { &self.scope }
}