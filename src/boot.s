.section ".text.boot"

.globl _start

_start:
    // halt all but main processor
    mrs     x5, MPIDR_EL1
    and     x5, x5, #0b11
    cbnz    x5, halt

    // initialize stack
    ldr     x5, =_start
    mov     sp, x5

    // clear bss
    ldr     x5, =__bss_start
    ldr     w6, =__bss_size
1:  cbz     w6, 2f
    str     xzr, [x5], #8
    sub     w6, w6, #1
    cbnz    w6, 1b

// jump to rust code, which shouldn't return
2:  bl      _kernel_main

// loop forever, send extra cpus here
// TODO: figure out what to do with them
halt:
    wfe
    b halt
