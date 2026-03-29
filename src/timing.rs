#![allow(dead_code)]

use crate::app::cancellation_token::CancellationToken;
use crate::interrupt::wfi;
use crate::peripherals::rtc::RTC_TICKS;
use crate::traits::Cancellable;
use core::sync::atomic::Ordering;

pub fn wait_ticks(ticks: usize, cancellation_token: &CancellationToken) {
    let start = RTC_TICKS.load(Ordering::Relaxed);
    let target = start.wrapping_add(ticks);

    while (RTC_TICKS
        .load(Ordering::Relaxed)
        .wrapping_sub(target)
        .cast_signed())
        < 0
    {
        if cancellation_token.is_cancelled() {
            return;
        }

        wfi();
    }
}

pub fn repeat_for_ticks<F>(ticks: usize, mut action: F, cancellation_token: &CancellationToken)
where
    F: FnMut(),
{
    let start = RTC_TICKS.load(Ordering::Relaxed);
    let target = start.wrapping_add(ticks);

    while (RTC_TICKS
        .load(Ordering::Relaxed)
        .wrapping_sub(target)
        .cast_signed())
        < 0
    {
        if cancellation_token.is_cancelled() {
            return;
        }

        action();
    }
}
