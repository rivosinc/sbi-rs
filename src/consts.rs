// SPDX-FileCopyrightText: 2023 Rivos Inc.
//
// SPDX-License-Identifier: Apache-2.0

#![allow(missing_docs, dead_code)]

// Extension constants
pub const EXT_PUT_CHAR: u64 = 0x01;
pub const EXT_BASE: u64 = 0x10;
pub const EXT_HART_STATE: u64 = 0x48534D;
pub const EXT_PMU: u64 = 0x504D55;
pub const EXT_RESET: u64 = 0x53525354;
pub const EXT_DBCN: u64 = 0x4442434E; // DBCN
pub const EXT_ATTESTATION: u64 = 0x41545354; // ATST
pub const EXT_NACL: u64 = 0x4E41434C; // NACL
pub const EXT_COVE_HOST: u64 = 0x434F5648; // COVH
pub const EXT_COVE_INTERRUPT: u64 = 0x434F5649; // COVI
pub const EXT_COVE_GUEST: u64 = 0x434F5647; // COVG

pub const EXT_VENDOR_RANGE_START: u64 = 0x09000000;
pub const EXT_VENDOR_RANGE_END: u64 = 0x09FFFFFF;

pub const SBI_SUCCESS: i64 = 0;
pub const SBI_ERR_INVALID_ADDRESS: i64 = -5;
