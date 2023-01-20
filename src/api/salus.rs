// Copyright (c) 2023 by Rivos Inc.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

use crate::salus::*;
use crate::{ecall_send, Result};

/// Copies `len` bytes from `from` to `to`.
///
/// # Safety
///
/// - `to` must be writable for `len` bytes.
/// - `src` must be readable for `len` bytes.
/// - the ranges starting at `to` and `src` for `len` bytes must not overlap.
/// - nothing mutates the memory of the `to` and `from` ranges while this function is called.
pub unsafe fn test_memcpy(to: *mut u8, from: *const u8, len: u64) -> Result<()> {
    let function = SalusTestFunction::MemCopy(MemCopyArgs {
        to: to as u64,
        from: from as u64,
        len,
    });
    let msg = SalusSbiMessage::SalusTest(function).into();
    ecall_send(&msg)?;
    Ok(())
}
