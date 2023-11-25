use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use core::fmt::{Debug, Formatter, Result};

use libax::hv::{HyperCraftHal, HyperCraftHalImpl, GuestPhysAddr, HostPhysAddr, phys_to_virt, virt_to_phys, Result as HyperResult, Error, GuestPageTable, GuestPageTableTrait};

use page_table_entry::MappingFlags;

// about guests
pub const BIOS_PADDR: HostPhysAddr = 0x4000000;
pub const BIOS_SIZE: usize = 0x1000;

pub const GUEST_IMAGE_PADDR: HostPhysAddr = 0x4001000;
pub const GUEST_IMAGE_SIZE: usize = 0x10_0000; // 1M

pub const GUEST_PHYS_MEMORY_BASE: GuestPhysAddr = 0;
pub const BIOS_ENTRY: GuestPhysAddr = 0x8000;
pub const GUEST_ENTRY: GuestPhysAddr = 0x20_0000;
pub const GUEST_PHYS_MEMORY_SIZE: usize = 0x100_0000; // 16M

pub const MAX_VMS: usize = 2;

pub const fn is_aligned(addr: usize) -> bool {
    (addr & (HyperCraftHalImpl::PAGE_SIZE - 1)) == 0
}

#[derive(Debug)]
enum Mapper {
    Offset(usize),
}

#[derive(Debug)]
pub struct GuestMemoryRegion {
    pub gpa: GuestPhysAddr,
    pub hpa: HostPhysAddr,
    pub size: usize,
    pub flags: MappingFlags,
}

pub struct MapRegion {
    pub start: GuestPhysAddr,
    pub size: usize,
    pub flags: MappingFlags,
    mapper: Mapper,
}

impl MapRegion {
    pub fn new_offset(
        start_gpa: GuestPhysAddr,
        start_hpa: HostPhysAddr,
        size: usize,
        flags: MappingFlags,
    ) -> Self {
        assert!(is_aligned(start_gpa));
        assert!(is_aligned(start_hpa));
        assert!(is_aligned(size));
        let offset = start_gpa - start_hpa;
        Self {
            start: start_gpa,
            size,
            flags,
            mapper: Mapper::Offset(offset),
        }
    }

    fn is_overlap_with(&self, other: &Self) -> bool {
        let s0 = self.start;
        let e0 = s0 + self.size;
        let s1 = other.start;
        let e1 = s1 + other.size;
        !(e0 <= s1 || e1 <= s0)
    }

    fn target(&self, gpa: GuestPhysAddr) -> HostPhysAddr {
        match self.mapper {
            Mapper::Offset(off) => gpa.wrapping_sub(off),
        }
    }

    fn map_to(&self, npt: &mut GuestPageTable) -> HyperResult {
        let mut start = self.start;
        let end = start + self.size;
        while start < end {
            let target = self.target(start);
            npt.map(start, target, self.flags)?;
            start += HyperCraftHalImpl::PAGE_SIZE;
        }
        Ok(())
    }

    fn unmap_to(&self, npt: &mut GuestPageTable) -> HyperResult {
        let mut start = self.start;
        let end = start + self.size;
        while start < end {
            npt.unmap(start)?;
            start += HyperCraftHalImpl::PAGE_SIZE;
        }
        Ok(())
    }
}

impl Debug for MapRegion {
    fn fmt(&self, f: &mut Formatter) -> Result {
        f.debug_struct("MapRegion")
            .field("range", &(self.start..self.start + self.size))
            .field("size", &self.size)
            .field("flags", &self.flags)
            .field("mapper", &self.mapper)
            .finish()
    }
}

impl From<GuestMemoryRegion> for MapRegion {
    fn from(r: GuestMemoryRegion) -> Self {
        Self::new_offset(r.gpa, r.hpa, r.size, r.flags)
    }
}

pub struct GuestPhysMemorySet {
    regions: BTreeMap<GuestPhysAddr, MapRegion>,
    npt: GuestPageTable,
}

impl GuestPhysMemorySet {
    pub fn new() -> HyperResult<Self> {
        Ok(Self {
            npt: GuestPageTable::new()?,
            regions: BTreeMap::new(),
        })
    }

    pub fn nest_page_table_root(&self) -> HostPhysAddr {
        self.npt.root_paddr().into()
    }

    fn test_free_area(&self, other: &MapRegion) -> bool {
        if let Some((_, before)) = self.regions.range(..other.start).last() {
            if before.is_overlap_with(other) {
                return false;
            }
        }
        if let Some((_, after)) = self.regions.range(other.start..).next() {
            if after.is_overlap_with(other) {
                return false;
            }
        }
        true
    }

    pub fn map_region(&mut self, region: MapRegion) -> HyperResult {
        if region.size == 0 {
            return Ok(());
        }
        if !self.test_free_area(&region) {
            warn!(
                "MapRegion({:#x}..{:#x}) overlapped in:\n{:#x?}",
                region.start,
                region.start + region.size,
                self
            );
            return Err(Error::InvalidParam);
        }
        region.map_to(&mut self.npt)?;
        self.regions.insert(region.start, region);
        Ok(())
    }

    pub fn clear(&mut self) {
        for region in self.regions.values() {
            region.unmap_to(&mut self.npt).unwrap();
        }
        self.regions.clear();
    }
}

impl Drop for GuestPhysMemorySet {
    fn drop(&mut self) {
        self.clear();
    }
}

impl Debug for GuestPhysMemorySet {
    fn fmt(&self, f: &mut Formatter) -> Result {
        f.debug_struct("GuestPhysMemorySet")
            .field("page_table_root", &self.nest_page_table_root())
            .field("regions", &self.regions)
            .finish()
    }
}

