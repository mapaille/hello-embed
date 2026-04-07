pub mod notes;

use core::ptr::{read_volatile, write_volatile};

use crate::app::cancellation_token::CancellationToken;
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

        // === 5. Continuous playback: SEQEND0 auto-restarts SEQ0 (no LOOP) ===
        // Simpler/reliable for sustained tones; SEQ0/1 identical so restarts seamlessly.
        self.pwm.loop_disable();
        self.pwm.shorts_seqend0_seqstart0();

        // === 6. Set initial frequency (440 Hz) ===
        let countertop = PWM_CLOCK_HZ / DEFAULT_FREQ_HZ;
        self.pwm.countertop(countertop);
        let duty = (countertop / 2) as u16; // 50% duty for max volume
        self.set_duty(duty);

        // === 7. EasyDMA sequence pointers ===
        let duty_ptr: *const u16 = unsafe { (&raw const DUTY_CYCLE.data).cast::<u16>() };
        let duty_ptr_val = duty_ptr as u32;

        // SEQ[0] and SEQ[1] identical; SHORTS keeps it running indefinitely.
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

        // Debug assertions to verify hardware state and help debug audio issues
        debug_assert_eq!(self.pwm.read_reg(0x504), 0, "PWM MODE should be UP (0)");
        debug_assert_eq!(
            self.pwm.read_reg(0x50C),
            4,
            "PWM PRESCALER should be DIV_16 (4)"
        );
        debug_assert_eq!(
            self.pwm.read_reg(0x514),
            0,
            "PWM LOOP should be disabled (0)"
        );
        debug_assert_eq!(
            self.pwm.read_reg(0x200),
            1u32 << 4,
            "PWM SHORTS should enable SEQEND0->SEQSTART0"
        );
        let ct = self.pwm.read_reg(0x508);
        debug_assert!(
            ct > 100 && ct < 10000,
            "Initial COUNTERTOP out of range for 440Hz"
        );
        debug_assert_eq!(self.pwm.read_reg(0x500), 1, "PWM ENABLE should be set");
        debug_assert_eq!(
            self.pwm.read_reg(0x560),
            self.pin.pin_number(),
            "PSEL.OUT[0] mismatch"
        );
        debug_assert_eq!(self.pwm.read_reg(0x520), duty_ptr_val, "SEQ0_PTR mismatch");
        debug_assert_eq!(self.pwm.read_reg(0x540), duty_ptr_val, "SEQ1_PTR mismatch");
        debug_assert_eq!(self.pwm.read_reg(0x524), 1, "SEQ0_CNT should be 1");
        debug_assert_eq!(self.pwm.read_reg(0x544), 1, "SEQ1_CNT should be 1");
        debug_assert_eq!(self.pwm.read_reg(0x528), 0, "SEQ0_REFRESH should be 0");
        debug_assert_eq!(self.pwm.read_reg(0x548), 0, "SEQ1_REFRESH should be 0");
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
        debug_assert!(
            (100..=20000).contains(&freq_hz),
            "Frequency out of audible range"
        );
        let countertop = PWM_CLOCK_HZ / freq_hz;
        debug_assert!(
            (20..=20000).contains(&countertop),
            "Countertop out of valid range"
        );
        self.pwm.countertop(countertop);
        let duty = (countertop / 2) as u16;
        self.set_duty(duty);
        debug_assert_eq!(
            self.pwm.read_reg(0x508),
            countertop,
            "COUNTERTOP register write failed"
        );
        self.pwm.tasks_seqstart0(); // Ensure sequence restarts with new params
    }

    /// Write duty cycle value to the `EasyDMA` buffer.
    #[allow(clippy::unused_self)]
    pub fn set_duty(&self, duty: u16) {
        unsafe {
            let ptr: *mut u16 = (&raw mut DUTY_CYCLE.data).cast::<u16>();
            write_volatile(ptr, duty);
            let read_back = read_volatile(ptr);
            debug_assert_eq!(read_back, duty, "DMA duty buffer write failed");
        }
    }
}

unsafe impl Sync for Speaker {}
unsafe impl Send for Speaker {}
