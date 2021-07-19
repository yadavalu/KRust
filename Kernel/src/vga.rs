use volatile::Volatile;
use core::fmt;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]

pub enum Colours {
    Black,
    Blue,
    Green,
    Cyan,
    Red,
    Magenta,
    Brown,
    LightGray,
    DarkGray,
    LightBlue,
    LightGreen,
    LightCyan,
    LightRed,
    Pink,
    Yellow,
    White,
}

const BUFFERHEIGHT: usize = 25;
const BUFFERWIDTH: usize = 80;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]

struct ColourCode(u8);

impl ColourCode {
    fn new(fg: Colours, bg: Colours) -> ColourCode {
        return ColourCode((bg as u8) << 4 | (fg as u8));
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]

struct Screenchar {
    ascii_char: u8,
    colour_code: ColourCode,
}


#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<Screenchar>; BUFFERWIDTH]; BUFFERHEIGHT],
}

pub struct Writer {
    column_pos: usize,
    colour_code: ColourCode,
    buffer: &'static mut Buffer,
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

pub fn KRustWriter() -> Writer {
    use core::fmt::Write;
    let mut writer = Writer {
        column_pos: BUFFERHEIGHT,
        colour_code: ColourCode::new(Colours::White, Colours::Black),
        buffer: unsafe {
            &mut *(0xb8000 as *mut Buffer)
        },
    };

    return writer;
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    KRustWriter().write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => ($crate::vga::_error(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! errorln {
    () => ($crate::error!("\n"));
    ($($arg:tt)*) => ($crate::error!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _error(args: fmt::Arguments) {
    use core::fmt::Write;

    let mut writer = Writer {
        column_pos: BUFFERHEIGHT,
        colour_code: ColourCode::new(Colours::Red, Colours::Black),
        buffer: unsafe {
            &mut *(0xb8000 as *mut Buffer)
        },
    };

    writer.write_fmt(args).unwrap();
}
