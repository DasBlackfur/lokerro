use std::fs::File;
use actix_web::http::StatusCode;
use lokerro::ErrorExtTrait;
use actix_web::ResponseError;

#[test]
#[cfg(feature = "actix_web")]
fn get_response() {
    assert_eq!(cause_error().loc().unwrap_err().status_code(), StatusCode::INTERNAL_SERVER_ERROR);
}

fn cause_error() -> Result<(), std::io::Error> {
    File::open("does not exist and if it does you're doing something wrong")?;
    Ok(())
}