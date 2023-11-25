#![no_std]
#![no_main]

extern crate alloc;
#[macro_use]
extern crate libax;

use libax::{
    hv::{
        HyperCraftHalImpl, PerCpu, VM,
    },
    info,
};
use core::sync::atomic::{AtomicUsize, Ordering};
use libax::thread;


mod x64;

const NUM_VM: usize = 2;
static FINISHED_TASKS: AtomicUsize = AtomicUsize::new(0);

#[no_mangle]
fn main(hart_id: usize) {
    println!("Hello, hv!");
    println!("into main {}", hart_id);

    let mut p = PerCpu::<HyperCraftHalImpl>::new(hart_id);
    p.hardware_enable().unwrap();
    let vmcs_revision_id = p.get_vmcs_revision_id();

    for id in 0..NUM_VM {
        thread::spawn(move || {
            println!("Hello, task {}! id = {:?}", id, thread::current().id());
            let gpm = x64::setup_gpm(id).unwrap();
            info!("{:#x?}", gpm);

            println!("Create VM{}...",id);
            let mut vm = VM::<HyperCraftHalImpl>::new(id);
            
            println!("VM {} add vcpu {}...", vm.get_vm_id(), 0);
            let vcpu_id = vm.add_vcpu(vmcs_revision_id, x64::BIOS_ENTRY, gpm.nest_page_table_root()).unwrap();

            let vcpu = vm.get_vcpu(vcpu_id).unwrap();
            println!("Running vcpu {}...", vcpu.get_vcpu_id());
            vcpu.run();

            let _order = FINISHED_TASKS.fetch_add(1, Ordering::Relaxed);
            if option_env!("SMP") == Some("1") {
                assert!(_order == id); // FIFO scheduler
            }
        });
    }

    println!("Hello, main task!");
    while FINISHED_TASKS.load(Ordering::Relaxed) < NUM_VM {
        thread::yield_now();
    }
    println!("Task yielding tests run OK!");
    
    p.hardware_disable().unwrap();

    return;

}
