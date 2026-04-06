use core::ptr::{write_volatile, NonNull};

pub static PWM0: Pwm = Pwm::new(0x4001_C000);

const TASKS_STOP_OFFSET: usize = 0x004 / 4;
const TASKS_SEQSTART0_OFFSET: usize = 0x008 / 4;
const ENABLE_OFFSET: usize = 0x500 / 4;
const MODE_OFFSET: usize = 0x504 / 4;
const COUNTERTOP_OFFSET: usize = 0x508 / 4;
const SEQ0_PTR_OFFSET: usize = 0x520 / 4;
const SEQ0_ENDDELAY_OFFSET: usize = 0x52C / 4;
const SEQ0_CNT_OFFSET: usize = 0x524 / 4;
const SEQ0_REFRESH_OFFSET: usize = 0x528 / 4;
const PRESCALER_OFFSET: usize = 0x50C / 4;
const PSEL_OUT0_OFFSET: usize = 0x560 / 4;

pub struct Pwm {
    base_addr: NonNull<u32>,
}

impl Pwm {
    pub const fn new(base_addr: u32) -> Self {
        Self {
            base_addr: NonNull::new(base_addr as *mut u32).unwrap(),
        }
    }

    pub fn tasks_stop(&self) {
        unsafe {
            write_volatile(self.base_addr.add(TASKS_STOP_OFFSET).as_ptr(), 1u32);
        }
    }

    pub fn tasks_seqstart0(&self) {
        unsafe {
            write_volatile(self.base_addr.add(TASKS_SEQSTART0_OFFSET).as_ptr(), 1u32);
        }
    }

    pub fn enable(&self) {
        unsafe {
            write_volatile(self.base_addr.add(ENABLE_OFFSET).as_ptr(), 1u32);
        }
    }

    pub fn mode(&self) {
        unsafe {
            write_volatile(self.base_addr.add(MODE_OFFSET).as_ptr(), 0u32);
        }
    }

    pub fn countertop(&self) {
        unsafe {
            write_volatile(self.base_addr.add(COUNTERTOP_OFFSET).as_ptr(), 16000u32);
        }
    }

    pub fn seq0_refresh(&self) {
        unsafe {
            write_volatile(self.base_addr.add(SEQ0_REFRESH_OFFSET).as_ptr(), 0u32);
        }
    }

    pub fn seq0_cnt(&self) {
        unsafe {
            write_volatile(self.base_addr.add(SEQ0_CNT_OFFSET).as_ptr(), 1u32);
        }
    }

    pub fn seq0_ptr(&self, duty_ptr_addr: usize) {
        unsafe {
            write_volatile(
                self.base_addr.add(SEQ0_PTR_OFFSET).as_ptr(),
                duty_ptr_addr as u32,
            );
        }
    }

    pub fn seq0_enddelay(&self) {
        unsafe {
            write_volatile(self.base_addr.add(SEQ0_ENDDELAY_OFFSET).as_ptr(), 0u32);
        }
    }

    pub fn prescaler(&self) {
        unsafe {
            write_volatile(self.base_addr.add(PRESCALER_OFFSET).as_ptr(), 0u32);
        }
    }

    pub fn psel_out_0(&self) {
        unsafe {
            write_volatile(self.base_addr.add(PSEL_OUT0_OFFSET).as_ptr(), 0u32);
        }
    }

    pub fn decoder(&self) {
        unsafe {
            write_volatile(self.base_addr.add(0x510 / 4).as_ptr(), 0u32);
        }
    }

    pub fn loop_(&self) {
        unsafe {
            write_volatile(self.base_addr.add(0x514 / 4).as_ptr(), 0u32);
        }
    }
}

unsafe impl Sync for Pwm {}
unsafe impl Send for Pwm {}
