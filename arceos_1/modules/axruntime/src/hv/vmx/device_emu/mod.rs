
mod i8259_pic;
mod lapic;
mod uart16550;

extern crate alloc;
use alloc::{sync::Arc, vec, vec::Vec};
use hypercraft::HyperResult;

pub use self::lapic::VirtLocalApic;
use self::uart16550::Uart16550;
use core::any::Any;

pub trait PortIoDevice: Any + Send + Sync {
    fn port_range(&self) -> core::ops::Range<u16>;
    fn read(&self, port: u16, access_size: u8) -> HyperResult<u32>;
    fn write(&self, port: u16, access_size: u8, value: u32) -> HyperResult;
}

impl dyn PortIoDevice {
    fn as_any_arc(self: Arc<Self>) -> Arc<dyn Any> {
        return self;
    }
    fn downcast_arc<T: PortIoDevice>(self: Arc<Self>) -> Option<Arc<T>> {
        // info!("count before: {}", Arc::strong_count(&self));
        let x = self.as_any_arc();
        // let y = x.clone();

        if x.is::<T>() {
            // info!("x.is::<{}>", core::any::type_name::<T>());
            // info!("count in1: {}", Arc::strong_count(&y));
            // into_raw不会改变引用计数
            let p = Arc::into_raw(x);
            // info!("count in2: {}", Arc::strong_count(&y));
            let new = unsafe { Arc::from_raw(p as *const T) };
            // info!("count after: {}", Arc::strong_count(&new));
            return Some(new);
        }
        info!("x.is not <{}>", core::any::type_name::<T>());
        return None;
    }
}

pub struct VirtDeviceList {
    port_io_devices: Vec<Arc<dyn PortIoDevice>>,
}

impl VirtDeviceList {
    pub fn find_port_io_device(&self, port: u16) -> Option<&Arc<dyn PortIoDevice>> {
        self.port_io_devices
            .iter()
            .find(|dev| dev.port_range().contains(&port))
    }

    pub fn find_uart(&self, port: u16) -> Option<Arc<Uart16550>> {
        if let Some(dev) = self.find_port_io_device(port) {
            let p = dev.clone().downcast_arc::<Uart16550>().unwrap();
            // info!("count after: {}", Arc::strong_count(&p));
            // info!("find!!! {}",p.baud_rate);
            return Some(p);
        }
        None
    }
}

pub const MAX_VMS: usize = 2;

lazy_static::lazy_static! {
    static ref VIRT_DEVICES : Vec<VirtDeviceList> = {
        let mut temp = Vec::new();
        for i in 0..MAX_VMS {
            temp.push(VirtDeviceList {
                port_io_devices: vec![
                    Arc::new(uart16550::Uart16550::new(0x3f8, i)), // COM1
                    Arc::new(i8259_pic::I8259Pic::new(0x20)), // PIC1
                    Arc::new(i8259_pic::I8259Pic::new(0xA0)), // PIC2
                ],
            });
        };
        temp
    };
}

pub fn all_virt_devices(id:usize) -> &'static VirtDeviceList {
    &VIRT_DEVICES[id]
}
