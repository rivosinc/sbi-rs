// SPDX-FileCopyrightText: 2023 Rivos Inc.
//
// SPDX-License-Identifier: Apache-2.0

use crate::error::*;
use crate::{SbiFunction, SbiMessage};

// Salus Vendor Extensions.

const EXT_SALUS_TEST: u64 = 0x09FFFFFF;

/// Salus-specific SBI vendor extensions.
pub enum SalusSbiMessage {
    /// Salus Test functions.
    SalusTest(SalusTestFunction),
}

impl SalusSbiMessage {
    /// Parse a Salus Sbi Message from registers.
    pub fn from_regs(args: &[u64]) -> Result<Self> {
        use SalusSbiMessage::*;
        match args[7] {
            EXT_SALUS_TEST => SalusTestFunction::from_regs(args).map(SalusTest),
            _ => Err(Error::NotSupported),
        }
    }

    /// Returns the `SbiFunction` trait of this message.
    fn func(&self) -> impl SbiFunction {
        use SalusSbiMessage::*;
        match *self {
            SalusTest(func) => func,
        }
    }

    /// Returns the Extension ID of this message.
    fn eid(&self) -> u64 {
        use SalusSbiMessage::*;
        match *self {
            SalusTest(_) => EXT_SALUS_TEST,
        }
    }
}

impl From<SalusSbiMessage> for SbiMessage {
    fn from(msg: SalusSbiMessage) -> SbiMessage {
        let args: [u64; 8] = [
            msg.func().a0(),
            msg.func().a1(),
            msg.func().a2(),
            msg.func().a3(),
            msg.func().a4(),
            msg.func().a5(),
            msg.func().a6(),
            msg.eid(),
        ];
        SbiMessage::Vendor(args)
    }
}

/// Functions defined for the Salus Test extension
#[derive(Clone, Copy, Debug)]
pub enum SalusTestFunction {
    /// Memcopy Test.
    MemCopy(MemCopyArgs),
}

impl SalusTestFunction {
    /// Attempts to parse `Self` from the passed in `a0-a7`.
    pub(crate) fn from_regs(args: &[u64]) -> Result<Self> {
        use SalusTestFunction::*;

        match args[6] {
            0 => Ok(MemCopy(MemCopyArgs {
                to: args[0],
                from: args[1],
                len: args[2],
            })),
            _ => Err(Error::NotSupported),
        }
    }
}

impl SbiFunction for SalusTestFunction {
    fn a0(&self) -> u64 {
        use SalusTestFunction::*;
        match self {
            MemCopy(args) => args.to,
        }
    }

    fn a1(&self) -> u64 {
        use SalusTestFunction::*;
        match self {
            MemCopy(args) => args.from,
        }
    }

    fn a2(&self) -> u64 {
        use SalusTestFunction::*;
        match self {
            MemCopy(args) => args.len,
        }
    }

    fn a6(&self) -> u64 {
        use SalusTestFunction::*;
        match self {
            MemCopy(_) => 0,
        }
    }
}

/// Arguments to the memcpy test function
#[derive(Clone, Copy, Debug)]
pub struct MemCopyArgs {
    /// Destination Address.
    pub to: u64,
    /// Source Address.
    pub from: u64,
    /// Length in bytes of the copy.
    pub len: u64,
}
