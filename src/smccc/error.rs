// Copyright 2023 the authors.
// This project is dual-licensed under Apache 2.0 and MIT terms.
// See LICENSE-APACHE and LICENSE-MIT for details.

//! Utility functions for error handling.

pub const SUCCESS: i32 = 0;

pub fn success_or_error_32<E: From<i32>>(value: u32) -> Result<(), E> {
    success_or_error(value as i32)
}

pub fn success_or_error_64<E: From<i32>>(value: u64) -> Result<(), E> {
    success_or_error(value as i32)
}

fn success_or_error<E: From<i32>>(value: i32) -> Result<(), E> {
    if value == SUCCESS {
        Ok(())
    } else {
        Err(value.into())
    }
}
