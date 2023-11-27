use crate::drivers::uart::set_int_en;
pub fn sys_uart_int_en(c: u8) -> isize{
    set_int_en(c);
    0
}