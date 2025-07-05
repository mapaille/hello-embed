use crate::drivers::screen::frames::{images, letters, symbols};
use crate::drivers::screen::frames::frame::Frame;

pub struct Animation<const X: usize, const Y: usize, const SIZE: usize> {
    pub frames: &'static[&'static Frame<X, Y>; SIZE],
}

pub const ANIMATION_LOVE: Animation<5, 5, 5> = Animation {
    frames: &[
        &letters::LETTER_E,
        &symbols::SYMBOL_PLUS,
        &letters::LETTER_M,
        &symbols::SYMBOL_EQUAL,
        &images::IMAGE_HEART,
    ],
};

pub const ANIMATION_HEARTBEAT: Animation<5, 5, 4> = Animation {
    frames: &[
        &Frame(&[
            [false, true, false, true, false],
            [true, true, true, true, true],
            [true, true, true, true, true],
            [false, true, true, true, false],
            [false, false, true, false, false],
        ]),
        &Frame(&[
            [false, false, false, false, false],
            [false, true, false, true, false],
            [false, true, true, true, false],
            [false, false, true, false, false],
            [false, false, false, false, false],
        ]),
        &Frame(&[
            [false, false, false, false, false],
            [false, true, false, true, false],
            [false, true, true, true, false],
            [false, false, true, false, false],
            [false, false, false, false, false],
        ]),
        &Frame(&[
            [false, false, false, false, false],
            [false, true, false, true, false],
            [false, true, true, true, false],
            [false, false, true, false, false],
            [false, false, false, false, false],
        ])
    ],
};

pub const ANIMATION_LOADING: Animation<5, 5, 16> = Animation {
    frames: &[
        &Frame(&[
           [true, false, false, false, false],
           [false, false, false, false, false],
           [false, false, false, false, false],
           [false, false, false, false, false],
           [false, false, false, false, false],
        ]),
        &Frame(&[
            [false, true, false, false, false],
            [false, false, false, false, false],
            [false, false, false, false, false],
            [false, false, false, false, false],
            [false, false, false, false, false],
        ]),
        &Frame(&[
            [false, false, true, false, false],
            [false, false, false, false, false],
            [false, false, false, false, false],
            [false, false, false, false, false],
            [false, false, false, false, false],
        ]),
        &Frame(&[
            [false, false, false, true, false],
            [false, false, false, false, false],
            [false, false, false, false, false],
            [false, false, false, false, false],
            [false, false, false, false, false],
        ]),
        &Frame(&[
            [false, false, false, false, true],
            [false, false, false, false, false],
            [false, false, false, false, false],
            [false, false, false, false, false],
            [false, false, false, false, false],
        ]),
        &Frame(&[
            [false, false, false, false, false],
            [false, false, false, false, true],
            [false, false, false, false, false],
            [false, false, false, false, false],
            [false, false, false, false, false],
        ]),
        &Frame(&[
            [false, false, false, false, false],
            [false, false, false, false, false],
            [false, false, false, false, true],
            [false, false, false, false, false],
            [false, false, false, false, false],
        ]),
        &Frame(&[
            [false, false, false, false, false],
            [false, false, false, false, false],
            [false, false, false, false, false],
            [false, false, false, false, true],
            [false, false, false, false, false],
        ]),
        &Frame(&[
            [false, false, false, false, false],
            [false, false, false, false, false],
            [false, false, false, false, false],
            [false, false, false, false, false],
            [false, false, false, false, true],
        ]),
        &Frame(&[
            [false, false, false, false, false],
            [false, false, false, false, false],
            [false, false, false, false, false],
            [false, false, false, false, false],
            [false, false, false, true, false],
        ]),
        &Frame(&[
            [false, false, false, false, false],
            [false, false, false, false, false],
            [false, false, false, false, false],
            [false, false, false, false, false],
            [false, false, true, false, false],
        ]),
        &Frame(&[
            [false, false, false, false, false],
            [false, false, false, false, false],
            [false, false, false, false, false],
            [false, false, false, false, false],
            [false, true, false, false, false],
        ]),
        &Frame(&[
            [false, false, false, false, false],
            [false, false, false, false, false],
            [false, false, false, false, false],
            [false, false, false, false, false],
            [true, false, false, false, false],
        ]),
        &Frame(&[
            [false, false, false, false, false],
            [false, false, false, false, false],
            [false, false, false, false, false],
            [true, false, false, false, false],
            [false, false, false, false, false],
        ]),
        &Frame(&[
            [false, false, false, false, false],
            [false, false, false, false, false],
            [true, false, false, false, false],
            [false, false, false, false, false],
            [false, false, false, false, false],
        ]),
        &Frame(&[
            [false, false, false, false, false],
            [true, false, false, false, false],
            [false, false, false, false, false],
            [false, false, false, false, false],
            [false, false, false, false, false],
        ]),
    ],
};
