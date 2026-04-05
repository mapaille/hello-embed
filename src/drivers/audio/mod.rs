pub mod notes;

use crate::app::cancellation_token::CancellationToken;
use crate::peripherals::gpio::GpioPin;
use crate::peripherals::pwm::Pwm;
use crate::timing::wait_ticks;

pub struct Speaker {
    pin: &'static GpioPin,
    pwm: &'static Pwm,
}

impl Speaker {
    pub const fn new(pin: &'static GpioPin, pwm: &'static Pwm) -> Self {
        Self { pin, pwm }
    }

    pub fn init(&self) {
        self.pin.configure_output();
        self.pin.set_low();

        self.pwm.psel_out_0();
        self.pwm.enable();
        self.pwm.mode();
        self.pwm.prescaler();
        self.pwm.countertop();
        self.pwm.seq0_ptr();
        self.pwm.seq0_cnt();
        self.pwm.seq0_refresh();
    }

    pub fn beep(&self, cancellation_token: &CancellationToken) {
        self.pwm.prescaler();
        self.pwm.tasks_seqstart0();
        wait_ticks(9800, cancellation_token);
        self.pwm.tasks_stop();
    }
}

#[inline(never)]
#[allow(unused_assignments)]
fn delay_cycles(mut cycles: u32) {
    unsafe {
        core::arch::asm!(
        "1:",
        "subs {0}, {0}, #1",
        "bne 1b",
        inout(reg) cycles,
        );
    }
}
