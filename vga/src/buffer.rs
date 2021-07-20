use volatile::Volatile;

use crate::colours::ColourCode;

pub const BUFFERHEIGHT: usize = 25;
pub const BUFFERWIDTH: usize = 80;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct Screenchar {
    pub ascii_char: u8,
    pub colour_code: ColourCode,
}


#[repr(transparent)]
pub struct Buffer {
    pub chars: [[Volatile<Screenchar>; BUFFERWIDTH]; BUFFERHEIGHT],
}
