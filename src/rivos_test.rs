// Copyright (c) 2022 by Rivos Inc.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

use crate::error::*;
use crate::function::*;

/// Functions defined for the Rivos test extension
#[derive(Clone, Copy, Debug)]
pub enum RivosTestFunction {
    /// Returns the implemented version of the SBI standard.
    MemCopy(MemCopyArgs),
}

impl RivosTestFunction {
    /// Attempts to parse `Self` from the passed in `a0-a7`.
    pub(crate) fn from_regs(args: &[u64]) -> Result<Self> {
        use RivosTestFunction::*;

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

impl SbiFunction for RivosTestFunction {
    fn a6(&self) -> u64 {
        use RivosTestFunction::*;
        match self {
            MemCopy(_) => 0,
        }
    }

    fn a0(&self) -> u64 {
        use RivosTestFunction::*;
        match self {
            MemCopy(args) => args.to,
        }
    }

    fn a1(&self) -> u64 {
        use RivosTestFunction::*;
        match self {
            MemCopy(args) => args.from,
        }
    }

    fn a2(&self) -> u64 {
        use RivosTestFunction::*;
        match self {
            MemCopy(args) => args.len,
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
