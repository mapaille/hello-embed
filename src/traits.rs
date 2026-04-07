use core::ptr::{read_volatile, write_volatile, NonNull};
use crate::app::cancellation_token::CancellationToken;
use crate::drivers::display::{animations, frames};

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

pub trait Register {
    fn base_addr(&self) -> NonNull<u8>;

    fn write_reg(&self, byte_offset: usize, value: u32) {
        unsafe {
            let reg_ptr = self.base_addr().as_ptr().add(byte_offset).cast::<u32>();
            write_volatile(reg_ptr, value);
        }
    }

    fn read_reg(&self, byte_offset: usize) -> u32 {
        unsafe {
            let reg_ptr = self.base_addr().as_ptr().add(byte_offset).cast::<u32>();
            read_volatile(reg_ptr)
        }
    }
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
