#![cfg(feature = "actix_web")]
use actix_web::ResponseError;
use crate::Error;

impl ResponseError for Error {
}