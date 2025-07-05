MEMORY
{
    FLASH : ORIGIN = 0x00000000, LENGTH = 512K
    RAM   : ORIGIN = 0x20000000, LENGTH = 128K
}


/*──────────────────────────────────────────────────────────────
  Sections
  ─────────────────────────────────────────────────────────────*/
SECTIONS
{
    /* 0x0000_0000 – vector table (KEEP so the linker never discards it) */
    .vectors ORIGIN(FLASH) :
    {
        KEEP(*(.vectors))
    } > FLASH

    /* Program code & read-only data */
    .text :
    {
        *(.text .text.*);
        *(.rodata .rodata.*);
        KEEP(*(.init));
        KEEP(*(.fini));
        _etext = .;                 /* end of text = load address of .data */
    } > FLASH

    /* Initialised data – copied from FLASH to RAM in startup */
    .data : AT ( _etext )
    {
        _sdata = .;
        *(.data .data.*);
        _edata = .;
    } > RAM

    /* Zero-initialised data */
    .bss (NOLOAD) :
    {
        _sbss = .;
        *(.bss .bss.*);
        *(COMMON);
        _ebss = .;
    } > RAM

    /* Optional “noinit” section that survives soft-reset */
    .noinit (NOLOAD) :
    {
        *(.noinit .noinit.*);
    } > RAM
}

/*──────────────────────────────────────────────────────────────
  Symbols expected by Rust startup code
  ─────────────────────────────────────────────────────────────*/
PROVIDE(_stack_start = ORIGIN(RAM) + LENGTH(RAM));   /* initial MSP      */
PROVIDE(_stack_end   = _stack_start);                /* compat alias     */

PROVIDE(_sheap = _ebss);      /* heap can start right after .bss if needed */
