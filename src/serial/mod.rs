use core::fmt;

pub mod serial;
pub mod ports;
pub mod flags;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::serial::print_fmt(format_args!($($arg)*)))
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn print_fmt(args: fmt::Arguments) {
    use core::fmt::Write;

    // FIXME: Change from mut static to lazy_static! or Mutex
    unsafe {
        ports::SERIAL_PORT.write_fmt(args).unwrap();
    }
}
