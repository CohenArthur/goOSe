use crate::serial::ports;
use crate::serial::flags;

use crate::asm_wrappers;

use core::fmt;

pub struct Serial {
    pub port: u16,
}

impl Serial {
    // FIXME: Needed ?
    pub fn syscall_write(data: &[u8], count: usize) {
        for i in 0..count {
            asm_wrappers::outb(ports::COM1, data[i]);
        }
    }

    pub fn init(port: u16) -> Serial {
        let new_s = Serial { port: port };

        /* We initialize the Baude Rate of the port to 38400 bps */
        asm_wrappers::outb(port + flags::DLL_OFF, 0x3);
        asm_wrappers::outb(port + flags::DLH_OFF, 0x0);

        new_s
    }

    pub fn init_com1() -> Serial {
        return Serial::init(ports::COM1);
    }

    fn _write_str(&self, data: &str) {
        for byte in data.bytes() {
            if byte == '\n' as u8 {
                asm_wrappers::outb(self.port, '\r' as u8);
                asm_wrappers::outb(self.port, '\n' as u8);
            } else {
                asm_wrappers::outb(self.port, byte);
            }
        }
    }
}

impl fmt::Write for Serial {
    fn write_str(&mut self, data: &str) -> fmt::Result {
        self._write_str(data);
        Ok(())
    }
}
