# Agentic Development Guide for hello-embed

This guide provides specific instructions for AI agents (like opencode) working on the BBC micro:bit v2 embedded Rust project. Following these guidelines will ensure effective, safe, and maintainable contributions.

## Core Principles

### 1. Always Start with Context
- Read `README.md` for project overview, setup, and usage instructions
- Understand the hardware constraints (BBC micro:bit v2, nRF52833, ARM Cortex-M4F)
- Review the architecture diagram in README before making changes

### 2. Embrace Simplicity (KISS)
- Favor simple, straightforward solutions over clever abstractions
- Embedded systems benefit from predictability and clarity
- If a solution requires complex reasoning to understand, simplify it

### 3. Build and Validate Frequently
- Run `cargo build` after every code change to catch compilation errors immediately
- Use `cargo clippy` to catch potential issues and improve code quality
- For logic changes, consider how they would be tested in an embedded context

### 4. Review Changes Thoroughly
- Check for bugs, side effects, and unwanted behavioral changes
- Verify interrupt handlers remain short and fast
- Ensure no accidental introduction of dynamic allocation
- Confirm hardware register accesses are correct and safe

### 5. Maintain Documentation
- Update `README.md` if you change behavior, add features, or modify the API
- Keep hardware-specific constraints documented
- Document any non-obvious design decisions

## Rust-Specific Embedded Practices

### Memory Management
- ❌ NEVER use `Vec`, `Box`, `String`, or other allocating types in hot paths
- ✅ Use static arrays or stack allocation when possible
- ✅ Use `core::sync::atomic` for shared state between interrupt contexts
- ✅ Consider heapless crate alternatives if allocation is absolutely necessary

### Interrupt Handling
- ❌ NEVER perform expensive computations in interrupt handlers
- ❌ NEVER block or wait in interrupt handlers
- ✅ Keep interrupt handlers as short as possible (ideally just setting flags)
- ✅ Offload processing to main loop when possible
- ✅ Use `wfi()` (wait for interrupt) in main loop for power efficiency

### Hardware Access
- ✅ Use volatile read/write operations for hardware registers (`read_volatile`/`write_volatile`)
- ✅ Wrap register access in safe abstractions when possible
- ✅ Validate hardware register values before use when they come from external sources
- ✅ Use appropriate memory ordering for atomic operations (usually `Relaxed` for embedded)

### Concurrency
- ✅ Use `core::sync::atomic` with appropriate ordering for shared state
- ✅ Use `CancellationToken` pattern for cooperative cancellation
- ✅ Consider interrupt safety when accessing shared data
- ❌ NEVER use `Mutex` or `RwLock` from std (not available in no_std without nightly features)

### Error Handling
- ✅ Use `panic_handler` appropriately for unrecoverable errors
- ✅ Consider recoverable error handling strategies where appropriate
- ✅ In embedded contexts, sometimes resetting is the appropriate error handling strategy

## Project-Specific Patterns

### Code Organization
- Follow the existing module structure:
  - `src/main.rs` - Entry point and panic handler
  - `src/system.rs` - Hardware initialization
  - `src/app/` - Application state and program runner
  - `src/drivers/` - Hardware drivers (display, audio, buttons, temperature)
  - `src/peripherals/` - Low-level hardware access (GPIO, PWM, RTC, temp)
  - `src/programs/` - User programs switchable via buttons
  - `src/interrupt.rs` - Interrupt handling
  - `src/traits.rs` - Shared trait definitions

### Hardware Abstraction
- Implement traits for hardware abstractions (`Displayable`, `Cancellable`, etc.)
- Use static references for singleton hardware access (`App::hardware`)
- Keep hardware-specific implementations in drivers/ and peripherals/
- Programs should depend only on abstractions, not concrete hardware types

### Program Development
- To add a new program:
  1. Create `src/programs/my_program.rs` implementing the `Program` trait
  2. Add module and instance in `src/programs/mod.rs`
  3. Add to `get_programs()` array in `src/programs/mod.rs`
- Programs must implement `run(&self, app: &App)` method
- Programs should use `cancellation_token` to check for cancellation
- Programs should be interrupt-safe and not block for long periods

