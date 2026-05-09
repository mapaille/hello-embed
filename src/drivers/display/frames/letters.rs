use crate::drivers::display::frames::frame::Frame;

pub const LETTER_E: Frame<5, 5> = Frame(&[
    [true, true, true, true, false],
    [true, false, false, false, false],
    [true, true, true, false, false],
    [true, false, false, false, false],
    [true, true, true, true, false],
]);

pub const LETTER_M: Frame<5, 5> = Frame(&[
    [true, false, false, false, true],
    [true, true, false, true, true],
    [true, false, true, false, true],
    [true, false, false, false, true],
    [true, false, false, false, true],
]);

pub const LETTER_T: Frame<5, 5> = Frame(&[
    [true, true, true, true, true],
    [false, false, true, false, false],
    [false, false, true, false, false],
    [false, false, true, false, false],
    [false, false, true, false, false],
]);

pub const LETTER_X: Frame<5, 5> = Frame(&[
    [true, false, false, false, true],
    [false, true, false, true, false],
    [false, false, true, false, false],
    [false, true, false, true, false],
    [true, false, false, false, true],
]);
