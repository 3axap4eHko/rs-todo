// interfaces/extractors.rs
use actix_web::{FromRequest, HttpRequest, dev::Payload, error::ErrorBadRequest};
use futures::future::{Ready, ready};
use std::str::FromStr;

/// Blanket implementation for any new‑type that implements `FromStr<Err = IdError>`
#[cfg(feature = "actix")]
macro_rules! impl_id_extractor {
    ($id:ty) => {
        impl FromRequest for $id {
            type Error = actix_web::Error;
            type Future = Ready<Result<Self, Self::Error>>;

            fn from_request(req: &HttpRequest, _pl: &mut Payload) -> Self::Future {
                ready(
                    req.match_info()
                       .query("id")
                       .parse()
                       .map_err(|e| ErrorBadRequest(json!({ "code": "INVALID_ID", "message": e.to_string() })))
                )
            }
        }
    };
}

impl_id_extractor!(TodoId);

