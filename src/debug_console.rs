// SPDX-FileCopyrightText: 2023 Rivos Inc.
//
// SPDX-License-Identifier: Apache-2.0

use crate::error::*;
use crate::function::*;

/// Functions for the Debug Console extension
#[derive(Copy, Clone, Debug)]
pub enum DebugConsoleFunction {
    /// Prints the given string to the system console.
    Write {
        /// The length of the string to print.
        len: u64,
        /// The address of the string.
        addr: u64,
        /// For rv32, the upper bits of address if needed.
        addr_hi: u64,
    },
    /// Reads from the console.
    Read {
        /// The length of the buffer to read into.
        len: u64,
        /// The address of the buffer to read into.
        addr: u64,
        /// For rv32, the upper bits of address if needed.
        addr_hi: u64,
    },
    /// Writes a single byte to the console.
    WriteByte {
        /// The byte to write.
        byte: u64,
    },
}

impl DebugConsoleFunction {
    /// Attempts to parse `Self` from the passed in `a0-a7`.
    pub(crate) fn from_regs(args: &[u64]) -> Result<Self> {
        Ok(match args[6] {
            0 => DebugConsoleFunction::Write {
                len: args[0],
                addr: args[1],
                addr_hi: args[2],
            },
            1 => DebugConsoleFunction::Read {
                len: args[0],
                addr: args[1],
                addr_hi: args[2],
            },
            2 => DebugConsoleFunction::WriteByte { byte: args[0] },
            _ => return Err(Error::NotSupported),
        })
    }
}

impl SbiFunction for DebugConsoleFunction {
    fn a0(&self) -> u64 {
        match self {
            DebugConsoleFunction::Write {
                len,
                addr: _,
                addr_hi: _,
            } => *len,
            DebugConsoleFunction::Read {
                len,
                addr: _,
                addr_hi: _,
            } => *len,
            DebugConsoleFunction::WriteByte { byte } => *byte,
        }
    }

    fn a1(&self) -> u64 {
        match self {
            DebugConsoleFunction::Write {
                len: _,
                addr,
                addr_hi: _,
            } => *addr,
            DebugConsoleFunction::Read {
                len: _,
                addr,
                addr_hi: _,
            } => *addr,
            DebugConsoleFunction::WriteByte { byte: _ } => 0,
        }
    }

    fn a2(&self) -> u64 {
        match self {
            DebugConsoleFunction::Write {
                len: _,
                addr: _,
                addr_hi,
            } => *addr_hi,
            DebugConsoleFunction::Read {
                len: _,
                addr: _,
                addr_hi,
            } => *addr_hi,
            DebugConsoleFunction::WriteByte { byte: _ } => 0,
        }
    }
}
