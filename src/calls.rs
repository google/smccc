// Copyright 2022 the authors.
// This project is dual-licensed under Apache 2.0 and MIT terms.
// See LICENSE-APACHE and LICENSE-MIT for details.

//! Functions to make PSCI calls.

use crate::error::{success_or_error_32, success_or_error_64, Error};
use crate::smccc::{call32, call64};
use crate::{
    PSCI_CPU_OFF, PSCI_CPU_SUSPEND_64, PSCI_SYSTEM_OFF, PSCI_SYSTEM_RESET, PSCI_SYSTEM_RESET2_64,
    PSCI_VERSION,
};

/// Returns the version of PSCI implemented.
pub fn version() -> u32 {
    call32(PSCI_VERSION, [0, 0, 0, 0, 0, 0, 0])[0]
}

/// Suspends execution of a core or topology node.
pub fn cpu_suspend(
    power_state: u32,
    entry_point_address: u64,
    context_id: u64,
) -> Result<(), Error> {
    success_or_error_64(
        call64(
            PSCI_CPU_SUSPEND_64,
            [
                power_state.into(),
                entry_point_address,
                context_id,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
        )[0],
    )
}

/// Powers down the current core.
pub fn cpu_off() -> Result<(), Error> {
    success_or_error_32(call32(PSCI_CPU_OFF, [0, 0, 0, 0, 0, 0, 0])[0])
}

/// Shuts down the system.
pub fn system_off() -> Result<(), Error> {
    success_or_error_32(call32(PSCI_SYSTEM_OFF, [0, 0, 0, 0, 0, 0, 0])[0])
}

/// Resets the system.
pub fn system_reset() -> Result<(), Error> {
    success_or_error_32(call32(PSCI_SYSTEM_RESET, [0, 0, 0, 0, 0, 0, 0])[0])
}

/// Resets the system in an architectural or vendor-specific way.
pub fn system_reset2(reset_type: u32, cookie: u64) -> Result<(), Error> {
    success_or_error_64(
        call64(
            PSCI_SYSTEM_RESET2_64,
            [
                reset_type.into(),
                cookie,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
        )[0],
    )
}
