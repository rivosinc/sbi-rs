// SPDX-FileCopyrightText: 2023 Rivos Inc.
//
// SPDX-License-Identifier: Apache-2.0

use crate::error::*;
use crate::function::*;

/// Number of bytes in the `NaclShmem` scratch area.
pub const NACL_SCRATCH_BYTES: usize = 2048;

/// Layout of the shared-memory area registered with `SetShmem`.
#[repr(C)]
pub struct NaclShmem {
    /// Scratch space. The layout of this scratch space is defined by the particular function being
    /// invoked.
    ///
    /// For the `TvmCpuRun` function in the COVE-Host extension, the layout of this scratch space
    /// matches the `TsmShmemScratch` struct.
    pub scratch: [u64; NACL_SCRATCH_BYTES / 8],
    _reserved: [u64; 240],
    /// Bitmap indicating which CSRs in `csrs` the host wishes to sync.
    ///
    /// Currently unused in the COVE-related extensions and will not be read or written by the TSM.
    pub dirty_bitmap: [u64; 16],
    /// Hypervisor and virtual-supervisor CSRs. The 12-bit CSR number is transformed into a 10-bit
    /// index by extracting bits `{csr[11:10], csr[7:0]}` since `csr[9:8]` is always 2'b10 for HS
    /// and VS CSRs.
    ///
    /// These CSRs may be updated by `TvmCpuRun` in the COVE-Host extension. See the documentation
    /// of `TvmCpuRun` for more details.
    pub csrs: [u64; 1024],
}

impl NaclShmem {
    /// Returns the index in `csrs` of the HS or VS CSR at `csr_num`.
    pub fn csr_index(csr_num: u16) -> usize {
        (((csr_num & 0xc00) >> 2) | (csr_num & 0xff)) as usize
    }
}

impl Default for NaclShmem {
    fn default() -> Self {
        Self {
            scratch: [0; 256],
            _reserved: [0; 240],
            dirty_bitmap: [0; 16],
            csrs: [0; 1024],
        }
    }
}

/// NaclFunction::ProbeFeature feature IDs.
pub enum NaclFeature {
    /// The synchronize CSR feature describes the ability of the SBI implementation
    /// (Salus) to allow supervisor software (Host) to write RISC-V H-extension CSRs
    /// using the CSR space.
    SyncCSR = 0,
    /// The synchronize HFENCE feature describes the ability of the SBI implementation
    /// (Salus) to allow supervisor software (Host) to do HFENCE using the scratch space
    SyncHfence = 1,
    /// The synchronize SRET feature describes the ability of the SBI implementation
    /// (Salus) to do synchronization of CSRs and HFENCEs in the nested acceleration
    /// shared memory for the supervisor software (Host) along with SRET emulation.
    SyncSret = 2,
    /// The autoswap CSR feature describes the ability of the SBI implementation (Salus)
    /// to swap certain RISC-V H-extension CSR values from the nested acceleration shared
    /// memory whenever the virtualization state of the supervisor software (Host) changes.
    AutoSwapCsr = 3,
}

/// Functions provided by the Nested Virtualization Acceleration (NACL) extension.
#[derive(Copy, Clone, Debug)]
pub enum NaclFunction {
    /// Allows the nested hypervisor to query Nacl features supported by the host hypervisor.
    /// Features are given in `enum NaclFeature`. Salus doesn't support any of the features
    /// and only allows the host to read CSRs. Any writes are ignored.
    ///
    /// a6 = 0
    ProbeFeature {
        /// a0 = Feature ID being checked.
        feature_id: u64,
    },
    /// Registers the nested hypervisor <-> host hypervisor shared memory area for the calling CPU.
    /// `shmem_pfn` is the base PFN of where the `NaclShmem` struct will be placed in the caller's
    /// physical address space. The entire range of memory occupied by the `NaclShmem` struct must
    /// remain accessible to the caller until the `NaclShmem` structure is unregistered by calling
    /// this function with `shmem_pfn` set to -1. In particular this means that, in the presence of
    /// the COVE-Host extension, the memory occupied by the `NaclShmem` structure is "pinned" in
    /// the non-confidential state and cannot be converted.
    ///
    /// a6 = 1
    SetShmem {
        /// a0 = Address of the shared memory area.
        shmem_addr: u64,
    },
    // There are other functions in the proposed NACL extension, but we ignore them as they aren't
    // relevant to the COVE extensions. Note that this violates SBI policy, but since both the COVE and
    // NACL extensions are in active development, we let it go for now.
}

impl NaclFunction {
    /// Attempts to parse `Self` from the passed in `a0-a7`.
    pub(crate) fn from_regs(args: &[u64]) -> Result<Self> {
        use NaclFunction::*;
        match args[6] {
            0 => Ok(ProbeFeature {
                feature_id: args[0],
            }),
            1 => Ok(SetShmem {
                shmem_addr: args[0],
            }),
            _ => Err(Error::NotSupported),
        }
    }
}

impl SbiFunction for NaclFunction {
    fn a6(&self) -> u64 {
        use NaclFunction::*;
        match self {
            ProbeFeature { .. } => 0,
            SetShmem { .. } => 1,
        }
    }

    fn a0(&self) -> u64 {
        use NaclFunction::*;
        match self {
            ProbeFeature { feature_id } => *feature_id,
            SetShmem { shmem_addr } => *shmem_addr,
        }
    }
}
