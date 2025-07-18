use crate::{interrupt, peripherals};

pub struct TempSensor;

impl TempSensor {
    pub fn new() -> Self {
        Self {}
    }

    pub fn read(&self) -> u32 {
        peripherals::temp::start();

        while !peripherals::temp::is_ready() {
            interrupt::wfi();
        }

        peripherals::temp::stop();
        let value = peripherals::temp::read_temp();
        peripherals::temp::clear();
        value
    }
}
