use std::fmt;

use int_enum::IntEnum;
use enum_iterator::{all, Sequence};

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum, Sequence)]
pub enum ResistorColor {
    Black = 0,
    Brown = 1,
    Red = 2,
    Orange = 3,
    Yellow = 4,
    Green = 5,
    Blue = 6,
    Violet = 7,
    Grey = 8,
    White = 9,
}

impl fmt::Display for ResistorColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn color_to_value(color: ResistorColor) -> u32 {
    return color.int_value()
}

pub fn value_to_color_string(value: u32) -> String {
    let color = ResistorColor::from_int(value);
    if color.is_ok() {
        return color.unwrap().to_string();
    } else {
        return "value out of range".to_owned();
    }
}

pub fn colors() -> Vec<ResistorColor> {
    return all::<ResistorColor>().collect()
}
