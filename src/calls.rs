// Copyright 2022 the authors.
// This project is dual-licensed under Apache 2.0 and MIT terms.
// See LICENSE-APACHE and LICENSE-MIT for details.

//! Functions to make PSCI calls.

use crate::error::{success_or_error_32, success_or_error_64, Error};
use crate::smccc::{call32, call64};
use crate::{
    AffinityState, LowestAffinityLevel, MigrateType, PSCI_AFFINITY_INFO_64, PSCI_CPU_OFF,
    PSCI_CPU_ON_64, PSCI_CPU_SUSPEND_64, PSCI_MIGRATE_64, PSCI_MIGRATE_INFO_TYPE,
    PSCI_MIGRATE_INFO_UP_CPU_64, PSCI_SYSTEM_OFF, PSCI_SYSTEM_RESET, PSCI_SYSTEM_RESET2_64,
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

/// Powers up a core.
pub fn cpu_on(target_cpu: u64, entry_point_address: u64, context_id: u64) -> Result<(), Error> {
    success_or_error_64(
        call64(
            PSCI_CPU_ON_64,
            [
                target_cpu,
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

/// Gets the status of an affinity instance.
pub fn affinity_info(
    target_affinity: u64,
    lowest_affinity_level: LowestAffinityLevel,
) -> Result<AffinityState, Error> {
    (call64(
        PSCI_AFFINITY_INFO_64,
        [
            target_affinity,
            lowest_affinity_level as u64,
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
    )[0] as i32)
        .try_into()
}

/// Asks the Trusted OS to migrate its context to a specific core.
pub fn migrate(target_cpu: u64) -> Result<(), Error> {
    success_or_error_64(
        call64(
            PSCI_MIGRATE_64,
            [target_cpu, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        )[0],
    )
}

/// Identifies the levelof multicore support in the Trusted OS.
pub fn migrate_info_type() -> Result<MigrateType, Error> {
    (call32(PSCI_MIGRATE_INFO_TYPE, [0, 0, 0, 0, 0, 0, 0])[0] as i32).try_into()
}

/// Returns the MPIDR value of the current resident core of the Trusted OS.
pub fn migrate_info_up_cpu() -> u64 {
    call64(
        PSCI_MIGRATE_INFO_UP_CPU_64,
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    )[0]
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
