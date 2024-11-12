// Copyright 2022 the authors.
// This project is dual-licensed under Apache 2.0 and MIT terms.
// See LICENSE-APACHE and LICENSE-MIT for details.

//! PSCI error codes.

pub use crate::error::SUCCESS;

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
#[derive(Copy, Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
    /// PSCI call not supported.
    #[error("PSCI call not supported")]
    NotSupported,
    /// Invalid parameters to PSCI call.
    #[error("Invalid parameters to PSCI call")]
    InvalidParameters,
    /// PSCI call denied.
    #[error("PSCI call denied")]
    Denied,
    /// Core already on.
    #[error("Core already on")]
    AlreadyOn,
    /// Core already being turned on.
    #[error("Core already being turned on")]
    OnPending,
    /// Internal failure in PSCI call.
    #[error("Internal failure in PSCI call")]
    InternalFailure,
    /// Trusted OS not present on target core.
    #[error("Trusted OS not present on target core")]
    NotPresent,
    /// Core disabled.
    #[error("Core disabled")]
    Disabled,
    /// Invalid address passed to PSCI call.
    #[error("Invalid address passed to PSCI call")]
    InvalidAddress,
    /// An unexpected return value from a PSCI function.
    #[error("Unknown PSCI return value {0} ({0:#x})")]
    Unknown(i64),
}

impl From<Error> for i64 {
    fn from(error: Error) -> i64 {
        match error {
            Error::NotSupported => NOT_SUPPORTED.into(),
            Error::InvalidParameters => INVALID_PARAMETERS.into(),
            Error::Denied => DENIED.into(),
            Error::AlreadyOn => ALREADY_ON.into(),
            Error::OnPending => ON_PENDING.into(),
            Error::InternalFailure => INTERNAL_FAILURE.into(),
            Error::NotPresent => NOT_PRESENT.into(),
            Error::Disabled => DISABLED.into(),
            Error::InvalidAddress => INVALID_ADDRESS.into(),
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
            _ => Error::Unknown(value.into()),
        }
    }
}

impl From<i64> for Error {
    fn from(value: i64) -> Self {
        if let Ok(value) = i32::try_from(value) {
            value.into()
        } else {
            Error::Unknown(value)
        }
    }
}
