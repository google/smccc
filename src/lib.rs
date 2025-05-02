// Copyright 2022 the authors.
// This project is dual-licensed under Apache 2.0 and MIT terms.
// See LICENSE-APACHE and LICENSE-MIT for details.

//! Functions for version 1.4 of the Arm SMC Calling Convention and version 1.1 of the Arm Power
//! State Coordination Interface (PSCI) version 1.1, and relevant constants.
//!
//! Note that the PSCI and SMCCC arch calls may be made via either HVC or SMC. You can choose which
//! one to use by passing either [`Hvc`] or [`Smc`] as a type parameter to the relevant function.
//!
//! This crate currently only supports aarch64 and the SMC64 versions of the PSCI calls, in the
//! cases that both SMC32 and SMC64 versions exist.

#![no_std]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(clippy::undocumented_unsafe_blocks)]

pub mod arch;
pub mod error;
pub mod psci;

/// Use a Hypervisor Call (HVC).
#[cfg(any(target_arch = "aarch64", target_arch = "arm"))]
pub struct Hvc;

/// Use a Secure Moniter Call (SMC).
#[cfg(any(target_arch = "aarch64", target_arch = "arm"))]
pub struct Smc;

/// Functions to make an HVC or SMC call.
pub trait Call {
    /// Makes a call using the 32-bit calling convention.
    fn call32(function: u32, args: [u32; 7]) -> [u32; 8];
    /// Makes a call using the 64-bit calling convention.
    fn call64(function: u32, args: [u64; 17]) -> [u64; 18];
}

#[cfg(any(target_arch = "aarch64", target_arch = "arm"))]
impl Call for Hvc {
    fn call32(function: u32, args: [u32; 7]) -> [u32; 8] {
        hvc32(function, args)
    }

    fn call64(function: u32, args: [u64; 17]) -> [u64; 18] {
        #[cfg(not(target_arch = "aarch64"))]
        panic!("HVC64 not supported on 32-bit architecture");
        #[cfg(target_arch = "aarch64")]
        hvc64(function, args)
    }
}

#[cfg(any(target_arch = "aarch64", target_arch = "arm"))]
impl Call for Smc {
    fn call32(function: u32, args: [u32; 7]) -> [u32; 8] {
        smc32(function, args)
    }

    fn call64(function: u32, args: [u64; 17]) -> [u64; 18] {
        #[cfg(not(target_arch = "aarch64"))]
        panic!("SMC64 not supported on 32-bit architecture");
        #[cfg(target_arch = "aarch64")]
        smc64(function, args)
    }
}

/// Makes an HVC32 call to the hypervisor, following the SMC Calling Convention version 1.4.
#[cfg(any(target_arch = "aarch64", target_arch = "arm"))]
#[inline(always)]
pub fn hvc32(function: u32, args: [u32; 7]) -> [u32; 8] {
    // SAFETY: This shouldn't affect our memory, and we follow the calling convention so registers
    // are saved and restored as expected.
    unsafe {
        let mut ret = [0; 8];

        #[cfg(target_arch = "aarch64")]
        core::arch::asm!(
            "hvc #0",
            inout("w0") function => ret[0],
            inout("w1") args[0] => ret[1],
            inout("w2") args[1] => ret[2],
            inout("w3") args[2] => ret[3],
            inout("w4") args[3] => ret[4],
            inout("w5") args[4] => ret[5],
            inout("w6") args[5] => ret[6],
            inout("w7") args[6] => ret[7],
            options(nostack)
        );
        // LLVM uses r6 internally and so we aren't allowed to use it as an input or output here. To
        // work around this we save and restore r6 and copy from/to a temporary register instead.
        #[cfg(target_arch = "arm")]
        core::arch::asm!(
            "mov {tmp}, r6",
            "mov r6, {r6_value}",
            "hvc #0",
            "mov {r6_value}, r6",
            "mov r6, {tmp}",
            r6_value = inout(reg) args[5] => ret[6],
            tmp = out(reg) _,
            inout("r0") function => ret[0],
            inout("r1") args[0] => ret[1],
            inout("r2") args[1] => ret[2],
            inout("r3") args[2] => ret[3],
            inout("r4") args[3] => ret[4],
            inout("r5") args[4] => ret[5],
            inout("r7") args[6] => ret[7],
            options(nostack)
        );

        ret
    }
}

