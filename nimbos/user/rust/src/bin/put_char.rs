#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::write;
use user_lib::set_uart_int_en;
#[no_mangle]
pub fn main() -> i32 {
    set_uart_int_en(0x01);
    write(1, "COM1 writes OK!!!!\n".as_bytes());
    set_uart_int_en(0x00);
    0
}