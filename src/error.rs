// Copyright 2022 the authors.
// This project is dual-licensed under Apache 2.0 and MIT terms.
// See LICENSE-APACHE and LICENSE-MIT for details.

//! PSCI error codes.

pub const SUCCESS: i32 = 0;
pub const NOT_SUPPORTED: i32 = -1;
pub const INVALID_PARAMETERS: i32 = -2;
pub const DENIED: i32 = -3;
pub const ALREADY_ON: i32 = -4;
pub const ON_PENDING: i32 = -5;
pub const INTERNAL_FAILURE: i32 = -6;
pub const NOT_PRESENT: i32 = -7;
pub const DISABLED: i32 = -8;
pub const INVALID_ADDRESS: i32 = -9;

/// Standard PSCI errors.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Error {
    NotSupported,
    InvalidParameters,
    Denied,
    AlreadyOn,
    OnPending,
    InternalFailure,
    NotPresent,
    Disabled,
    InvalidAddress,
    /// An unexpected return value from a PSCI function.
    Unknown(i32),
}

pub(crate) fn success_or_error_32(value: u32) -> Result<(), Error> {
    success_or_error(value as i32)
}

pub(crate) fn success_or_error_64(value: u64) -> Result<(), Error> {
    success_or_error(value as i32)
}

fn success_or_error(value: i32) -> Result<(), Error> {
    if value == SUCCESS {
        Ok(())
    } else {
        Err(value.into())
    }
}

impl From<Error> for i32 {
    fn from(error: Error) -> i32 {
        match error {
            Error::NotSupported => NOT_SUPPORTED,
            Error::InvalidParameters => INVALID_PARAMETERS,
            Error::Denied => DENIED,
            Error::AlreadyOn => ALREADY_ON,
            Error::OnPending => ON_PENDING,
            Error::InternalFailure => INTERNAL_FAILURE,
            Error::NotPresent => NOT_PRESENT,
            Error::Disabled => DISABLED,
            Error::InvalidAddress => INVALID_ADDRESS,
            Error::Unknown(value) => value,
        }
    }
}

impl From<i32> for Error {
    fn from(value: i32) -> Self {
        match value {
            NOT_SUPPORTED => Error::NotSupported,
            INVALID_PARAMETERS => Error::InvalidParameters,
            DENIED => Error::Denied,
            ALREADY_ON => Error::AlreadyOn,
            ON_PENDING => Error::OnPending,
            INTERNAL_FAILURE => Error::InternalFailure,
            NOT_PRESENT => Error::NotPresent,
            DISABLED => Error::Disabled,
            INVALID_ADDRESS => Error::InvalidAddress,
            _ => Error::Unknown(value),
        }
    }
}
