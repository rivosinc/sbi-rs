// SPDX-FileCopyrightText: 2023 Rivos Inc.
//
// SPDX-License-Identifier: Apache-2.0

/// Debug Console for printing strings through SBI.
pub mod debug_console;

/// Host interfaces for reset extension.
pub mod reset;

/// Host interfaces for hart state management.
pub mod state;

/// Host interfaces for nested virtualization acceleration.
pub mod nacl;

/// Host interfaces for confidential computing.
pub mod cove_host;

/// Host interfaces for confidential computing interrupt virtualization.
pub mod cove_interrupt;

/// Guest interfaces for confidential computing.
pub mod cove_guest;

/// Host interfaces for PMU.
pub mod pmu;

/// Base SBI inferfaces.
pub mod base;

/// Host interfaces for attestation.
pub mod attestation;

/// Salus vendor extensions.
pub mod salus;
