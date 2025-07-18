#![allow(dead_code)]
#![allow(unused_imports)]

use crate::app::App;
use crate::app::cancellation::CancellationToken;
use crate::drivers::screens::EmbeddedScreen;

pub mod love_program;
pub mod startup_program;
pub mod temp_program;

pub use love_program::LoveProgram;
pub use startup_program::StartupProgram;
pub use temp_program::TempProgram;

pub trait RunnableProgram {
    fn run(&mut self, app: &mut App, cancellation_token: &CancellationToken);
}
