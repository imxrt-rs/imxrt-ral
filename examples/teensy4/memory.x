MEMORY
{
    RAM     (rwx): ORIGIN = 0x20200000, LENGTH = 512K
    FLASH   (rwx): ORIGIN = 0x60001400, LENGTH = 1979K
    BOOT    (r  ): ORIGIN = 0x60000000, LENGTH = 5K
}

EXTERN(FLEXSPI_CONFIGURATION_BLOCK);

SECTIONS
{
    /* If you add more sections to flash, you must add this section here */
    __lflash = SIZEOF(.boot) + SIZEOF(.vector_table) + SIZEOF(.text) + SIZEOF(.rodata) + SIZEOF(.data) + SIZEOF(.gnu.sgstubs);
    .boot ORIGIN(BOOT) :
    {
        /* Firmware Configuration Block (FCB) */
        KEEP(*(.fcb));
        FILL(0xFFFFFFFF);
        . = ORIGIN(BOOT) + 0x1000;
        __ivt = .;
        /* ------------------
         * Image Vector Table
         * ------------------
         */
        LONG(0x402000D1);           /* Header, magic number */
        LONG(ADDR(.vector_table));  /* Address of the vectors table */
        LONG(0x00000000);           /* RESERVED */
        LONG(0x00000000);           /* Device Configuration Data (unused) */
        LONG(__boot_data);          /* Address to boot data */
        LONG(__ivt);                /* Self reference, required by boot ROM */
        LONG(0x00000000);           /* Command Sequence File (unused) */
        LONG(0x00000000);           /* RESERVED */
        /* ---------
         * Boot data
         * ---------
         */
        __boot_data = .;
        LONG(ORIGIN(BOOT));         /* Start of image (origin of flash) */
        LONG(__lflash);             /* Length of flash */
        LONG(0x00000000);           /* Plugin flag (unused) */
        /* --------- */
    } > BOOT   
}
