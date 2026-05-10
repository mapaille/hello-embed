use crate::interrupt::wfi;
use crate::peripherals::temp::Temp;

pub struct Thermometer {
    temp: &'static Temp,
    max_attempts: usize,
}

impl Thermometer {
    pub const fn new(temp: &'static Temp, max_attempts: usize) -> Self {
        Self { temp, max_attempts }
    }

    pub fn read_temperature(&self) -> Option<u32> {
        self.temp.start();
        let mut attempts = 0;
        while !self.temp.is_ready() {
            if attempts >= self.max_attempts {
                self.temp.stop();
                self.temp.clear();
                return None;
            }
            attempts += 1;
            wfi();
        }

        self.temp.stop();
        let value = self.temp.read();
        self.temp.clear();

        // Round to nearest whole number: add half the divisor (2) before dividing by 4
        let temperature = (value + 2) / 4;

        Some(temperature)
    }
}
