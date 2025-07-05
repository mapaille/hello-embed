pub mod frames;
pub mod animations;

use core::sync::atomic::Ordering;
use crate::pin::Pin;
use crate::rtc::RTC_TICKS;
use crate::interrupt::wfi;

pub struct Screen<const X : usize, const Y : usize> {
    pub row_pins: [Pin; Y],
    pub col_pins: [Pin; X],
}

impl<const X: usize, const Y: usize> Screen<X, Y> {
    pub fn init(row_pins: [Pin; Y], col_pins: [Pin; X]) -> Self {
        let screen = Screen {
            row_pins,
            col_pins,
        };

        for pin in screen.row_pins.iter() {
            pin.as_output().set_low();
        }

        for pin in screen.col_pins.iter() {
            pin.as_output().set_high();
        }

        screen
    }

    #[inline(always)]
    pub fn refresh_once(&mut self, frame: &frames::frame::Frame<X, Y>) {
        for (row_index, row_pin) in self.row_pins.iter().enumerate() {
            for (col_index, col_pin) in self.col_pins.iter().enumerate() {
                if frame.0[row_index][col_index] {
                    col_pin.set_low();
                } else {
                    col_pin.set_high();
                }
            }

            row_pin.set_high();
            wfi();
            row_pin.set_low();
        }
    }

    #[inline(always)]
    pub fn refresh_for(&mut self, frame: &frames::frame::Frame<X, Y>, ticks: u32) {
        let start = RTC_TICKS.load(Ordering::Relaxed);
        let target = start.wrapping_add(ticks);

        while (RTC_TICKS.load(Ordering::Relaxed).wrapping_sub(target) as i32) < 0 {
            self.refresh_once(frame);
        }
    }

    pub fn play_animation_once<const SIZE: usize>(&mut self, animation: &animations::Animation<X, Y, SIZE>, fps: u32) {
        let frame_duration = 1000 / fps;

        for frame in animation.frames.iter() {
            self.refresh_for(frame, frame_duration);
        }
    }

    pub fn play_animation_for<const SIZE: usize>(&mut self, animation: &animations::Animation<X, Y, SIZE>, fps: u32, times: u32) {
        for _ in 0..times {
            self.play_animation_once(animation, fps)
        }
    }

    pub fn clear(&mut self) {
        self.refresh_once(&frames::frame::Frame(&[[false; X]; Y]));
    }
}
