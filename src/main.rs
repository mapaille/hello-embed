#![no_std]
#![no_main]

mod peripherals;
mod drivers;
mod power;
mod clock;
mod pin;
mod interrupt;

use core::panic::PanicInfo;
use peripherals::{gpio, rtc, temp};
use drivers::screen::{animations, frames, Screen};
use pin::Pin;

const SCREEN_ROW_PINS: [Pin; 5] = [gpio::p0::ROW1, gpio::p0::ROW2, gpio::p0::ROW3, gpio::p0::ROW4, gpio::p0::ROW5];
const SCREEN_COL_PINS: [Pin; 5] = [gpio::p0::COL1, gpio::p0::COL2, gpio::p0::COL3, gpio::p1::COL4, gpio::p0::COL5];

unsafe extern "C" {
    static _stack_start: u32;
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reset_handler() -> ! {
    clock::use_high_frequency_clock();
    interrupt::enable_global_interrupts();
    rtc::init();

    let mut screen = Screen::init(SCREEN_ROW_PINS, SCREEN_COL_PINS);
    
    screen.play_animation_for(&animations::ANIMATION_LOADING, 30, 2);
    power::enable_low_power();
    clock::use_low_frequency_clock();

    loop {
        temp::start();
        screen.play_animation_once(&animations::ANIMATION_LOVE, 2);
        screen.play_animation_for(&animations::ANIMATION_HEARTBEAT, 5, 3);

        while !temp::is_ready() {
            interrupt::wfi();
        }

        temp::stop();

        let t = temp::read_temp() / 4;

        if t < 100 {
            let first_digit = frames::get_digit(t / 10).unwrap_or(&frames::DIGIT_0);
            let second_digit = frames::get_digit(t % 10).unwrap_or(&frames::DIGIT_0);

            screen.refresh_for(first_digit, 500);
            screen.refresh_for(second_digit, 500);
        }

        temp::clear();
        screen.clear();
        rtc::wait_ticks(500);
    }
}

#[repr(C)]
pub union Vector {
    pub start: &'static u32,
    pub reset_handler: unsafe extern "C" fn() -> !,
    pub handler: unsafe extern "C" fn(),   // address of an ISR
    pub reserved: usize,                   // 0 = “not used”
}

//───────────────────────────────────────────────────────────────────────
//  Full, contiguous vector table:  16 core exceptions + 48 IRQs
//───────────────────────────────────────────────────────────────────────
#[unsafe(link_section = ".vectors")]   // ← make sure the linker script places
#[used]                        //    this section at 0x0000_0000
#[unsafe(no_mangle)]
pub static VECTORS: [Vector; 16 + 48] = [
    // ───── System exception vectors ──────────────────────────────────
    Vector { start: unsafe { &_stack_start }} , //  0  Initial MSP
    Vector { reset_handler                             },   //  1  Reset
    Vector { reserved: 0 },                                   //  2  NMI
    Vector { reserved: 0 },                                   //  3  HardFault
    Vector { reserved: 0 },                                   //  4  MemManage
    Vector { reserved: 0 },                                   //  5  BusFault
    Vector { reserved: 0 },                                   //  6  UsageFault
    Vector { reserved: 0 },                                   //  7
    Vector { reserved: 0 },                                   //  8
    Vector { reserved: 0 },                                   //  9
    Vector { reserved: 0 },                                   // 10
    Vector { reserved: 0 },                                   // 11  SVCall
    Vector { reserved: 0 },                                   // 12  DebugMon
    Vector { reserved: 0 },                                   // 13
    Vector { reserved: 0 },                                   // 14  PendSV
    Vector { reserved: 0          },   // 15  SysTick

    // ───── Peripheral IRQ vectors (n = offset-16) ────────────────────
    Vector { reserved: 0 },                                   //  0  POWER_CLOCK
    Vector { reserved: 0 },                                   //  1  RADIO
    Vector { reserved: 0 },                                   //  2  UARTE0 / UART0
    Vector { reserved: 0 },                                   //  3  SPIM0 / SPIS0 / TWI0
    Vector { reserved: 0 },                                   //  4  SPIM1 / SPIS1 / TWI1
    Vector { reserved: 0 },                                   //  5  NFCT
    Vector { reserved: 0 },                                   //  6  GPIOTE
    Vector { reserved: 0 },                                   //  7  SAADC
    Vector { reserved: 0                             },   //  8  TIMER0
    Vector { reserved: 0 },                                   //  9  TIMER1
    Vector { reserved: 0 },                                   // 10  TIMER2
    Vector { handler: rtc::rtc0_handler             },   // 11  RTC0
    Vector { reserved: 0 },                                   // 12  TEMP
    Vector { reserved: 0 },                                   // 13  RNG
    Vector { reserved: 0 },                                   // 14  ECB
    Vector { reserved: 0 },                                   // 15  CCM_AAR
    Vector { reserved: 0 },                                   // 16  WDT
    Vector { reserved: 0 },                                   // 17  RTC1
    Vector { reserved: 0 },                                   // 18  QDEC
    Vector { reserved: 0 },                                   // 19  COMP / LPCOMP
    Vector { reserved: 0 },                                   // 20  SWI0 / EGU0
    Vector { reserved: 0 },                                   // 21  SWI1 / EGU1
    Vector { reserved: 0 },                                   // 22  SWI2 / EGU2
    Vector { reserved: 0 },                                   // 23  SWI3 / EGU3
    Vector { reserved: 0 },                                   // 24  SWI4 / EGU4
    Vector { reserved: 0 },                                   // 25  SWI5 / EGU5
    Vector { reserved: 0 },                                   // 26  TIMER3
    Vector { reserved: 0 },                                   // 27  TIMER4
    Vector { reserved: 0 },                                   // 28  PWM0
    Vector { reserved: 0 },                                   // 29  PDM0
    Vector { reserved: 0 },                                   // 30  MWU
    Vector { reserved: 0 },                                   // 31  PWM1
    Vector { reserved: 0 },                                   // 32  PWM2
    Vector { reserved: 0 },                                   // 33  SPIM2 / SPIS2 / SPI2
    Vector { reserved: 0 },                                   // 34  RTC2
    Vector { reserved: 0 },                                   // 35  I2S
    Vector { reserved: 0 },                                   // 36  FPU
    Vector { reserved: 0 },                                   // 37  USBD
    Vector { reserved: 0 },                                   // 38  UARTE1
    Vector { reserved: 0 },                                   // 39  QSPI
    Vector { reserved: 0 },                                   // 40  CRYPTOCELL
    Vector { reserved: 0 },                                   // 41  SPIM3
    Vector { reserved: 0 },                                   // 42  PWM3
    Vector { reserved: 0 },                                   // 43  NVMC
    Vector { reserved: 0 },                                   // 44  P0 (GPIO port 0)
    Vector { reserved: 0 },                                   // 45  P1 (GPIO port 1)
    Vector { reserved: 0 },                                   // 46  IPC
    Vector { reserved: 0 },                                   // 47  SPIM4
];
