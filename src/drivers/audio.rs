use crate::peripherals::gpio::GpioPin;

const CPU_FREQ_HZ: u32 = 32_000_000;

pub const NOTE_C4: u32 = 261;
pub const NOTE_D4: u32 = 293;
pub const NOTE_E4: u32 = 329;
pub const NOTE_F4: u32 = 349;
pub const NOTE_G4: u32 = 391;
pub const NOTE_A4: u32 = 440;
pub const NOTE_B4: u32 = 493;

pub const NOTE_C5: u32 = 523;
pub const NOTE_D5: u32 = 587;
pub const NOTE_E5: u32 = 659;
pub const NOTE_F5: u32 = 698;
pub const NOTE_G5: u32 = 783;
pub const NOTE_A5: u32 = 880;
pub const NOTE_B5: u32 = 987;

pub struct Speaker {
    pin: &'static GpioPin,
}

impl Speaker {
    pub const fn new(pin: &'static GpioPin) -> Self {
        Self { pin }
    }

    pub fn init(&self) {
        self.pin.configure_output();
        self.stop();
    }

    pub fn play_tone(&self, frequency_hz: u32, duration_ms: u32) {
        let period_cycles = CPU_FREQ_HZ / frequency_hz;
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
