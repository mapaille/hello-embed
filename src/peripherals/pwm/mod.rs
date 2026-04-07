use core::ptr::NonNull;
use crate::traits::Register;

pub const PWM0: Pwm = Pwm::new(0x4001_C000);

pub struct Pwm {
    base_addr: NonNull<u8>,
}

impl Pwm {
    pub const fn new(base_addr: u32) -> Self {
        Self {
            base_addr: unsafe { NonNull::new_unchecked(base_addr as *mut u8) },
        }
    }

    pub fn tasks_stop(&self) {
        self.write_reg(0x004, 1u32);
    }

    pub fn tasks_seqstart0(&self) {
        self.write_reg(0x008, 1u32);
    }

    pub fn enable(&self) {
        self.write_reg(0x500, 1u32);
    }

    pub fn mode(&self) {
        self.write_reg(0x504, 0u32);
    }

    pub fn countertop(&self, value: u32) {
        self.write_reg(0x508, value);
    }

    pub fn decoder_common(&self) {
        self.write_reg(0x510, 0u32);
    }

    pub fn loop_max(&self) {
        self.write_reg(0x514, 0xFFFF_FFFFu32);
    }

    pub fn seq0_ptr(&self, ptr: *mut [u16;1]) {
        self.write_reg(0x520, ptr as u32);
    }

    pub fn seq0_cnt(&self) {
        self.write_reg(0x524, 1u32);
    }

    pub fn seq0_refresh(&self) {
        self.write_reg(0x528, 0u32);
    }

    pub fn prescaler(&self) {
        self.write_reg(0x50C, 7u32);
    }

    pub fn seq0_enddelay(&self) {
        self.write_reg(0x52C, 0u32);
    }

    pub fn psel_out_0(&self) {
        self.write_reg(0x560, 0u32);
    }
}

impl Register for Pwm {
    fn base_addr(&self) -> NonNull<u8> {
        self.base_addr
    }
}
