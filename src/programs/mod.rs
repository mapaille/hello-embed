#![allow(dead_code)]
#![allow(unused_imports)]

use crate::app::App;
use crate::app::cancellation::CancellationToken;
use crate::drivers::screens::EmbeddedScreen;
use core::ops::Deref;

pub mod love_program;
pub mod startup_program;
pub mod temperature_program;

pub use love_program::LoveProgram;
pub use startup_program::StartupProgram;
pub use temperature_program::TemperatureProgram;

const STARTUP_PROGRAM: StartupProgram = StartupProgram::new();
const LOVE_PROGRAM: LoveProgram = LoveProgram::new();
const TEMPERATURE_PROGRAM: TemperatureProgram = TemperatureProgram::new();

pub trait Program {
    fn run(&mut self, app: &mut App, cancellation_token: &CancellationToken);
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq)]
pub enum ProgramId {
    None = 0,
    Startup = 1,
    Love = 2,
    Temperature = 3,
}

impl From<ProgramId> for u8 {
    fn from(id: ProgramId) -> Self {
        id as Self
    }
}

impl From<u8> for ProgramId {
    fn from(id: u8) -> Self {
        match id {
            1 => Self::Startup,
            2 => Self::Love,
            3 => Self::Temperature,
            _ => Self::None,
        }
    }
}
