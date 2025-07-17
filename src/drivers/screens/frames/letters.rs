use crate::drivers::screens::frames::frame;

pub const LETTER_E: frame::Frame<5, 5> = frame::Frame(&[
    [true, true, true, true, false],
    [true, false, false, false, false],
    [true, true, true, false, false],
    [true, false, false, false, false],
    [true, true, true, true, false],
]);

pub const LETTER_M: frame::Frame<5, 5> = frame::Frame(&[
    [true, false, false, false, true],
    [true, true, false, true, true],
    [true, false, true, false, true],
    [true, false, false, false, true],
    [true, false, false, false, true],
]);
