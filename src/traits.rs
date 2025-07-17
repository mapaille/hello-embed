use crate::cancellation::CancellationToken;
use crate::drivers::screens::{animations, frames};

pub trait Cancellable {
    fn cancel(&self);
}

pub trait Resettable {
    fn reset(&self);
}

pub trait Pressable {
    fn is_pressed(&self) -> bool;
}

pub trait Displayable<const X: usize, const Y: usize> {
    fn refresh_once(&mut self, frame: &frames::frame::Frame<X, Y>);
    fn refresh_for(
        &mut self,
        frame: &frames::frame::Frame<X, Y>,
        ticks: u32,
        cancellation_token: &CancellationToken,
    );
    fn play_animation_once<const SIZE: usize>(
        &mut self,
        animation: &animations::Animation<X, Y, SIZE>,
        fps: u32,
        cancellation_token: &CancellationToken,
    );
    fn play_animation_for<const SIZE: usize>(
        &mut self,
        animation: &animations::Animation<X, Y, SIZE>,
        fps: u32,
        times: u32,
        cancellation_token: &CancellationToken,
    );
    fn clear(&mut self);
}
