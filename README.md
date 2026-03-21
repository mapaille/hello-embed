# hello-embed

Minimal embedded Rust project for **BBC micro:bit v2**.

Hobby project to learn low-level programming and Rust (`no_std`, bare-metal, interrupts, peripherals) on real hardware.

## What it does

- Switches between different programs/modes using the built-in buttons (A/B)
- Displays custom LED matrix animations and patterns
- Reads the on-chip temperature sensor and shows the value on the display
- Uses RTC (Real-Time Counter) interrupts for periodic tasks (e.g. timing, measurements)

## Hardware

- BBC micro:bit v2
- USB cable
