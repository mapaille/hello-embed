use crate::traits::Register;
use core::ptr::NonNull;

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

    pub fn disable(&self) {
        self.write_reg(0x500, 0u32);
    }

    pub fn mode(&self) {
        self.write_reg(0x504, 0u32); // Up counter
    }

    pub fn countertop(&self, value: u32) {
        self.write_reg(0x508, value);
    }

    pub fn decoder_common(&self) {
        self.write_reg(0x510, 0u32);
    }

    pub fn loop_disable(&self) {
        self.write_reg(0x514, 0u32);
    }

    /// SHORTS: automatically restart SEQ[0] on SEQEND0. Simpler continuous tone without relying on LOOP.
    pub fn shorts_seqend0_seqstart0(&self) {
        self.write_reg(0x200, 1u32 << 4); // SEQEND0_SEQSTART0 (nRF52 PS 6.15.4)
    }

    pub fn seq0_ptr(&self, ptr: *const u16) {
        self.write_reg(0x520, ptr as u32);
    }

    pub fn seq0_cnt(&self, count: u32) {
        self.write_reg(0x524, count);
    }

    pub fn seq0_refresh(&self, value: u32) {
        self.write_reg(0x528, value);
    }

    pub fn prescaler(&self) {
        self.write_reg(0x50C, 4u32); // DIV_16 → 1 MHz PWM clock
    }

    pub fn seq0_enddelay(&self) {
        self.write_reg(0x52C, 0u32);
    }

    pub fn seq1_ptr(&self, ptr: *const u16) {
        self.write_reg(0x540, ptr as u32);
    }

    pub fn seq1_cnt(&self, count: u32) {
        self.write_reg(0x544, count);
    }

    pub fn seq1_refresh(&self, value: u32) {
        self.write_reg(0x548, value);
    }

    pub fn seq1_enddelay(&self) {
        self.write_reg(0x54C, 0u32);
    }

    pub fn psel_out_0(&self, pin: u32) {
        self.write_reg(0x560, pin);
    }

    /// Disconnect all PSEL.OUT channels (write 0xFFFFFFFF = disconnected)
    pub fn psel_disconnect_all(&self) {
        self.write_reg(0x560, 0xFFFF_FFFFu32);
        self.write_reg(0x564, 0xFFFF_FFFFu32);
        self.write_reg(0x568, 0xFFFF_FFFFu32);
        self.write_reg(0x56C, 0xFFFF_FFFFu32);
    }
}

impl Register for Pwm {
    fn base_addr(&self) -> NonNull<u8> {
        self.base_addr
    }
}
