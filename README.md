# hello-embed

Minimal embedded Rust for **BBC micro:bit v2** (nRF52833, ARM Cortex-M4F).

Hobby project to learn low-level programming (`no_std`, bare-metal, interrupts, peripherals).

## What it does

- Switches between programs using buttons A/B
- LED matrix animations and patterns
- Temperature sensor display
- Speaker/sound output

## Useful Links

- [micro:bit v2 schematic](https://tech.microbit.org/hardware/schematic/) - Official micro:bit v2 hardware schematic
- [microbit-dal](https://github.com/lancaster-university/microbit-dal/) - Lancaster University BBC micro:bit runtime
- [MicroPython microbit-v2](https://github.com/microbit-foundation/micropython-microbit-v2) - Official micro:bit v2 MicroPython implementation

## Documentation

Schematics and hardware specs are available in `docs/`:
- `cpu-specs.pdf` - nRF52833 CPU specifications
- `hardware.pdf` - Hardware overview
- `schematics.pdf` - Circuit schematics
- `speaker-specs.pdf` - Speaker specifications

## Requirements

- `rustup target add thumbv7em-none-eabihf`
- `cargo install probe-run` (or `cargo-embed`)
- Hardware: BBC micro:bit v2

## Commands

```bash
cargo build              # Debug build
cargo build --release    # Optimized build
cargo embed              # Flash debug
cargo embed --release    # Flash release
cargo clippy             # Linting
```

## Architecture

```
src/
├── main.rs              # Entry point, panic handler
├── system.rs            # Hardware init
├── app/                 # Application state, program runner
├── drivers/             # Hardware drivers (display, audio, buttons, temp)
│   ├── display/         # LED matrix display
│   └── audio/           # Speaker/sound
├── peripherals/         # Low-level HW access (GPIO, PWM, RTC, temp)
├── programs/            # User programs (switchable via buttons A/B)
│   ├── empty_program.rs
│   ├── startup_program.rs
│   ├── love_program.rs
│   ├── temperature_program.rs
│   ├── x_program.rs
│   └── audio_program.rs
└── interrupt.rs         # Interrupt handling
```

## Adding a Program

1. Create `src/programs/my_program.rs` implementing `Program` trait
2. Add module and instance in `src/programs/mod.rs`
3. Add to `get_programs()` array

## Key Patterns

- Use `CancellationToken` to gracefully stop programs
- Use `wfi()` to sleep until interrupt
- Hardware singleton accessed via `app.hardware`
- Buttons A/B cycle through programs

## Constraints

- No dynamic allocation (no `Vec`, `Box`, etc.)
- No standard library
- Interrupts must be short/fast
- Stack size is limited
