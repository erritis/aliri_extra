[![crates.io][crates-badge]][crates-url]
[![documentation][docs-badge]][docs-url]
[![MIT License][mit-badge]][mit-url]

[crates-badge]: https://img.shields.io/crates/v/aliri_clamis.svg
[crates-url]: https://crates.io/crates/aliri_clamis
[docs-badge]: https://docs.rs/aliri_clamis/badge.svg
[docs-url]: https://docs.rs/aliri_clamis
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: LICENSE

# nested-env-parser

aliri_claims is a crate with ready-made claims structures for deserialization and subsequent use in the project.

## installation

Install using cargo:

```no_run,ignore
cargo install aliri_claims
```

## Usage

### Actix Web

```rust
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

Current version: 0.1.0

License: MIT