#[repr(align(4096))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct AlignedMemory<const LEN: usize>([u8; LEN]);



pub(super) static mut GUEST_PHYS_MEMORY: [AlignedMemory<GUEST_PHYS_MEMORY_SIZE>;MAX_VMS]=
    [AlignedMemory([0; GUEST_PHYS_MEMORY_SIZE]);MAX_VMS];
pub(super) static mut IO_PHYS_MEMORY: [AlignedMemory<0x1000>;MAX_VMS]=
    [AlignedMemory([0; 0x1000]);MAX_VMS];
// pub(super) static mut HPET_PHYS_MEMORY: [AlignedMemory<0x1000>;MAX_VMS]=
//     [AlignedMemory([0; 0x1000]);MAX_VMS];
pub(super) static mut LAPIC_PHYS_MEMORY: [AlignedMemory<0x1000>;MAX_VMS]=
    [AlignedMemory([0; 0x1000]);MAX_VMS];
// pub(super) static mut GUEST_PHYS_MEMORY_0: AlignedMemory<GUEST_PHYS_MEMORY_SIZE>=
//     AlignedMemory([0; GUEST_PHYS_MEMORY_SIZE]);
// pub(super) static mut GUEST_PHYS_MEMORY_1: AlignedMemory<GUEST_PHYS_MEMORY_SIZE>=
//     AlignedMemory([0; GUEST_PHYS_MEMORY_SIZE]);

fn gpa_as_mut_ptr(guest_paddr: GuestPhysAddr, id: usize) -> *mut u8 {
    let offset = unsafe { &GUEST_PHYS_MEMORY[id] as *const _ as usize };
    let host_vaddr = guest_paddr + offset;
    host_vaddr as *mut u8
    // if id == 0 {
    //     let offset = unsafe { &GUEST_PHYS_MEMORY_0 as *const _ as usize };
    //     let host_vaddr = guest_paddr + offset;
    //     host_vaddr as *mut u8
    // } else {
    //     // id = 1
    //     let offset = unsafe { &GUEST_PHYS_MEMORY_1 as *const _ as usize };
    //     let host_vaddr = guest_paddr + offset;
    //     host_vaddr as *mut u8
    // }
}

#[cfg(target_arch = "x86_64")]
fn load_guest_image(hpa: HostPhysAddr, load_gpa: GuestPhysAddr, size: usize, id: usize) {
    let image_ptr = usize::from(phys_to_virt(hpa.into())) as *const u8;
    let image = unsafe { core::slice::from_raw_parts(image_ptr, size) };

    trace!("loading to guest memory: host {:#x} to guest {:#x}, size {:#x}", image_ptr as usize, load_gpa, size);

    unsafe {
        core::slice::from_raw_parts_mut(gpa_as_mut_ptr(load_gpa, id), size).copy_from_slice(image)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ConfigFile {
    pub id: usize,
    pub memory: usize,
    pub vcpu_count: usize,
    pub io_apic: usize,
    pub HPET: usize,
    pub local_apic: usize,
}

#[cfg(target_arch = "x86_64")]
pub fn setup_gpm(id: usize, config_file: ConfigFile) -> HyperResult<GuestPhysMemorySet> {
    // copy BIOS and guest images

    use libax::hv::HostVirtAddr;
    load_guest_image(BIOS_PADDR, BIOS_ENTRY, BIOS_SIZE, id);
    load_guest_image(GUEST_IMAGE_PADDR, GUEST_ENTRY, GUEST_IMAGE_SIZE, id);

    // create nested page table and add mapping
    let mut gpm = GuestPhysMemorySet::new()?;
    // let hpa_base:usize = virt_to_phys((gpa_as_mut_ptr(GUEST_PHYS_MEMORY_BASE + id * GUEST_PHYS_MEMORY_SIZE) as HostVirtAddr).into()).into();
    // let hpa_base = 0x26_8000;
    // info!("hpa_base {:x}",hpa_base);
    let mut guest_memory_regions = Vec::new();
    guest_memory_regions.push(GuestMemoryRegion {
        // RAM
        gpa: GUEST_PHYS_MEMORY_BASE,
        hpa: virt_to_phys((gpa_as_mut_ptr(GUEST_PHYS_MEMORY_BASE, id) as HostVirtAddr).into()).into(),
        // hpa: hpa_base,
        size: config_file.memory,
        flags: MappingFlags::READ | MappingFlags::WRITE | MappingFlags::EXECUTE,
    });
    if config_file.io_apic != 0 {
        guest_memory_regions.push(GuestMemoryRegion {
            // IO APIC
            gpa: 0xfec0_0000,
            hpa: unsafe { &IO_PHYS_MEMORY[id] as *const _ as usize },
            size: 0x1000,
            flags: MappingFlags::READ | MappingFlags::WRITE | MappingFlags::DEVICE,
        });
    }
    if config_file.HPET != 0 {
        guest_memory_regions.push(GuestMemoryRegion {
            // HPET
            gpa: 0xfed0_0000,
            hpa: 0xfed0_0000,
            size: 0x1000,
            flags: MappingFlags::READ | MappingFlags::WRITE | MappingFlags::DEVICE,
        });
    }
    if config_file.local_apic != 0 {
        guest_memory_regions.push(GuestMemoryRegion {
            // Local APIC
            gpa: 0xfee0_0000,
            hpa: unsafe { &LAPIC_PHYS_MEMORY[id] as *const _ as usize },
            size: 0x1000,
            flags: MappingFlags::READ | MappingFlags::WRITE | MappingFlags::DEVICE,
        });
    }
    
    for r in guest_memory_regions.into_iter() {
        gpm.map_region(r.into())?;
    }
    Ok(gpm)
}
