use crate::drivers::display::frames::frame::Frame;

pub const SYMBOL_EQUAL: Frame<5, 5> = Frame(&[
    [false, false, false, false, false],
    [true, true, true, true, true],
    [false, false, false, false, false],
    [true, true, true, true, true],
    [false, false, false, false, false],
]);

pub const SYMBOL_PLUS: Frame<5, 5> = Frame(&[
    [false, false, true, false, false],
    [false, false, true, false, false],
    [true, true, true, true, true],
    [false, false, true, false, false],
    [false, false, true, false, false],
]);
