use core::ptr::NonNull;

const PWM0: Pwm = Pwm::new(0x4001_C000);

pub struct Pwm {
    base_addr: NonNull<u32>,
}

impl Pwm {
    pub const fn new(base_addr: u32) -> Self {
        Self {
            base_addr: NonNull::new(base_addr as *mut u32).unwrap(),
        }
    }
}
