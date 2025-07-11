#![no_std]
extern crate alloc;

use alloc::boxed::Box;
use core::{
    any::type_name,
    error::Error as ErrorTrait,
    fmt::{Debug, Display},
};

pub type Result<T> = core::result::Result<T, Error>;

pub struct Error {
    name: &'static str,
    location: Location,
    cause: Option<Box<Error>>,
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
        writeln!(f, "{} in {:?}", self.name, self.location)?;
        if let Some(cause) = &self.cause {
            write!(f, "Caused by: {cause}")?;
        }
        Ok(())
    }
}

impl<E: ErrorTrait> From<E> for Error {
    #[track_caller]
    fn from(error: E) -> Self {
        //println!("Fetching location in From ErrorTrait");
        Self::new(
            type_name::<E>(),
            core::panic::Location::caller(),
            error.source().map(|inner| Box::new(Error::from(inner))),
        )
    }
}

impl Error {
    pub fn new(
        name: &'static str,
        location: &'static core::panic::Location,
        cause: Option<Box<Error>>,
    ) -> Self {
        Self {
            name,
            location: Location::new(location),
            cause,
        }
    }
}

pub trait ErrorExt<T> {
    fn loc(self) -> Result<T>;
    fn loc_msg(self, msg: &'static str) -> Result<T>;
}

pub trait ErrorExtTrait<T> {
    fn loc(self) -> Result<T>;
    fn loc_msg(self, msg: &'static str) -> Result<T>;
}

pub trait ErrorExtCompat<T> {
    fn loc_compat(self) -> Result<T>;
    fn loc_compat_msg(self, msg: &'static str) -> Result<T>;
}

impl<T> ErrorExt<T> for Option<T> {
    #[track_caller]
    fn loc(self) -> Result<T> {
        match self {
            Some(t) => Ok(t),
            None => Err(Error::new(
                type_name::<Option<T>>(),
                core::panic::Location::caller(),
                None,
            )),
        }
    }

    #[track_caller]
    fn loc_msg(self, msg: &'static str) -> Result<T> {
        match self {
            Some(t) => Ok(t),
            None => Err(Error::new(
                msg,
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
                core::panic::Location::caller(),
                Some(Box::new(err)),
            )),
        }
    }

    #[track_caller]
    fn loc_msg(self, msg: &'static str) -> Result<T> {
        match self {
            Ok(ok) => Ok(ok),
            Err(err) => Err(Error::new(
                msg,
                core::panic::Location::caller(),
                Some(Box::new(err)),
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
        //println!("Fetching location in ErrorExtTrait");
        match self {
            Ok(ok) => Ok(ok),
            Err(err) => Err(Error::new(
                type_name::<E>(),
                core::panic::Location::caller(),
                Some(Box::new(Error::from(err))),
            )),
        }
    }

    #[track_caller]
    fn loc_msg(self, msg: &'static str) -> Result<T> {
        //println!("Fetching location in ErrorExtTrait");
        match self {
            Ok(ok) => Ok(ok),
            Err(err) => Err(Error::new(
                msg,
                core::panic::Location::caller(),
                Some(Box::new(Error::from(err))),
            )),
        }
    }
}

impl<T, E> ErrorExtCompat<T> for core::result::Result<T, E> {
    #[track_caller]
    fn loc_compat(self) -> Result<T> {
        //println!("Fetching location in ErrorExtCompat");
        match self {
            Ok(ok) => Ok(ok),
            Err(_) => Err(Error::new(
                type_name::<E>(),
                core::panic::Location::caller(),
                None,
            )),
        }
    }

    #[track_caller]
    fn loc_compat_msg(self, msg: &'static str) -> Result<T> {
        //println!("Fetching location in ErrorExtCompat");
        match self {
            Ok(ok) => Ok(ok),
            Err(_) => Err(Error::new(
                msg,
                core::panic::Location::caller(),
                None,
            )),
        }
    }
}
