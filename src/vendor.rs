// Copyright (c) 2023 by Rivos Inc.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

use crate::error::*;
use crate::function::*;

/// Functions for the Reset extension
#[derive(Copy, Clone, Debug)]
pub enum VendorFunction {
    /// Sends a Vendor-specific message
    Vendor {
        eid: u32,
        fid: u32,
        arg0: u64,
        arg1: u64,
        arg2: u64,
        arg3: u64,
        arg4: u64,
        arg5: u64,
    },
}

impl VendorFunction {
    /// Attempts to parse `Self` from the passed in `a0-a7`.
    pub(crate) fn from_regs(args: &[u64]) -> Result<Self> {
        use VendorFunction::*;

        Ok(match args[8] {
            0 => Vendor {
                eid: 0,
                fid: 0,
                arg0: (args[0]),
                arg1: (args[1]),
                arg2: (args[2]),
                arg3: (args[3]),
                arg4: (args[4]),
                arg5: (args[5]),
            },
            _ => return Err(Error::NotSupported),
        })
    }

    pub fn call0(eid: u32, fid: u32, arg0: u64) -> Self {
        VendorFunction::Vendor {
            eid: (eid),
            fid: (fid),
            arg0: (arg0),
            arg1: (0),
            arg2: (0),
            arg3: (0),
            arg4: (0),
            arg5: (0),
        }
    }

    pub fn call1(eid: u32, fid: u32, arg0: u64, arg1: u64) -> Self {
        VendorFunction::Vendor {
            eid: (eid),
            fid: (fid),
            arg0: (arg0),
            arg1: (arg1),
            arg2: (0),
            arg3: (0),
            arg4: (0),
            arg5: (0),
        }
    }

    pub fn call2(eid: u32, fid: u32, arg0: u64, arg1: u64, arg2: u64) -> Self {
        VendorFunction::Vendor {
            eid: (eid),
            fid: (fid),
            arg0: (arg0),
            arg1: (arg1),
            arg2: (arg2),
            arg3: (0),
            arg4: (0),
            arg5: (0),
        }
    }

    pub fn call3(eid: u32, fid: u32, arg0: u64, arg1: u64, arg2: u64, arg3: u64) -> Self {
        VendorFunction::Vendor {
            eid: (eid),
            fid: (fid),
            arg0: (arg0),
            arg1: (arg1),
            arg2: (arg2),
            arg3: (arg3),
            arg4: (0),
            arg5: (0),
        }
    }

    pub fn call4(
        eid: u32,
        fid: u32,
        arg0: u64,
        arg1: u64,
        arg2: u64,
        arg3: u64,
        arg4: u64,
    ) -> Self {
        VendorFunction::Vendor {
            eid: (eid),
            fid: (fid),
            arg0: (arg0),
            arg1: (arg1),
            arg2: (arg2),
            arg3: (arg3),
            arg4: (arg4),
            arg5: (0),
        }
    }

    pub fn call5(
        eid: u32,
        fid: u32,
        arg0: u64,
        arg1: u64,
        arg2: u64,
        arg3: u64,
        arg4: u64,
        arg5: u64,
    ) -> Self {
        VendorFunction::Vendor {
            eid: (eid),
            fid: (fid),
            arg0: (arg0),
            arg1: (arg1),
            arg2: (arg2),
            arg3: (arg3),
            arg4: (arg4),
            arg5: (arg5),
        }
    }
}

impl SbiFunction for VendorFunction {
    fn a0(&self) -> u64 {
        match self {
            VendorFunction::Vendor {
                eid: _,
                fid: _,
                arg0,
                arg1: _,
                arg2: _,
                arg3: _,
                arg4: _,
                arg5: _,
            } => *arg0 as u64,
        }
    }

    fn a1(&self) -> u64 {
        match self {
            VendorFunction::Vendor {
                eid: _,
                fid: _,
                arg0: _,
                arg1,
                arg2: _,
                arg3: _,
                arg4: _,
                arg5: _,
            } => *arg1 as u64,
        }
    }

    fn a2(&self) -> u64 {
        match self {
            VendorFunction::Vendor {
                eid: _,
                fid: _,
                arg0: _,
                arg1: _,
                arg2,
                arg3: _,
                arg4: _,
                arg5: _,
            } => *arg2 as u64,
        }
    }

    fn a3(&self) -> u64 {
        match self {
            VendorFunction::Vendor {
                eid: _,
                fid: _,
                arg0: _,
                arg1: _,
                arg2: _,
                arg3,
                arg4: _,
                arg5: _,
            } => *arg3 as u64,
        }
    }
    fn a4(&self) -> u64 {
        match self {
            VendorFunction::Vendor {
                eid: _,
                fid: _,
                arg0: _,
                arg1: _,
                arg2: _,
                arg3: _,
                arg4,
                arg5: _,
            } => *arg4 as u64,
        }
    }

    fn a5(&self) -> u64 {
        match self {
            VendorFunction::Vendor {
                eid: _,
                fid: _,
                arg0: _,
                arg1: _,
                arg2: _,
                arg3: _,
                arg4: _,
                arg5,
            } => *arg5 as u64,
        }
    }
}
