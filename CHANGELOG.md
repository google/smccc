# Changelog

## 0.2.2

### Bugfixes

- Workaround build issue on some aarch32 targets where r7 is reserved as the frame pointer.

## 0.2.1

### Bugfixes

- Removed `nomem` options from SMC and HVC call `asm!` blocks. The implementation of an HVC or SMC
  call may access the program's memory, so this isn't necessarily correct.

## 0.2.0

### Breaking changes

- Added `psci::Version` type, which is used for the return value of `psci::version`.

### New features

- Error types now implement `core::error::Error`.
- Added 32-bit versions of 64-bit PSCI calls.
- Added support for aarch32.

## 0.1.1

### Bugfixes

- Fixed docs.rs to build for aarch64.

## 0.1.0

Renamed crate to `smccc`.

### Breaking changes

- Moved PSCI code to the `psci` module, moved other modules up one level.
- Use type parameters rather than features to specify HVC vs. SMC for PSCI and arch calls.
- Changed `error::Error::Unknown` to contain an `i64` rather than an `i32`.

## `psci` 0.1.3

### Bugfixes

- Fixed type of `smccc::error::success_or_error_64`. This is a breaking change relative to 0.1.2 but
  it was yanked.

## `psci` 0.1.2 (yanked)

### New features

- Added constants, types and functions for standard Arm architucture SMCCC calls, in `smccc::arch`
  module.
- Added helpers in `smccc::error` module for handling negative return values as errors.

## `psci` 0.1.1

### New features

- Exposed functions for SMC and HVC calls for use outside of PSCI.

## `psci` 0.1.0

Initial release with PSCI constants and functions.
