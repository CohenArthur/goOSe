use crate::vga::attribute::Attribute;

const VGA_BUF_W: usize = 80;
const VGA_BUF_H: usize = 25;
const VGA_AREA: usize = VGA_BUF_H * VGA_BUF_W;

static VGA_BUFFER_ADDR: usize = 0xb8000;

fn write_byte(data: u8, index: usize) {
    unsafe {
        core::ptr::write_volatile((VGA_BUFFER_ADDR + index) as *mut u8, data);
    }
}

pub struct Buffer {
    size: usize,
    characters: [u8; VGA_AREA],
    attributes: [Attribute; VGA_AREA],
}

impl Buffer {
    pub fn new() -> Buffer {
        let new_buf = Buffer {
            size: 0,
            characters: [0x0; VGA_AREA],
            attributes: [*Attribute::new().with_fg_color(0xa); VGA_AREA],
        };

        new_buf
    }

    pub fn from_str(data: &str) -> Buffer {
        let mut new_buf = Buffer::new();

        let bytes = data.as_bytes();

        for i in 0..data.len() {
            new_buf.append(bytes[i]);
        }

        new_buf
    }

    pub fn append(&mut self, data: u8) -> &Buffer {
        match data {
            // FIXME: Fix 'magical' values
            invalid_byte if invalid_byte >= 0x7e => {
                self.append('?' as u8);
            }
            b'\n' => {
                self.new_line();
            }
            _ => {
                self.characters[self.size] = data;
                self.size += 1;
            }
        };

        self
    }

    pub fn new_line(&mut self) -> &Buffer {
        let cells_to_fill = 80 - self.size % 80;

        for i in 0..cells_to_fill {
            self.append(0);
        }

        self.size += cells_to_fill;

        self
    }

    pub fn reset(&mut self) -> &Buffer {
        for i in 0..self.size {
            self.characters[i] = 0x0;
            self.attributes[i] = Attribute::new();
        }

        self
    }

    pub fn write(&self) -> usize {
        let mut idx = 0;

        for tuple in self.characters.iter().zip(self.attributes.iter()) {
            let (symbol, attr) = tuple;

            write_byte(*symbol, idx);
            write_byte(attr.get_representation(), idx + 1);

            idx += 2;
        }

        return idx;
    }

    pub fn flush(&mut self) -> usize {
        let written = self.write();

        self.reset();

        return written;
    }
}