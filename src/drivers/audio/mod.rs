pub mod notes;

use core::ptr::{read_volatile, write_volatile};

use crate::peripherals::gpio::GpioPin;
use crate::peripherals::pwm::Pwm;
use crate::traits::Register;

/// PWM clock frequency in Hz (16 MHz base / `DIV_16` prescaler)
const PWM_CLOCK_HZ: u32 = 1_000_000;

/// Default tone frequency in Hz (A4)
const DEFAULT_FREQ_HZ: u32 = 440;

/// Word-aligned buffer for PWM `EasyDMA`.
/// The repr(align(4)) guarantees the buffer sits on a 4-byte boundary in RAM.
#[repr(C, align(4))]
struct DmaBuffer {
    data: [u16; 1],
}

static mut DUTY_CYCLE: DmaBuffer = DmaBuffer { data: [0] };

pub struct Speaker {
    pin: &'static GpioPin,
    pwm: &'static Pwm,
}

impl Speaker {
    pub const fn new(pin: &'static GpioPin, pwm: &'static Pwm) -> Self {
        Self { pin, pwm }
    }

    #[allow(clippy::cast_possible_truncation)]
    pub fn init(&self) {
        self.pwm.disable();
        self.pin.set_low();
        self.pin.configure_speaker();
        self.pin.set_high();
        self.pwm.psel_disconnect_all();
        self.pwm.psel_out_0(self.pin.pin_number());
        self.pwm.mode();
        self.pwm.prescaler();
        self.pwm.decoder_common();
        self.pwm.loop_disable();
        self.pwm.shorts_seqend0_seqstart0();

        let countertop = PWM_CLOCK_HZ / DEFAULT_FREQ_HZ;
        self.pwm.countertop(countertop);
        let duty = (countertop / 2) as u16;
        self.set_duty(duty);

        let duty_ptr: *const u16 = unsafe { (&raw const DUTY_CYCLE.data).cast::<u16>() };
        self.pwm.seq0_ptr(duty_ptr);
        self.pwm.seq0_cnt(1);
        self.pwm.seq0_refresh(0);
        self.pwm.seq0_enddelay();

        self.pwm.seq1_ptr(duty_ptr);
        self.pwm.seq1_cnt(1);
        self.pwm.seq1_refresh(0);
        self.pwm.seq1_enddelay();

        self.pwm.enable();

        debug_assert_eq!(self.pwm.read_reg(0x50C), 4);
        debug_assert_eq!(self.pwm.read_reg(0x200), 1u32 << 4);
        debug_assert_eq!(self.pwm.read_reg(0x500), 1);
        let ct = self.pwm.read_reg(0x508);
        debug_assert!(ct > 100 && ct < 10000);
    }

    pub fn stop(&self) {
        self.pwm.tasks_stop();
    }

    #[allow(clippy::cast_possible_truncation)]
    pub fn set_frequency(&self, freq_hz: u32) {
        debug_assert!((100..=20000).contains(&freq_hz));
        let countertop = PWM_CLOCK_HZ / freq_hz;
        debug_assert!((20..=20000).contains(&countertop));
        self.pwm.countertop(countertop);
        let duty = (countertop / 2) as u16;
        self.set_duty(duty);
        self.pwm.tasks_seqstart0();
    }

    /// Write duty cycle value to the `EasyDMA` buffer.
    #[allow(clippy::unused_self)]
    pub fn set_duty(&self, duty: u16) {
        unsafe {
            let ptr: *mut u16 = (&raw mut DUTY_CYCLE.data).cast::<u16>();
            write_volatile(ptr, duty);
            debug_assert_eq!(read_volatile(ptr), duty);
        }
    }
}

unsafe impl Sync for Speaker {}
unsafe impl Send for Speaker {}
