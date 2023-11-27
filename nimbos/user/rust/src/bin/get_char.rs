#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::read;
use user_lib::set_uart_int_en;

const LF: u8 = b'\n';

#[no_mangle]
pub fn main() -> i32 {
    set_uart_int_en(0x01);
    let mut buf = [0u8; 1];
    let mut ret = [0u8; 32];
    let mut i = 0;
    while read(0, &mut buf) > 0 && i < 32{
        ret[i] = buf[0];
        i = i+1;
        if buf[0] == LF {
            break;
        }
    }
    set_uart_int_en(0x00);
    println!("the result: ");
    for i in 0..32 {
        println!("{}: {}",i ,ret[i] as char);
    }
    0
}