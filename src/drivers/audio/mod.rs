pub mod notes;

use core::ptr::write_volatile;

use crate::app::cancellation_token::CancellationToken;
use crate::peripherals::gpio::GpioPin;
use crate::peripherals::pwm::Pwm;

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

    #[allow(clippy::cast_possible_truncation)] // countertop/2 always fits u16
    pub fn init(&self) {
        // === 1. Disable PWM before configuring (required by nRF52833 spec) ===
        self.pwm.disable();

        // === 2. GPIO setup: high-drive output for speaker ===
        self.pin.configure_speaker();

        // === 3. Connect PWM channel 0 to the speaker pin ===
        // First disconnect all channels, then connect channel 0 to P0.00
        self.pwm.psel_disconnect_all();
        self.pwm.psel_out_0(self.pin.pin_number());

        // === 4. PWM waveform configuration ===
        self.pwm.mode(); // Up counter
        self.pwm.prescaler(); // DIV_16 → 1 MHz PWM clock
        self.pwm.decoder_common(); // All channels share one duty value

        // === 5. Continuous playback via LOOP=1 + SHORTS auto-restart ===
        // CODAL pattern: play SEQ[0]→SEQ[1] once, then SHORTS automatically
        // triggers SEQSTART[0] again → infinite loop without CPU intervention.
        self.pwm.loop_one();
        self.pwm.shorts_loopsdone_seqstart0();

        // === 6. Set initial frequency (440 Hz) ===
        let countertop = PWM_CLOCK_HZ / DEFAULT_FREQ_HZ;
        self.pwm.countertop(countertop);
        let duty = (countertop / 2) as u16; // 50% duty for max volume
        self.set_duty(duty);

        // === 7. EasyDMA sequence pointers ===
        let duty_ptr: *const u16 = unsafe { (&raw const DUTY_CYCLE.data).cast::<u16>() };

        // SEQ[0] and SEQ[1] both point to the same buffer.
        // With LOOP=1, the PWM plays SEQ[0] then SEQ[1] then SHORTS restarts.
        self.pwm.seq0_ptr(duty_ptr);
        self.pwm.seq0_cnt(1);
        self.pwm.seq0_refresh(0); // Re-read duty every PWM period
        self.pwm.seq0_enddelay();

        self.pwm.seq1_ptr(duty_ptr);
        self.pwm.seq1_cnt(1);
        self.pwm.seq1_refresh(0);
        self.pwm.seq1_enddelay();

        // === 8. Enable PWM (must be AFTER all configuration) ===
        self.pwm.enable();
    }

    pub fn start(&self, _cancellation_token: &CancellationToken) {
        self.pwm.tasks_seqstart0();
    }

    pub fn stop(&self) {
        self.pwm.tasks_stop();
    }

    /// Set the speaker frequency in Hz.
    /// Updates countertop and duty cycle (50%) for the given frequency.
    #[allow(clippy::cast_possible_truncation)] // countertop/2 always fits u16
    pub fn set_frequency(&self, freq_hz: u32) {
        let countertop = PWM_CLOCK_HZ / freq_hz;
        self.pwm.countertop(countertop);
        let duty = (countertop / 2) as u16;
        self.set_duty(duty);
    }

    /// Write duty cycle value to the `EasyDMA` buffer.
    #[allow(clippy::unused_self)]
    pub fn set_duty(&self, duty: u16) {
        unsafe {
            let ptr: *mut u16 = (&raw mut DUTY_CYCLE.data).cast::<u16>();
            write_volatile(ptr, duty);
        }
    }
}

unsafe impl Sync for Speaker {}
unsafe impl Send for Speaker {}
