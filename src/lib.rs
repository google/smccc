// Copyright 2022 the authors.
// This project is dual-licensed under Apache 2.0 and MIT terms.
// See LICENSE-APACHE and LICENSE-MIT for details.

//! PSCI calls.

#![no_std]

#[cfg(any(feature = "hvc", feature = "smc"))]
mod calls;
pub mod error;
mod smccc;

#[cfg(any(feature = "hvc", feature = "smc"))]
pub use calls::{
    affinity_info, cpu_default_suspend, cpu_freeze, cpu_off, cpu_on, cpu_suspend, mem_protect,
    mem_protect_check_range, migrate, migrate_info_type, migrate_info_up_cpu, node_hw_state,
    psci_features, set_suspend_mode, stat_count, stat_residency, system_off, system_reset,
    system_reset2, system_suspend, version,
};
use error::Error;

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

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum LowestAffinityLevel {
    All = 0,
    Aff0Ignored = 1,
    Aff0Aff1Ignored = 2,
    Aff0Aff1Aff2Ignored = 3,
}

impl From<LowestAffinityLevel> for u64 {
    fn from(lowest_affinity_level: LowestAffinityLevel) -> u64 {
        (lowest_affinity_level as u32).into()
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AffinityState {
    On = 0,
    Off = 1,
    OnPending = 2,
}

impl TryFrom<i32> for AffinityState {
    type Error = Error;

    fn try_from(value: i32) -> Result<Self, Error> {
        match value {
            0 => Ok(Self::On),
            1 => Ok(Self::Off),
            2 => Ok(Self::OnPending),
            _ => Err(value.into()),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MigrateType {
    MigrateCapable = 0,
    NotMigrateCapable = 1,
    MigrationNotRequired = 2,
}

impl TryFrom<i32> for MigrateType {
    type Error = Error;

    fn try_from(value: i32) -> Result<Self, Error> {
        match value {
            0 => Ok(Self::MigrateCapable),
            1 => Ok(Self::NotMigrateCapable),
            2 => Ok(Self::MigrationNotRequired),
            _ => Err(value.into()),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PowerState {
    HwOn = 0,
    HwOff = 1,
    HwStandby = 2,
}

impl TryFrom<i32> for PowerState {
    type Error = Error;

    fn try_from(value: i32) -> Result<Self, Error> {
        match value {
            0 => Ok(Self::HwOn),
            1 => Ok(Self::HwOff),
            2 => Ok(Self::HwStandby),
            _ => Err(value.into()),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SuspendMode {
    PlatformCoordinated = 0,
    OsInitiated = 1,
}

impl From<SuspendMode> for u32 {
    fn from(suspend_mode: SuspendMode) -> u32 {
        suspend_mode as u32
    }
}
