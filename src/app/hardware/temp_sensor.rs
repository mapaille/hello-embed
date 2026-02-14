use crate::interrupt::wfi;
use crate::peripherals::temp;

pub struct TempSensor {
    max_attempts: usize,
}

pub trait TemperatureSensor {
    fn read_temperature(&self) -> Option<u32>;
}

impl TempSensor {
    pub const fn new(max_attempts: usize) -> Self {
        Self { max_attempts }
    }
}

impl TemperatureSensor for TempSensor {
    fn read_temperature(&self) -> Option<u32> {
        temp::start();

        let mut attempts = 0;
        while !temp::is_ready() {
            if attempts >= self.max_attempts {
                temp::stop();
                temp::clear();
                return None;
            }
            attempts += 1;
            wfi();
        }

        temp::stop();
        let value = temp::read();
        temp::clear();

        // Round to nearest whole number: add half the divisor (2) before dividing by 4
        let temperature = (value + 2) / 4;

        Some(temperature)
    }
}
