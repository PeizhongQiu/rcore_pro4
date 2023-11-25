#[macro_use]
mod regs;

// Codes in this module come mainly from https://github.com/rcore-os/RVM-Tutorial

mod ept;
mod lapic;
mod memory;
mod msr;
mod vmx;
mod percpu;

use crate::{GuestPageTableTrait, HyperCraftHal};
use page_table::PagingIf;

/// Initialize the hypervisor runtime.
pub fn init_hv_runtime() {
    if !vmx::has_hardware_support() {
        panic!("VMX not supported");
    }
}

/// Nested page table define.
pub use ept::ExtendedPageTable as NestedPageTable;

/// VCpu define.
pub use vmx::VmxVcpu as VCpu;
pub use percpu::PerCpu;
pub use vmx::{VmxExitReason, VmxExitInfo};
pub use vmx::VM;

////// Following are things to be implemented


impl<H: HyperCraftHal> VCpu<H> {
    /// Get the vcpu id.
    pub fn vcpu_id(&self) -> usize {
        self.get_vcpu_id()
    }
}

// /// VM define.
// pub struct VM<H: HyperCraftHal> {
//     _marker: core::marker::PhantomData<H>,
// }

/// VM exit information.
pub struct VmExitInfo {}

/// General purpose register index.
pub enum GprIndex {}

/// Hypercall message.
pub enum HyperCallMsg {}

