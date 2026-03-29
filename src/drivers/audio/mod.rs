pub mod notes;

use crate::peripherals::gpio::GpioPin;

pub struct Speaker {
    pin: &'static GpioPin,
    cpu_freq_hz: u32,
}

impl Speaker {
    pub const fn new(pin: &'static GpioPin, cpu_freq_hz: u32) -> Self {
        Self { pin, cpu_freq_hz }
    }

    pub fn init(&self) {
        self.pin.configure_output();
        self.stop();
    }

    pub fn play_tone(&self, frequency_hz: u32, duration_ms: u32) {
        let period_cycles = self.cpu_freq_hz / frequency_hz;
        let half_period_cycles = period_cycles / 2;
        let num_periods = u64::from(frequency_hz) * u64::from(duration_ms) / 1000;

        for _ in 0..num_periods {
            self.pin.set_high();
            delay_cycles(half_period_cycles);
            self.pin.set_low();
            delay_cycles(half_period_cycles);
        }

        self.stop();
    }

    pub fn beep(&self) {
        self.play_tone(2700, 100);
    }

    #[inline]
    pub fn play(&self) {
        self.pin.set_high();
    }

    #[inline]
    pub fn stop(&self) {
        self.pin.set_low();
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
