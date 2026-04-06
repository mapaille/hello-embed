pub mod notes;

use crate::app::cancellation_token::CancellationToken;
use crate::peripherals::gpio::GpioPin;
use crate::peripherals::pwm::Pwm;
use core::ptr;

static mut DUTY_RAM: u16 = 8000;

pub struct Speaker {
    pin: &'static GpioPin,
    pwm: &'static Pwm,
}

impl Speaker {
    pub const fn new(pin: &'static GpioPin, pwm: &'static Pwm) -> Self {
        Self { pin, pwm }
    }

    pub unsafe fn init(&self) {
        self.pin.configure_speaker();

        self.pwm.tasks_stop();
        self.pwm.prescaler();
        self.pwm.mode();
        self.pwm.decoder();
        self.pwm.loop_();
        self.pwm.countertop();
        self.pwm.seq0_cnt();
        self.pwm.seq0_refresh();
        self.pwm.seq0_enddelay();
        self.pwm.seq0_ptr(ptr::addr_of!(DUTY_RAM) as usize);
        self.pwm.psel_out_0();
        self.pwm.enable();
    }

    pub fn start(&self, _cancellation_token: &CancellationToken) {
        self.pwm.tasks_seqstart0();
    }

    pub fn stop(&self) {
        self.pwm.tasks_stop();
    }
}
