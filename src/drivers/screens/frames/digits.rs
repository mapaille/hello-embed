use crate::drivers::screens::frames::frame::Frame;

pub const DIGIT_0: Frame<5, 5> = Frame(&[
    [false, true, true, true, false],
    [false, true, false, true, false],
    [false, true, false, true, false],
    [false, true, false, true, false],
    [false, true, true, true, false],
]);

pub const DIGIT_1: Frame<5, 5> = Frame(&[
    [false, false, false, true, false],
    [false, false, false, true, false],
    [false, false, false, true, false],
    [false, false, false, true, false],
    [false, false, false, true, false],
]);

pub const DIGIT_2: Frame<5, 5> = Frame(&[
    [false, true, true, true, false],
    [false, false, false, true, false],
    [false, true, true, true, false],
    [false, true, false, false, false],
    [false, true, true, true, false],
]);

pub const DIGIT_3: Frame<5, 5> = Frame(&[
    [false, true, true, true, false],
    [false, false, false, true, false],
    [false, true, true, true, false],
    [false, false, false, true, false],
    [false, true, true, true, false],
]);

pub const DIGIT_4: Frame<5, 5> = Frame(&[
    [false, true, false, true, false],
    [false, true, false, true, false],
    [false, true, true, true, false],
    [false, false, false, true, false],
    [false, false, false, true, false],
]);

pub const DIGIT_5: Frame<5, 5> = Frame(&[
    [false, true, true, true, false],
    [false, true, false, false, false],
    [false, true, true, true, false],
    [false, false, false, true, false],
    [false, true, true, true, false],
]);

pub const DIGIT_6: Frame<5, 5> = Frame(&[
    [false, true, true, true, false],
    [false, true, false, false, false],
    [false, true, true, true, false],
    [false, true, false, true, false],
    [false, true, true, true, false],
]);

pub const DIGIT_7: Frame<5, 5> = Frame(&[
    [false, true, true, true, false],
    [false, false, false, true, false],
    [false, false, false, true, false],
    [false, false, false, true, false],
    [false, false, false, true, false],
]);

pub const DIGIT_8: Frame<5, 5> = Frame(&[
    [false, true, true, true, false],
    [false, true, false, true, false],
    [false, true, true, true, false],
    [false, true, false, true, false],
    [false, true, true, true, false],
]);

pub const DIGIT_9: Frame<5, 5> = Frame(&[
    [false, true, true, true, false],
    [false, true, false, true, false],
    [false, true, true, true, false],
    [false, false, false, true, false],
    [false, false, false, true, false],
]);
