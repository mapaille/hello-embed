#![allow(dead_code)]

use crate::pin::Pin;

pub const BASE: u32 = 0x5000_0000;

pub const RING0: Pin = Pin {
    base: BASE,
    offset: 0,
};

pub const COL2: Pin = Pin {
    base: BASE,
    offset: 11,
};

pub const BTN_A: Pin = Pin {
    base: BASE,
    offset: 14,
};

pub const ROW3: Pin = Pin {
    base: BASE,
    offset: 15,
};

pub const ROW5: Pin = Pin {
    base: BASE,
    offset: 19,
};

pub const ROW1: Pin = Pin {
    base: BASE,
    offset: 21,
};

pub const ROW2: Pin = Pin {
    base: BASE,
    offset: 22,
};

pub const BTN_B: Pin = Pin {
    base: BASE,
    offset: 23,
};

pub const ROW4: Pin = Pin {
    base: BASE,
    offset: 24,
};

pub const COL1: Pin = Pin {
    base: BASE,
    offset: 28,
};

pub const COL5: Pin = Pin {
    base: BASE,
    offset: 30,
};

pub const COL3: Pin = Pin {
    base: BASE,
    offset: 31,
};
