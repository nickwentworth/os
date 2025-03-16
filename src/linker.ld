__kernel_phys_start_addr = 0x80000;
__kernel_virt_start_addr = 0xFFFF000000000000 + __kernel_phys_start_addr;

ENTRY(_start)

SECTIONS {
    . = __kernel_virt_start_addr;
    __start = .;

    __text_start = .;
    .text : AT(__kernel_phys_start_addr) {
        KEEP(*(.text.boot))
        *(.text)
    }
    . = ALIGN(4096);
    __text_end = .;

    __rodata_start = .;
    .rodata : {
        *(.rodata)
    }
    . = ALIGN(4096);
    __rodata_end = .;

    __data_start = .;
    .data : {
        *(.data)
    }
    . = ALIGN(4096);
    __data_end = .;

    __bss_start = .;
    .bss : {
        bss = .;
        *(.bss)
    }
    . = ALIGN(4096);
    __bss_end = .;
    __bss_size = __bss_end - __bss_start;
    
    __end = .;
}
