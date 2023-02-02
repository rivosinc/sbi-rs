// Copyright (c) 2022 by Rivos Inc.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

use crate::error::*;
use crate::function::*;

/// Functions for the Reset extension
#[derive(Copy, Clone, Debug)]
pub enum ResetFunction {
    /// Performs a system reset.
    Reset {
        /// Determines the type of reset to perform.
        reset_type: ResetType,
        /// Represents the reason for system reset.
        reason: ResetReason,
    },
}

/// The types of reset a supervisor can request.
#[repr(u64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ResetType {
    /// Powers down the system.
    Shutdown = 0,
    /// Powers down, then reboots.
    ColdReset = 1,
    /// Reboots, doesn't power down.
    WarmReset = 2,
}

impl ResetType {
    // Creates a reset type from the a0 register value or returns an error if no mapping is
    // known for the given value.
    fn from_reg(a0: u64) -> Result<Self> {
        use ResetType::*;
        Ok(match a0 {
            0 => Shutdown,
            1 => ColdReset,
            2 => WarmReset,
            _ => return Err(Error::InvalidParam),
        })
    }
}

/// Reasons why a supervisor requests a reset.
#[repr(u64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ResetReason {
    /// Used for normal resets.
    NoReason = 0,
    /// Used when the system has failed.
    SystemFailure = 1,
}

impl ResetReason {
    // Creates a reset reason from the a1 register value or returns an error if no mapping is
    // known for the given value.
    fn from_reg(a1: u64) -> Result<Self> {
        use ResetReason::*;
        Ok(match a1 {
            0 => NoReason,
            1 => SystemFailure,
            _ => return Err(Error::InvalidParam),
        })
    }
}

impl ResetFunction {
    /// Attempts to parse `Self` from the passed in `a0-a7`.
    pub(crate) fn from_regs(args: &[u64]) -> Result<Self> {
        use ResetFunction::*;

        Ok(match args[6] {
            0 => Reset {
                reset_type: ResetType::from_reg(args[0])?,
                reason: ResetReason::from_reg(args[1])?,
            },
            _ => return Err(Error::NotSupported),
        })
    }

    /// Creates an operation to shutdown the machine.
    pub fn shutdown() -> Self {
        ResetFunction::Reset {
            reset_type: ResetType::Shutdown,
            reason: ResetReason::NoReason,
        }
    }
}

impl SbiFunction for ResetFunction {
    fn a0(&self) -> u64 {
        match self {
            ResetFunction::Reset {
                reset_type,
                reason: _,
            } => *reset_type as u64,
        }
    }

    fn a1(&self) -> u64 {
        match self {
            ResetFunction::Reset {
                reset_type: _,
                reason,
            } => *reason as u64,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shutdown_systemfailure() {
        let reset = ResetFunction::Reset {
            reset_type: ResetType::Shutdown,
            reason: ResetReason::SystemFailure,
        };

        assert_eq!(reset.a0(), 0); // a0 = Shutdown (0x00000000)
        assert_eq!(reset.a1(), 1); // a1 = System failure (0x00000001)
        assert_eq!(reset.a6(), 0); // a6 = function id 0
    }

    #[test]
    fn shutdown_noreason() {
        let reset = ResetFunction::Reset {
            reset_type: ResetType::Shutdown,
            reason: ResetReason::NoReason,
        };

        assert_eq!(reset.a0(), 0); // a0 = Shutdown (0x00000000)
        assert_eq!(reset.a1(), 0); // a1 = No reason (0x00000000)
        assert_eq!(reset.a6(), 0); // a6 = function id 0
    }

    #[test]
    fn coldreset_noreason() {
        let reset = ResetFunction::Reset {
            reset_type: ResetType::ColdReset,
            reason: ResetReason::NoReason,
        };

        assert_eq!(reset.a0(), 1); // a0 = Cold reboot (0x00000001)
        assert_eq!(reset.a1(), 0); // a1 = No reason (0x00000000)
        assert_eq!(reset.a6(), 0); // a6 = function id 0
    }

    #[test]
    fn warmreset_noreason() {
        let reset = ResetFunction::Reset {
            reset_type: ResetType::WarmReset,
            reason: ResetReason::NoReason,
        };

        assert_eq!(reset.a0(), 2); // a0 = Warm reboot (0x00000002)
        assert_eq!(reset.a1(), 0); // a1 = No reason (0x00000000)
        assert_eq!(reset.a6(), 0); // a6 = function id 0
    }

    #[test]
    fn shutdown() {
        let reset = ResetFunction::shutdown();

        assert_eq!(reset.a0(), 0); // a0 = Shutdown (0x00000000)
        assert_eq!(reset.a1(), 0); // a1 = No reason (0x00000000)
        assert_eq!(reset.a6(), 0); // a6 = function id 0
    }
}
