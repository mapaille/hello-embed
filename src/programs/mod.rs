#![allow(dead_code)]
#![allow(unused_imports)]

use crate::app::App;
use crate::cancellation::CancellationToken;
use crate::drivers::screens::EmbeddedScreen;

pub mod love_program;
pub mod startup_program;
pub mod temp_program;

pub trait Program {
    fn run(&mut self, app: &mut App, cancellation_token: &CancellationToken);
}
