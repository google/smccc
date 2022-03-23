// Copyright 2022 the authors.
// This project is dual-licensed under Apache 2.0 and MIT terms.
// See LICENSE-APACHE and LICENSE-MIT for details.

//! PSCI calls.

#![no_std]

mod error;
mod smccc;

use error::{success_or_error_32, success_or_error_64, Error};
#[cfg(any(feature = "hvc", feature = "smc"))]
use smccc::{call32, call64};

pub const PSCI_VERSION: u32 = 0x84000000;
pub const PSCI_CPU_SUSPEND_32: u32 = 0x84000001;
pub const PSCI_CPU_SUSPEND_64: u32 = 0xC4000001;
pub const PSCI_CPU_OFF: u32 = 0x84000002;
pub const PSCI_CPU_ON_32: u32 = 0x84000003;
pub const PSCI_CPU_ON_64: u32 = 0xC4000003;
pub const PSCI_AFFINITY_INFO_32: u32 = 0x84000004;
pub const PSCI_AFFINITY_INFO_64: u32 = 0xC4000004;
pub const PSCI_MIGRATE_32: u32 = 0x84000005;
pub const PSCI_MIGRATE_64: u32 = 0xC4000005;
pub const PSCI_MIGRATE_INFO_TYPE: u32 = 0x84000006;
pub const PSCI_MIGRATE_INFO_UP_CPU_32: u32 = 0x84000007;
pub const PSCI_MIGRATE_INFO_UP_CPU_64: u32 = 0xC4000007;
pub const PSCI_SYSTEM_OFF: u32 = 0x84000008;
pub const PSCI_SYSTEM_RESET: u32 = 0x84000009;
pub const PSCI_SYSTEM_RESET2_32: u32 = 0x84000012;
pub const PSCI_SYSTEM_RESET2_64: u32 = 0xC4000012;
pub const PSCI_MEM_PROTECT: u32 = 0x84000013;
pub const PSCI_MEM_PROTECT_CHECK_RANGE_32: u32 = 0x84000014;
pub const PSCI_MEM_PROTECT_CHECK_RANGE_64: u32 = 0xC4000014;
pub const PSCI_FEATURES: u32 = 0x8400000A;
pub const PSCI_CPU_FREEZE: u32 = 0x8400000B;
pub const PSCI_CPU_DEFAULT_SUSPEND_32: u32 = 0x8400000C;
pub const PSCI_CPU_DEFAULT_SUSPEND_64: u32 = 0xC400000C;
pub const PSCI_NODE_HW_STATE_32: u32 = 0x8400000D;
pub const PSCI_NODE_HW_STATE_64: u32 = 0xC400000D;
pub const PSCI_SYSTEM_SUSPEND_32: u32 = 0x8400000E;
pub const PSCI_SYSTEM_SUSPEND_64: u32 = 0xC400000E;
pub const PSCI_SET_SUSPEND_MODE: u32 = 0x8400000F;
pub const PSCI_STAT_RESIDENCY_32: u32 = 0x84000010;
pub const PSCI_STAT_RESIDENCY_64: u32 = 0xC4000010;
pub const PSCI_STAT_COUNT_32: u32 = 0x84000011;
pub const PSCI_STAT_COUNT_64: u32 = 0xC4000011;

/// Returns the version of PSCI implemented.
#[cfg(any(feature = "hvc", feature = "smc"))]
pub fn version() -> u32 {
    call32(PSCI_VERSION, [0, 0, 0, 0, 0, 0, 0])[0]
}

/// Suspends execution of a core or topology node.
#[cfg(any(feature = "hvc", feature = "smc"))]
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
#[cfg(any(feature = "hvc", feature = "smc"))]
pub fn cpu_off() -> Result<(), Error> {
    success_or_error_32(call32(PSCI_CPU_OFF, [0, 0, 0, 0, 0, 0, 0])[0])
}

/// Shuts down the system.
#[cfg(any(feature = "hvc", feature = "smc"))]
pub fn system_off() -> Result<(), Error> {
    success_or_error_32(call32(PSCI_SYSTEM_OFF, [0, 0, 0, 0, 0, 0, 0])[0])
}

/// Resets the system.
#[cfg(any(feature = "hvc", feature = "smc"))]
pub fn system_reset() -> Result<(), Error> {
    success_or_error_32(call32(PSCI_SYSTEM_RESET, [0, 0, 0, 0, 0, 0, 0])[0])
}

/// Resets the system in an architectural or vendor-specific way.
#[cfg(any(feature = "hvc", feature = "smc"))]
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
