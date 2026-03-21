use crate::programs::Program;

pub struct EmptyProgram;

impl Program for EmptyProgram {
    fn run(&self, _app: &crate::app::App) {}
}
