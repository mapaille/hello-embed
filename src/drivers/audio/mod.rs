pub mod notes;

use core::ptr::write_volatile;

use crate::app::cancellation_token::CancellationToken;
use crate::peripherals::gpio::GpioPin;
use crate::peripherals::pwm::Pwm;

static mut DUTY_CYCLE: [u16;1] = [8000];

pub struct Speaker {
    pin: &'static GpioPin,
    pwm: &'static Pwm,
}

impl Speaker {
    pub const fn new(pin: &'static GpioPin, pwm: &'static Pwm) -> Self {
        Self { pin, pwm }
    }

    pub fn init(&self) {
        // === GPIO setup ===
        self.pin.configure_output();
        self.pin.set_low();

        // === PWM peripheral setup ===
        self.pwm.psel_out_0();     // Connect PWM0 to the speaker pin
        self.pwm.enable();         // Enable the PWM peripheral
        self.pwm.mode();           // Usually UpAndDown or Up mode
        self.pwm.prescaler();      // e.g. Div16 or Div32
        self.pwm.countertop(16000); // Important: this sets the PWM frequency

        self.pwm.loop_max();          // LOOP or ONESHOT?
        self.pwm.decoder_common();        // Usually Common or NextStep

        const RAM_START: usize = 0x2000_0000;
        const RAM_END:   usize = 0x2002_0000;

        // === Sequence pointer ===
        let duty_ptr = unsafe { &raw mut DUTY_CYCLE };

        debug_assert_eq!((duty_ptr as usize) % 4, 0, "EasyDMA requires 4-byte alignment");

        debug_assert_eq!(
            (duty_ptr as usize) % 4,
            0,
            "EasyDMA requires the sequence buffer to be 4-byte (word) aligned"
        );

        self.set_duty([8000]);

        self.pwm.seq0_ptr(duty_ptr);

        // Force a visible panic if duty cycle looks wrong
        let current_duty = unsafe { core::ptr::read_volatile(duty_ptr) };

        debug_assert!(current_duty == [8000]);

        // === Critical sequence settings ===
        self.pwm.seq0_cnt();       // Must be >= 1 (number of u16 values)

        self.pwm.seq0_refresh();   // Usually 0 for continuous tone
        self.pwm.seq0_enddelay();  // Usually 0

        // === Final safety checks before starting ===
    }

    pub fn start(&self, _cancellation_token: &CancellationToken) {
        self.pwm.tasks_seqstart0();
    }

    pub fn stop(&self) {
        self.pwm.tasks_stop();
    }

    /// Change duty cycle while running (useful for tones)
    pub fn set_duty(&self, duty: [u16;1]) {
        unsafe { write_volatile(&raw mut DUTY_CYCLE, duty); }
    }
}

unsafe impl Sync for Speaker {}
unsafe impl Send for Speaker {}
