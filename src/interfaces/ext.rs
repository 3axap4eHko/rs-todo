use actix_utils::future::{Ready, ready};
use actix_web::{
    FromRequest, HttpRequest, dev::Payload, error::ErrorBadRequest,
};
use uuid::Uuid;

use std::sync::{Mutex, MutexGuard};

pub trait LockExt<T> {
    fn lock_actix(&self) -> Result<MutexGuard<T>, actix_web::Error>;
}

impl<T> LockExt<T> for Mutex<T> {
    fn lock_actix(&self) -> Result<MutexGuard<T>, actix_web::Error> {
        self.lock().map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!("Service lock error: {}", e))
        })
    }
}

pub struct UuidParam(pub Uuid);

impl FromRequest for UuidParam {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        match req.match_info().get("id") {
            Some(s) => match Uuid::parse_str(s) {
                Ok(uuid) => ready(Ok(UuidParam(uuid))),
                Err(_) => ready(Err(ErrorBadRequest("Invalid UUID"))),
            },
            None => ready(Err(ErrorBadRequest("Missing UUID parameter"))),
        }
    }
}
