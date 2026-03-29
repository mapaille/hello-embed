#![allow(dead_code)]
#![allow(unused_imports)]

use crate::app::App;
use crate::drivers::display::EmbeddedScreen;
use core::ops::Deref;

mod audio_program;
mod empty_program;
pub mod love_program;
pub mod startup_program;
pub mod temperature_program;
mod x_program;

use crate::programs::audio_program::AudioProgram;
use crate::programs::empty_program::EmptyProgram;
use crate::programs::x_program::XProgram;
pub use love_program::LoveProgram;
pub use startup_program::StartupProgram;
pub use temperature_program::TemperatureProgram;

const NUMBER_OF_PROGRAMS: usize = 6;
const EMPTY_PROGRAM: EmptyProgram = EmptyProgram;
const STARTUP_PROGRAM: StartupProgram = StartupProgram;
const LOVE_PROGRAM: LoveProgram = LoveProgram;
const TEMPERATURE_PROGRAM: TemperatureProgram = TemperatureProgram;
const X_PROGRAM: XProgram = XProgram;
const AUDIO_PROGRAM: AudioProgram = AudioProgram;

pub const fn get_programs() -> [&'static dyn Program; NUMBER_OF_PROGRAMS] {
    [
        &EMPTY_PROGRAM,
        &STARTUP_PROGRAM,
        &LOVE_PROGRAM,
        &TEMPERATURE_PROGRAM,
        &X_PROGRAM,
        &AUDIO_PROGRAM,
    ]
}

pub const fn get_number_of_programs() -> usize {
    NUMBER_OF_PROGRAMS
}

pub trait Program {
    fn run(&self, app: &App);
}
