use crate::serial::serial;

pub static COM1: u16 = 0x3f8;

pub static mut SERIAL_PORT: serial::Serial = serial::Serial { port: COM1 };
