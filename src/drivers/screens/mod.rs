pub mod animations;
pub mod frames;

use crate::app::cancellation::CancellationToken;
use crate::interrupt::wfi;
use crate::peripherals::gpio::GpioPin;
use crate::timing::repeat_for_ticks;
use crate::traits::Displayable;

pub struct EmbeddedScreen<const X: usize, const Y: usize> {
    width: usize,
    height: usize,
    row_pins: [GpioPin; Y],
    col_pins: [GpioPin; X],
}

impl<const X: usize, const Y: usize> EmbeddedScreen<X, Y> {
    pub fn new(row_pins: [GpioPin; Y], col_pins: [GpioPin; X]) -> Self {
        let screen = EmbeddedScreen {
            width: X,
            height: Y,
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
}

impl Displayable<5, 5> for EmbeddedScreen<5, 5> {
    #[inline(always)]
    fn refresh_once(&mut self, frame: &frames::frame::Frame<5, 5>) {
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
    fn refresh_for(
        &mut self,
        frame: &frames::frame::Frame<5, 5>,
        ticks: u32,
        cancellation_token: &CancellationToken,
    ) {
        repeat_for_ticks(
            ticks,
            || {
                self.refresh_once(frame);
            },
            cancellation_token,
        );
    }

    fn play_animation_once<const SIZE: usize>(
        &mut self,
        animation: &animations::Animation<5, 5, SIZE>,
        fps: u32,
        cancellation_token: &CancellationToken,
    ) {
        let frame_duration = 1000 / fps;

        for frame in animation.frames.iter() {
            if cancellation_token.is_cancelled() {
                return;
            }
            self.refresh_for(frame, frame_duration, cancellation_token);
        }
    }

    fn play_animation_for<const SIZE: usize>(
        &mut self,
        animation: &animations::Animation<5, 5, SIZE>,
        fps: u32,
        times: u32,
        cancellation_token: &CancellationToken,
    ) {
        for _ in 0..times {
            if cancellation_token.is_cancelled() {
                return;
            }
            self.play_animation_once(animation, fps, cancellation_token)
        }
    }

    fn clear(&mut self) {
        self.refresh_once(&frames::frame::Frame(&[[false; 5]; 5]));
    }
}
