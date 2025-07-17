use crate::drivers::screens::frames::frame;

pub const SYMBOL_EQUAL: frame::Frame<5, 5> = frame::Frame(&[
    [false, false, false, false, false],
    [true, true, true, true, true],
    [false, false, false, false, false],
    [true, true, true, true, true],
    [false, false, false, false, false],
]);

pub const SYMBOL_PLUS: frame::Frame<5, 5> = frame::Frame(&[
    [false, false, true, false, false],
    [false, false, true, false, false],
    [true, true, true, true, true],
    [false, false, true, false, false],
    [false, false, true, false, false],
]);
