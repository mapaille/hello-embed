#![allow(dead_code)]
#![allow(unused_imports)]

use crate::cancellation::CancellationToken;
use crate::drivers::screen::Screen;

pub mod love_program;
pub mod temp_program;
pub mod startup_program;

pub trait Program {
    fn run(&mut self, screen: &mut Screen<5, 5>, cancellation_token: &CancellationToken);
}
