use crate::drivers::screen::frames::frame::Frame;

pub const IMAGE_HEART: Frame<5, 5> = Frame(&[
    [false, true, false, true, false],
    [true, true, true, true, true],
    [true, true, true, true, true],
    [false, true, true, true, false],
    [false, false, true, false, false],
]);
