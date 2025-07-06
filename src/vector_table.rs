//! Interrupt vector table for the microcontroller
//! 
//! This module contains the complete vector table with 16 core ARM Cortex-M 
//! exceptions plus 48 peripheral IRQs specific to the nRF52833.

use crate::peripherals::rtc;

unsafe extern "C" {
    static _stack_start: u32;
}

#[repr(C)]
pub union Vector {
    pub start: &'static u32,
    pub reset_handler: unsafe extern "C" fn() -> !,
    pub handler: unsafe extern "C" fn(),   // address of an ISR
    pub reserved: usize,                   // 0 = "not used"
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
    Vector { reset_handler: super::reset_handler                             },   //  1  Reset
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
