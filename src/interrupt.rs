#![allow(dead_code)]

#[allow(clippy::inline_always)]
#[inline(always)]
pub fn wfi() {
    unsafe {
        core::arch::asm!("wfi", options(nomem, nostack, preserves_flags));
    }
}

pub fn enable_global_interrupts() {
    unsafe {
        core::arch::asm!("cpsie i", options(nomem, nostack, preserves_flags));
    }
}
