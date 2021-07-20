use core::fmt;

use crate::buffer::{Buffer, BUFFERHEIGHT};
use crate::colours::{Colours, ColourCode};
use crate::writer::{Writer};

pub fn KRustWriter() -> Writer {
    //use core::fmt::Write;
    let writer = Writer {
        column_pos: BUFFERHEIGHT,
        colour_code: ColourCode::new(Colours::White, Colours::Black),
        buffer: unsafe {
            &mut *(0xb8000 as *mut Buffer)
        },
    };

    return writer;
}    

#[macro_export]
macro_rules! write {
    ($($arg:tt)*) => ($crate::vga::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! writeline {
    () => ($crate::write!("\n"));
    ($($arg:tt)*) => ($crate::write!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    KRustWriter().write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! _error {
    ($($arg:tt)*) => ($crate::vga::__error(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! error {
    () => ($crate::_error!("\n"));
    ($($arg:tt)*) => ($crate::_error!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn __error(args: fmt::Arguments) {
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
