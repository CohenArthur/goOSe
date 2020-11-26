//! RISCV64 abstractions layer

pub mod exceptions;
pub mod interrupts;
pub mod trap;

use super::*;
use crate::kmain;
use crate::println;
use crate::utils::external_symbol_address;
use cfg_if::cfg_if;
use core::slice;
use fdt_rs::base::DevTree;
use fdt_rs::prelude::FallibleIterator;

/// UART0 address
pub static UART0: usize = 0x10000000;

cfg_if! {
    if #[cfg(test)] {
        use qemu_exit;
        pub static QEMU_EXIT: qemu_exit::RISCV64 = qemu_exit::RISCV64::new(0x100000);
    }
}

/// Entry point of the kernel. Setup the stack address, call `init` and call `kmain`.
#[no_mangle]
unsafe extern "C" fn kstart() -> ! {
    #[cfg(target_arch = "riscv64")]
    asm!(
        "la sp, STACK_START
        mv a0, a1
        call {}",
        sym init,
    );

    unreachable!();
}

/// Initialize proper rust execution environement.
#[no_mangle]
extern "C" fn init(fdt_addr: usize) -> ! {
    println!("\nRISCV64 Init"); // Separation from OpenSBI boot info
    println!("Fdt = {:#x}", fdt_addr);

    clear_bss();

    let fdt_slice: &[u8] =
        unsafe { slice::from_raw_parts_mut(fdt_addr as *mut u8, DevTree::MIN_HEADER_SIZE) };
    let fdt_size = match unsafe { DevTree::read_totalsize(fdt_slice) } {
        Ok(size) => size,
        Err(_) => panic!("There is no FDT at {:#x}", fdt_addr),
    };
    let fdt_slice: &[u8] = unsafe { slice::from_raw_parts_mut(fdt_addr as *mut u8, fdt_size) };

    let fdt = unsafe { DevTree::new(fdt_slice).unwrap() };
    println!("\nDevice tree:");
    let mut nodes = fdt.nodes();
    nodes.for_each(|node| {
        println!("Node: {}", node.name().unwrap());
        Ok(())
    });
    println!("");

    trap::init();
    interrupts::init();
    println!("Interrupts State: {:?}", interrupts::state());

    kmain();
}

/// Clear the BSS. Should already be done by some bootloaders but just in case.
fn clear_bss() {
    let _bss_start = unsafe { external_symbol_address(&BSS_START) };
    let _bss_end = unsafe { external_symbol_address(&BSS_END) };

    for addr in _bss_start.._bss_end {
        let addr = addr as *mut u8;
        unsafe {
            *addr = 0;
        }
    }

    println!("BSS cleared ({:#x} -> {:#x})", _bss_start, _bss_end);
}

/// Some architecture (x86...) have a specific instruction to write on some specific
/// address (IO ports). RISCV does not, so this is just a stub for writing at
/// a specified address.
pub fn outb(addr: usize, byte: u8) {
    let addr = addr as *mut u8;
    unsafe {
        *addr = byte;
    }
}
