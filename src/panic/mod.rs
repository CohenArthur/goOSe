#[cfg(not(test))]
use core::panic::PanicInfo;
#[cfg(not(test))]
use crate::println;

#[cfg(not(test))]
#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);

    loop {}
}

#[cfg(target_arch = "riscv64")]
#[no_mangle]
pub fn abort() {
    panic!("Jump !");
}
