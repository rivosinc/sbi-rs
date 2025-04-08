// SPDX-FileCopyrightText: 2023 Rivos Inc.
//
// SPDX-License-Identifier: Apache-2.0

use crate::CoveInterruptFunction::*;
use crate::TvmAiaParams;
use crate::{ecall_send, Result, SbiMessage};

/// Configures AIA virtualization for `tvm_id` with the settings in `tvm_aia_params`.
pub fn tvm_aia_init(tvm_id: u64, tvm_aia_params: TvmAiaParams) -> Result<()> {
    let msg = SbiMessage::CoveInterrupt(TvmAiaInit {
        tvm_id,
        params_addr: (&tvm_aia_params as *const TvmAiaParams) as u64,
        len: core::mem::size_of::<TvmAiaParams>() as u64,
    });
    // Safety: `TvmConfigureAia` will only read up to `len` bytes of the `TvmAiaParams` structure
    // we passed in.
    unsafe { ecall_send::<()>(&msg) }?;
    Ok(())
}

/// Sets the guest physical address of the specified vCPU's virtualized IMSIC to `imsic_addr`.
pub fn set_vcpu_imsic_addr(tvm_id: u64, vcpu_id: u64, imsic_addr: u64) -> Result<()> {
    let msg = SbiMessage::CoveInterrupt(TvmCpuSetImsicAddr {
        tvm_id,
        vcpu_id,
        imsic_addr,
    });
    // Safety: `TvmCpuSetImsicAddr` doesn't touch host memory in any way.
    unsafe { ecall_send::<()>(&msg) }?;
    Ok(())
}

/// Converts the guest interrupt file at `imsic_addr` for use with a TVM.
///
/// # Safety
///
/// The caller must not access the guest interrupt file again until it has been reclaimed.
pub unsafe fn convert_imsic(imsic_addr: u64) -> Result<()> {
    let msg = SbiMessage::CoveInterrupt(TsmConvertImsic { imsic_addr });
    // The caller must guarantee that they won't access the page at `imsic_addr`.
    ecall_send::<()>(&msg)?;
    Ok(())
}

/// Reclaims the guest interrupt file at `imsic_addr` that was previously converted with
/// `convert_imsic()`.
pub fn reclaim_imsic(imsic_addr: u64) -> Result<()> {
    let msg = SbiMessage::CoveInterrupt(TsmReclaimImsic { imsic_addr });
    // Safety: The referenced page is made available again, which is safe since it hasn't been
    // accessible since conversion.
    unsafe { ecall_send::<()>(&msg) }?;
    Ok(())
}

/// Binds a vCPU to this physical CPU and the specified set of confidential guest interrupt
/// files.
pub fn bind_vcpu_imsic(tvm_id: u64, vcpu_id: u64, imsic_mask: u64) -> Result<()> {
    let msg = SbiMessage::CoveInterrupt(TvmCpuBindImsic {
        tvm_id,
        vcpu_id,
        imsic_mask,
    });
    // Safety: The specified guest interrupt files must have already been inaccessible.
    unsafe { ecall_send::<()>(&msg) }?;
    Ok(())
}

/// Begins the unbind process for the specified vCPU from this physical CPU and its guest
/// interrupt files. The host must complete a TLB invalidation sequence for the TVM before
/// completing the unbind with `unbind_vcpu_imsic_end()`.
pub fn unbind_vcpu_imsic_begin(tvm_id: u64, vcpu_id: u64) -> Result<()> {
    let msg = SbiMessage::CoveInterrupt(TvmCpuUnbindImsicBegin { tvm_id, vcpu_id });
    // Safety: Does not access host memory.
    unsafe { ecall_send::<()>(&msg) }?;
    Ok(())
}

/// Completes the unbind process for the specified vCPU from this physical CPU and its guest
/// interrupt files. The interrupt files are free to be reclaimed or bound to another vCPU,
/// and the vCPU can now be bound to another physical CPU.
pub fn unbind_vcpu_imsic_end(tvm_id: u64, vcpu_id: u64) -> Result<()> {
    let msg = SbiMessage::CoveInterrupt(TvmCpuUnbindImsicEnd { tvm_id, vcpu_id });
    // Safety: Does not access host memory.
    unsafe { ecall_send::<()>(&msg) }?;
    Ok(())
}

/// Injects an external interrupt into the specified vCPU. The interrupt ID must have been
/// allowed with `allow_external_interrupt()` by the guest.
pub fn inject_external_interrupt(tvm_id: u64, vcpu_id: u64, interrupt_id: u64) -> Result<()> {
    let msg = SbiMessage::CoveInterrupt(TvmCpuInjectExternalInterrupt {
        tvm_id,
        vcpu_id,
        interrupt_id,
    });
    // Safety: Does not access host memory.
    unsafe { ecall_send::<()>(&msg) }?;
    Ok(())
}

/// Begins the rebinding process for the specified vCPU to this physical CPU and the specified
/// confidential guest interrupt file. The host must complete a TLB invalidation sequence
/// for the TVM before cloning old interrupt file state using `rebind_vcpu_imsic_clone`. Once cloned
/// the old file will be restored to new guest interrupt file on `rebind_vcpu_imsic_end` invocation.
pub fn rebind_vcpu_imsic_begin(tvm_id: u64, vcpu_id: u64, imsic_mask: u64) -> Result<()> {
    let msg = SbiMessage::CoveInterrupt(TvmCpuRebindImsicBegin {
        tvm_id,
        vcpu_id,
        imsic_mask,
    });
    // Safety: The specified guest interrupt files must have already been inaccessible.
    unsafe { ecall_send::<()>(&msg) }?;
    Ok(())
}

/// Clones the old guest interrupt file of the specified vCPU. Caller must make sure to call this from
/// old physical CPU. The guest interrupt file after this is free to be reclaimed or bound to another
/// vCPU.
pub fn rebind_vcpu_imsic_clone(tvm_id: u64, vcpu_id: u64) -> Result<()> {
    let msg = SbiMessage::CoveInterrupt(TvmCpuRebindImsicClone { tvm_id, vcpu_id });
    // Safety: Does not access host memory.
    unsafe { ecall_send::<()>(&msg) }?;
    Ok(())
}

/// Completes the rebind process for the specified vCPU from this physical CPU and its guest
/// interrupt files. Must be called from the same physical CPU as `rebind_vcpu_imsic_begin`.
pub fn rebind_vcpu_imsic_end(tvm_id: u64, vcpu_id: u64) -> Result<()> {
    let msg = SbiMessage::CoveInterrupt(TvmCpuRebindImsicEnd { tvm_id, vcpu_id });
    // Safety: Does not access host memory.
    unsafe { ecall_send::<()>(&msg) }?;
    Ok(())
}
