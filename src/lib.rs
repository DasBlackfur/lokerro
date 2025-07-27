#![no_std]
//! Lokerro aims to provide simple error handling, comparable to `anyhow`.
//!
//! It however has a few advantages:
//! - Relatively inexpensive location information compared to full backtraces
//! - Compatibility with bad error types (those who do not implement Error)
//! - Compatibility with complex error types (those who have hundres of lifetimes in their type specification)
//!
//! It also comes with a disadvantage you should consider if you are making a library:
//! - The error type cannot be used in other error handling code, meaning you will be locked into using lokerro for all downstream handling
//!
//! Here is an example how lokerro might be used in an application:
//! ```rust,should_panic
//! use std::{collections::HashMap, fs::File, io::Read};
//! use lokerro::{ErrorExt, Result};
//!
//! fn main() -> Result<()> {
//!     let result = load_settings().loc_msg("Loading settings failed")?;
//!
//!     println!("{result:?}");
//!
//!     Ok(())
//! }
//!
//! fn load_settings() -> Result<HashMap<u32, String>> {
//!     let mut config = HashMap::new();
//!     let config_1 = process_file("this_file_may_not_exist.txt").loc()?;
//!     let config_2 = process_file("this_file_neither.txt").loc()?;
//!
//!     config.insert(1, config_1);
//!     config.insert(2, config_2);
//!
//!     Ok(config)
//! }
//!
//! fn process_file(path: &str) -> Result<String> {
//!     let mut file = File::open(path)?;
//!     let mut string = String::new();
//!
//!     file.read_to_string(&mut string)?;
//!     Ok(string)
//! }
//! ```
//!
//! This example will exit with code 1 and print out the following:
//! ```text
//! Error: Loading settings failed in examples\realistic.rs:5:34
//! Caused by: lokerro::Error in examples\realistic.rs:14:64
//! Caused by: std::io::error::Error in examples\realistic.rs:24:20
//! ```

mod actix_web;

extern crate alloc;

use alloc::borrow::ToOwned;
use alloc::boxed::Box;
use alloc::format;
use alloc::string::{String};
use core::{
    any::type_name,
    error::Error as ErrorTrait,
    fmt::{Debug, Display}, ops::Deref,
};

pub type Result<T> = core::result::Result<T, Error>;

pub struct Error(Box<ErrorImpl>);

impl Deref for Error {
    type Target = ErrorImpl;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct ErrorImpl {
    name: &'static str,
    message: Option<String>,
    location: Location,
    cause: Option<Error>,
}

pub struct Location {
    file: &'static str,
    line: u32,
    col: u32,
}

impl Location {
    pub fn new(caller: &'static core::panic::Location<'static>) -> Self {
        Self {
            file: caller.file(),
            line: caller.line(),
            col: caller.column(),
        }
    }
}

impl Debug for Location {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}:{}:{}", self.file, self.line, self.col)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <Self as Debug>::fmt(self, f)
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match &self.message {
            None => writeln!(f, "{} in {:?}", self.name, self.location)?,
            Some(msg) => writeln!(f, "{} -> `{}` in {:?}", self.name, msg, self.location)?,
        }
        if let Some(cause) = &self.cause {
            write!(f, "Caused by: {cause}")?;
        }
        Ok(())
    }
}

impl<E: ErrorTrait> From<E> for Error {
    #[track_caller]
    fn from(error: E) -> Self {
        Self::new(
            type_name::<E>(),
            Some(format!("{error:?}")),
            core::panic::Location::caller(),
            error.source().map(Error::from),
        )
    }
}

impl Error {
    pub fn new(
        name: &'static str,
        msg: Option<String>,
        location: &'static core::panic::Location,
        cause: Option<Error>,
    ) -> Self {
        Error(Box::new(ErrorImpl {
            name,
            message: msg,
            location: Location::new(location),
            cause,
        }))
    }
}

pub trait ErrorExt<T> {
    fn loc(self) -> Result<T>;
    fn loc_msg(self, msg: &str) -> Result<T>;
}

pub trait ErrorExtTrait<T> {
    fn loc(self) -> Result<T>;
    fn loc_msg(self, msg: &str) -> Result<T>;
}

pub trait ErrorExtCompat<T> {
    fn loc_compat(self) -> Result<T>;
    fn loc_compat_msg(self, msg: &str) -> Result<T>;
}

impl<T> ErrorExt<T> for Option<T> {
    #[track_caller]
    fn loc(self) -> Result<T> {
        match self {
            Some(t) => Ok(t),
            None => Err(Error::new(
                type_name::<Option<T>>(),
                None,
                core::panic::Location::caller(),
                None,
            )),
        }
    }

    #[track_caller]
    fn loc_msg(self, msg: &str) -> Result<T> {
        match self {
            Some(t) => Ok(t),
            None => Err(Error::new(
                type_name::<Option<T>>(),
                Some(msg.to_owned()),
                core::panic::Location::caller(),
                None,
            )),
        }
    }
}

impl<T> ErrorExt<T> for Result<T> {
    #[track_caller]
    fn loc(self) -> Result<T> {
        match self {
            Ok(ok) => Ok(ok),
            Err(err) => Err(Error::new(
                type_name::<Error>(),
                None,
                core::panic::Location::caller(),
                Some(err),
            )),
        }
    }

    #[track_caller]
    fn loc_msg(self, msg: &str) -> Result<T> {
        match self {
            Ok(ok) => Ok(ok),
            Err(err) => Err(Error::new(
                type_name::<Error>(),
                Some(msg.to_owned()),
                core::panic::Location::caller(),
                Some(err),
            )),
        }
    }
}

impl<T, E> ErrorExtTrait<T> for core::result::Result<T, E>
where
    E: ErrorTrait,
{
    #[track_caller]
    fn loc(self) -> Result<T> {
        match self {
            Ok(ok) => Ok(ok),
            Err(err) => Err(Error::new(
                type_name::<E>(),
                None,
                core::panic::Location::caller(),
                Some(Error::from(err)),
            )),
        }
    }

    #[track_caller]
    fn loc_msg(self, msg: &str) -> Result<T> {
        match self {
            Ok(ok) => Ok(ok),
            Err(err) => Err(Error::new(
                type_name::<E>(),
                Some(msg.to_owned()),
                core::panic::Location::caller(),
                Some(Error::from(err)),
            )),
        }
    }
}

impl<T, E> ErrorExtCompat<T> for core::result::Result<T, E> {
    #[track_caller]
    fn loc_compat(self) -> Result<T> {
        match self {
            Ok(ok) => Ok(ok),
            Err(_) => Err(Error::new(
                type_name::<E>(),
                None,
                core::panic::Location::caller(),
                None,
            )),
        }
    }

    #[track_caller]
    fn loc_compat_msg(self, msg: &str) -> Result<T> {
        match self {
            Ok(ok) => Ok(ok),
            Err(_) => Err(Error::new(
                type_name::<E>(),
                Some(msg.to_owned()),
                core::panic::Location::caller(),
                None,
            )),
        }
    }
}
