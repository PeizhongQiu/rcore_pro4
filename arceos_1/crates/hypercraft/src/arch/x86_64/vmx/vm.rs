use crate::{GuestPhysAddr, HostPhysAddr, HyperCraftHal, HyperResult, HyperError};
use alloc::vec::Vec;
use super::vcpu::VmxVcpu;

/// the struct of VM
#[repr(C)]
pub struct VM<H: HyperCraftHal> {
    id: usize,
    vcpu_count: usize,
    vcpu: Vec<VmxVcpu<H>>,
}

impl<H: HyperCraftHal> VM<H> {
    /// create new VM
    pub fn new(
        id: usize,
    ) -> Self {
        Self {
            id: id,
            vcpu_count: 0,
            vcpu: Vec::new(),
        }
    }
    /// add a new vcpu to VM
    pub fn add_vcpu(&mut self, vmcs_revision_id: u32, entry: GuestPhysAddr, npt_root: HostPhysAddr) -> HyperResult<usize> {
        self.vcpu.push(VmxVcpu::new(self.id, self.vcpu_count,vmcs_revision_id, entry, npt_root)?);
        // update vcpu_count
        self.vcpu_count += 1;
        Ok(self.vcpu_count - 1)
    }
    /// Returns a reference to the vCPU with `vcpu_id` if it exists.
    pub fn get_vcpu(&mut self, vcpu_id: usize) -> HyperResult<&mut VmxVcpu<H>> {
        info!("{} {}", vcpu_id, self.vcpu_count);
        if vcpu_id < self.vcpu_count {
            let vcpu = &mut self.vcpu[vcpu_id];
            Ok(vcpu)
        } else {
            Err(HyperError::NotFound)
        }
        
    }
    /// get vm id
    pub fn get_vm_id(&self) -> usize{
        self.id
    }
}