### Display Usage
- Use `EmbeddedScreen` for LED matrix operations
- Leverage existing animation and frame utilities in `drivers/display/`
- Use `play_animation_once` or `play_animation_for` for animated content
- Use `refresh_for` for static content that needs to be displayed for a duration
- Always respect the `CancellationToken` to allow responsive UI

### Button Handling
- Buttons A/B are used to cycle through programs (handled in app logic)
- For custom button handling in programs, use drivers/button/
- Remember to debounce buttons in software if needed
- Button drivers provide `is_pressed()` method following `Pressable` trait

### Audio Output
- Use drivers/audio/ for speaker/sound output
- Leverage `notes.rs` for musical notes if creating melodies
- Audio output should be non-blocking or use callbacks where possible
- Consider power consumption when using audio extensively

### Temperature Sensing
- Use peripherals/temp/ for temperature sensor access
- Temperature readings may require calibration for accuracy
- Consider updating frequency - continuous reading wastes power

### Real-Time Clock (RTC)
- Use peripherals/rtc/ for timekeeping
- Use `app::rtc_handler` for second-level interrupts
- RTC can be used for scheduling or time-based events

## Safety Guidelines

### Unsafe Code
- Mark all `unsafe` blocks with clear safety comments explaining why they're safe
- Limit unsafe code to hardware register access when necessary
- Prefer safe abstractions over direct register manipulation when possible
- Review unsafe code extra carefully during change review

### Power Efficiency
- Utilize `wfi()` (wait for interrupt) in idle loops to reduce power consumption
- Minimize time spent with LEDs fully on (consider duty cycle for brightness)
- Power down unused peripherals when possible
- Consider using lower clock speeds if performance allows

### Determinism
- Avoid unbounded loops or recursion that could cause stack overflow
- Be aware of worst-case execution time for interrupt handlers
- Consider using static analysis tools to verify stack usage
- Test edge cases, especially around buffer boundaries

## Development Workflow

### Making Changes
1. Start by reading README.md to understand context
2. Identify the minimal change needed to address the requirement
3. Implement following existing code patterns and conventions
4. Run `cargo build` frequently during development
5. Review changes for correctness, safety, and adherence to guidelines
6. Update documentation if behavior or API changed
7. Final verification with `cargo build` and `cargo clippy`

### Debugging Approach
- In embedded contexts, debugging options are limited
- Use LED patterns or audio cues for debugging information when possible
- Consider using semihosting or RTT if available and appropriate
- Remember that adding debug code can change timing characteristics
- Remove or conditionally compile debug code for production builds

### Performance Considerations
- Profile critical paths if performance becomes an issue
- Remember that debug builds are significantly slower than release
- Focus on algorithmic optimizations before micro-optimizations
- Consider lookup tables instead of computation when appropriate for embedded

## Specific to This Project

### Constraints to Always Remember
- **No standard library**: Only `core` and allocator-provided crates available
- **No dynamic allocation** in interrupt handlers or time-critical code
- **Interrupts must be short/fast**: Aim for microseconds, not milliseconds
- **Stack size is limited**: Avoid large stack allocations or deep recursion
- **Power matters**: Battery-powered device, efficiency extends battery life

### Hardware-Specific Notes
- BBC micro:bit v2 has specific GPIO pinout - check schematics.pdf
- nRF52833 has specific peripheral access patterns
- Display is 5x5 LED matrix with specific timing requirements
- Built-in sensors (temperature, accelerometer) have specific access methods
- Speaker output has specific voltage and current limitations

### Testing Strategy
- Unit test logic that doesn't depend on hardware when possible
- Use integration tests with actual hardware for hardware-dependent code
- Consider simulation or emulation for early-stage testing
- Always verify changes on actual hardware before considering complete

## When in Doubt

1. Look at existing code for similar patterns
2. Check if there's a trait that provides the abstraction you need
3. Verify your solution doesn't violate embedded Rust constraints
4. Ensure your solution is simple and maintainable
5. Confirm you've run `cargo build` and `cargo clippy`
6. Review your changes for potential issues

By following these guidelines, you'll contribute effectively to this embedded Rust project while maintaining its safety, simplicity, and reliability.