/// Makes an SMC32 call to the firmware, following the SMC Calling Convention version 1.4.
#[cfg(any(target_arch = "aarch64", target_arch = "arm"))]
#[inline(always)]
pub fn smc32(function: u32, args: [u32; 7]) -> [u32; 8] {
    // SAFETY: This shouldn't affect our memory, and we follow the calling convention so registers
    // are saved and restored as expected.
    unsafe {
        let mut ret = [0; 8];

        #[cfg(target_arch = "aarch64")]
        core::arch::asm!(
            "smc #0",
            inout("w0") function => ret[0],
            inout("w1") args[0] => ret[1],
            inout("w2") args[1] => ret[2],
            inout("w3") args[2] => ret[3],
            inout("w4") args[3] => ret[4],
            inout("w5") args[4] => ret[5],
            inout("w6") args[5] => ret[6],
            inout("w7") args[6] => ret[7],
            options(nostack)
        );
        #[cfg(target_arch = "arm")]
        core::arch::asm!(
            "mov {tmp}, r6",
            "mov r6, {r6_value}",
            "smc #0",
            "mov {r6_value}, r6",
            "mov r6, {tmp}",
            r6_value = inout(reg) args[5] => ret[6],
            tmp = out(reg) _,
            inout("r0") function => ret[0],
            inout("r1") args[0] => ret[1],
            inout("r2") args[1] => ret[2],
            inout("r3") args[2] => ret[3],
            inout("r4") args[3] => ret[4],
            inout("r5") args[4] => ret[5],
            inout("r7") args[6] => ret[7],
            options(nostack)
        );

        ret
    }
}

/// Makes an HVC64 call to the hypervisor, following the SMC Calling Convention version 1.4.
#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub fn hvc64(function: u32, args: [u64; 17]) -> [u64; 18] {
    // SAFETY: This shouldn't affect our memory, and we follow the calling convention so registers
    // are saved and restored as expected.
    unsafe {
        let mut ret = [0; 18];

        core::arch::asm!(
            "hvc #0",
            inout("x0") function as u64 => ret[0],
            inout("x1") args[0] => ret[1],
            inout("x2") args[1] => ret[2],
            inout("x3") args[2] => ret[3],
            inout("x4") args[3] => ret[4],
            inout("x5") args[4] => ret[5],
            inout("x6") args[5] => ret[6],
            inout("x7") args[6] => ret[7],
            inout("x8") args[7] => ret[8],
            inout("x9") args[8] => ret[9],
            inout("x10") args[9] => ret[10],
            inout("x11") args[10] => ret[11],
            inout("x12") args[11] => ret[12],
            inout("x13") args[12] => ret[13],
            inout("x14") args[13] => ret[14],
            inout("x15") args[14] => ret[15],
            inout("x16") args[15] => ret[16],
            inout("x17") args[16] => ret[17],
            options(nostack)
        );

        ret
    }
}

/// Makes an SMC64 call to the firmware, following the SMC Calling Convention version 1.4.
#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub fn smc64(function: u32, args: [u64; 17]) -> [u64; 18] {
    // SAFETY: This shouldn't affect our memory, and we follow the calling convention so registers
    // are saved and restored as expected.
    unsafe {
        let mut ret = [0; 18];

        core::arch::asm!(
            "smc #0",
            inout("x0") function as u64 => ret[0],
            inout("x1") args[0] => ret[1],
            inout("x2") args[1] => ret[2],
            inout("x3") args[2] => ret[3],
            inout("x4") args[3] => ret[4],
            inout("x5") args[4] => ret[5],
            inout("x6") args[5] => ret[6],
            inout("x7") args[6] => ret[7],
            inout("x8") args[7] => ret[8],
            inout("x9") args[8] => ret[9],
            inout("x10") args[9] => ret[10],
            inout("x11") args[10] => ret[11],
            inout("x12") args[11] => ret[12],
            inout("x13") args[12] => ret[13],
            inout("x14") args[13] => ret[14],
            inout("x15") args[14] => ret[15],
            inout("x16") args[15] => ret[16],
            inout("x17") args[16] => ret[17],
            options(nostack)
        );

        ret
    }
}
