use crate::app::cancellation::CancellationToken;
use crate::drivers::screens::{animations, frames};

pub trait Cancellable {
    fn cancel(&self);
    fn is_cancelled(&self) -> bool;
}

pub trait Resettable {
    fn reset(&self);
}

pub trait Pressable {
    fn is_pressed(&self) -> bool;
}

pub trait Clearable {
    fn clear(&self);
}

pub trait Displayable<const X: usize, const Y: usize> {
    fn refresh_once(&self, frame: &frames::frame::Frame<X, Y>);
    fn refresh_for(
        &self,
        frame: &frames::frame::Frame<X, Y>,
        ticks: usize,
        cancellation_token: &CancellationToken,
    );
    fn play_animation_once<const SIZE: usize>(
        &self,
        animation: &animations::Animation<X, Y, SIZE>,
        fps: usize,
        cancellation_token: &CancellationToken,
    );
    fn play_animation_for<const SIZE: usize>(
        &self,
        animation: &animations::Animation<X, Y, SIZE>,
        fps: usize,
        times: usize,
        cancellation_token: &CancellationToken,
    );
}
