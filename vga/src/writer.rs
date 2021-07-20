use core::fmt;

use crate::buffer::{Buffer, Screenchar, BUFFERHEIGHT, BUFFERWIDTH};
use crate::colours::ColourCode;

pub struct Writer {
    pub column_pos: usize,
    pub colour_code: ColourCode,
    pub buffer: &'static mut Buffer,
}

impl Writer {
    fn newline(&mut self) {
        for row in 1..BUFFERHEIGHT {
            for column in 0..BUFFERWIDTH {
                let character = self.buffer.chars[row][column].read();
                self.buffer.chars[row - 1][column].write(character);
            }
        }

        self.clear_row(BUFFERHEIGHT - 1);
        self.column_pos = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = Screenchar {
            ascii_char: b' ',
            colour_code: self.colour_code,
        };

        for column in 0..BUFFERWIDTH {
            self.buffer.chars[row][column].write(blank);
        }
    }

    pub fn print_bytes(&mut self, byte: u8) {
        match byte {
            b'\n' => self.newline(),
            // TODO
            b' ' => {
                self.column_pos += 1;
                if self.column_pos >= BUFFERHEIGHT {
                    self.newline();
                }
            }
            byte => {
                if self.column_pos >= BUFFERHEIGHT {
                    self.newline();
                }
    
                let row = BUFFERHEIGHT - 1;
                let column = self.column_pos;
    
                let colour_code = self.colour_code;
    
                self.buffer.chars[row][column].write(Screenchar { 
                    ascii_char: byte, colour_code
                });

                self.column_pos += 1;
            }
        }
    }

    pub fn print_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x28..=0x7e | b'\n' => self.print_bytes(byte),
                _ => self.print_bytes(0xfe),
            }
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.print_string(s);
        return Ok(());
    }
}
