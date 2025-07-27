use std::fs::File;

use lokerro::{ErrorExt as _, ErrorExtCompat as _, ErrorExtTrait as _, Result as LokerroResult};

#[test]
#[cfg_attr(
    target_family = "windows",
    should_panic = "called `Result::unwrap()` on an `Err` value: lokerro::Error in tests\\try.rs:15:23\nCaused by: try::BadError in tests\\try.rs:105:23\n"
)]
#[cfg_attr(
    target_family = "unix",
    should_panic = "called `Result::unwrap()` on an `Err` value: lokerro::Error in tests/try.rs:15:23\nCaused by: try::BadError in tests/try.rs:105:23\n"
)]
fn unwrap_chained_bad_error() {
    chain_bad_error().loc().unwrap()
}

#[test]
#[cfg_attr(
    target_family = "windows",
    should_panic = "called `Result::unwrap()` on an `Err` value: lokerro::Error in tests\\try.rs:28:27\nCaused by: try::BadError -> `This is a bad error` in tests\\try.rs:110:23\n"
)]
#[cfg_attr(
    target_family = "unix",
    should_panic = "called `Result::unwrap()` on an `Err` value: lokerro::Error in tests/try.rs:28:27\nCaused by: try::BadError -> `This is a bad error` in tests/try.rs:110:23\n"
)]
fn unwrap_chained_bad_error_msg() {
    chain_bad_error_msg().loc().unwrap()
}

#[test]
#[cfg_attr(
    target_family = "windows",
    should_panic = "called `Result::unwrap()` on an `Err` value: lokerro::Error in tests\\try.rs:41:19\nCaused by: std::io::error::Error -> `Os { code: 2, kind: NotFound, message: \"No such file or directory\" }` in tests\\try.rs:95:5\n"
)]
#[cfg_attr(
    target_family = "unix",
    should_panic = "called `Result::unwrap()` on an `Err` value: lokerro::Error in tests/try.rs:41:19\nCaused by: std::io::error::Error -> `Os { code: 2, kind: NotFound, message: \"No such file or directory\" }` in tests/try.rs:95:5\n"
)]
fn unwrap_chained_error() {
    chain_error().loc().unwrap()
}

#[test]
#[cfg_attr(
    target_family = "windows",
    should_panic = "called `Result::unwrap()` on an `Err` value: lokerro::Error in tests\\try.rs:54:23\nCaused by: std::io::error::Error -> `This is a good error` in tests\\try.rs:100:19\nCaused by: std::io::error::Error -> `Os { code: 2, kind: NotFound, message: \"No such file or directory\" }` in tests\\try.rs:100:19\n"
)]
#[cfg_attr(
    target_family = "unix",
    should_panic = "called `Result::unwrap()` on an `Err` value: lokerro::Error in tests/try.rs:54:23\nCaused by: std::io::error::Error -> `This is a good error` in tests/try.rs:100:19\nCaused by: std::io::error::Error -> `Os { code: 2, kind: NotFound, message: \"No such file or directory\" }` in tests/try.rs:100:19\n"
)]
fn unwrap_chained_error_msg() {
    chain_error_msg().loc().unwrap()
}

#[test]
#[cfg_attr(
    target_family = "windows",
    should_panic = "called `Result::unwrap()` on an `Err` value: lokerro::Error in tests\\try.rs:67:27\nCaused by: lokerro::Error in tests\\try.rs:85:19\nCaused by: std::io::error::Error -> `Os { code: 2, kind: NotFound, message: \"No such file or directory\" }` in tests\\try.rs:95:5\n"
)]
#[cfg_attr(
    target_family = "unix",
    should_panic = "called `Result::unwrap()` on an `Err` value: lokerro::Error in tests/try.rs:67:27\nCaused by: lokerro::Error in tests/try.rs:85:19\nCaused by: std::io::error::Error -> `Os { code: 2, kind: NotFound, message: \"No such file or directory\" }` in tests/try.rs:95:5\n"
)]
fn unwrap_chained_lokerro_error() {
    chain_lokerro_error().loc().unwrap()
}

#[test]
#[cfg_attr(
    target_family = "windows",
    should_panic = "called `Result::unwrap()` on an `Err` value: lokerro::Error in tests\\try.rs:80:31\nCaused by: lokerro::Error -> `This is a lokerro error` in tests\\try.rs:90:19\nCaused by: std::io::error::Error -> `Os { code: 2, kind: NotFound, message: \"No such file or directory\" }` in tests\\try.rs:95:5\n"
)]
#[cfg_attr(
    target_family = "unix",
    should_panic = "called `Result::unwrap()` on an `Err` value: lokerro::Error in tests/try.rs:80:31\nCaused by: lokerro::Error -> `This is a lokerro error` in tests/try.rs:90:19\nCaused by: std::io::error::Error -> `Os { code: 2, kind: NotFound, message: \"No such file or directory\" }` in tests/try.rs:95:5\n"
)]
fn unwrap_chained_lokerro_error_msg() {
    chain_lokerro_error_msg().loc().unwrap()
}


fn chain_lokerro_error() -> LokerroResult<()> {
    chain_error().loc()?;
    Ok(())
}

fn chain_lokerro_error_msg() -> LokerroResult<()> {
    chain_error().loc_msg("This is a lokerro error")?;
    Ok(())
}

fn chain_error() -> LokerroResult<()>{
    cause_error()?;
    Ok(())
}

fn chain_error_msg() -> LokerroResult<()>{
    cause_error().loc_msg("This is a good error")?;
    Ok(())
}

fn chain_bad_error() -> LokerroResult<()> {
    cause_bad_error().loc_compat()?;
    Ok(())
}

fn chain_bad_error_msg() -> LokerroResult<()> {
    cause_bad_error().loc_compat_msg("This is a bad error")?;
    Ok(())
}

enum BadError {
    CommonMistake
}

fn cause_bad_error() -> Result<(), BadError> {
    Err(BadError::CommonMistake)
}

fn cause_error() -> Result<(), std::io::Error> {
    File::open("does not exist and if it does you're doing something wrong")?;
    Ok(())
}