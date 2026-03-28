use crate::peripherals::gpio::GpioPin;

pub struct Speaker {
    pin: &'static GpioPin,
}

impl Speaker {
    pub const fn new(pin: &'static GpioPin) -> Self {
        Self { pin }
    }

    pub fn init(&self) {
        self.pin.configure_output();
        self.stop(); // make sure it is silent
    }

    /// Blocking test tone – plays for `duration_ms` then automatically stops
    /// Generic frequency so you can experiment, but 2700 Hz is the sweet spot
    pub fn play_tone(&self, frequency_hz: u32, duration_ms: u32) {
        // HALF speed on the crystal → 32 MHz
        const CPU_FREQ_HZ: u32 = 32_000_000;

        let period_cycles = CPU_FREQ_HZ / frequency_hz;
        let half_period_cycles = period_cycles / 2; // 50 % duty

        // How many full periods we need for the requested duration
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

    pub fn play(&self) {
        self.pin.set_high();
    }

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
