//! x86 page table entries on 64-bit paging.
mod pte;
mod epte;

pub use pte::{PTF, X64PTE};
pub use epte::EPTEntry;
