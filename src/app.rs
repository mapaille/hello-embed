use crate::{
    drivers::screen::{animations, frames, Screen},
    peripherals::temp,
    timing::wait_ticks,
    interrupt
};

pub fn run(screen: &mut Screen<5, 5>) -> ! {
    screen.play_animation_for(&animations::ANIMATION_LOADING, 30, 2);
    
    loop {
        temp::start();
        
        screen.play_animation_once(&animations::ANIMATION_LOVE, 2);
        screen.play_animation_for(&animations::ANIMATION_HEARTBEAT, 5, 3);

        while !temp::is_ready() {
            interrupt::wfi();
        }

        temp::stop();
        
        // Round to nearest whole number: add half the divisor (2) before dividing by 4
        let temperature = (temp::read_temp() + 2) / 4;

        display_temperature(screen, temperature);

        temp::clear();
        screen.clear();
        wait_ticks(500);
    }
}

fn display_temperature(screen: &mut Screen<5, 5>, temperature: u32) {
    if temperature < 100 {
        let first_digit = frames::get_digit(temperature / 10).unwrap_or(&frames::DIGIT_0);
        let second_digit = frames::get_digit(temperature % 10).unwrap_or(&frames::DIGIT_0);

        screen.refresh_for(first_digit, 500);
        screen.refresh_for(second_digit, 500);
    }
}
