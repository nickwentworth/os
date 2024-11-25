.section ".text.boot"

.globl _start

_start:
    // halt all but main processor
    mrs     x5, MPIDR_EL1
    and     x5, x5, #0b11
    cbnz    x5, halt

el3_entry:
    // verify we are currently in EL3
    mrs     x5, CurrentEL
    lsr     x5, x5, #2
    cmp     x5, #3
    blt     el2_entry

    // configure EL3 system registers
    ldr     x5, __SCR_EL3
    msr     SCR_EL3, x5

    // move to EL2
    ldr     x5, __SPSR_EL3
    msr     SPSR_EL3, x5
    ldr     x5, =el2_entry
    msr     ELR_EL3, x5
    eret

el2_entry:
    // verify we are currently in EL2
    mrs     x5, CurrentEL
    lsr     x5, x5, #2
    cmp     x5, #2
    blt     el1_entry

    // configure EL2 system registers
    ldr     x5, __HCR_EL2
    msr     HCR_EL2, x5

    // move to EL1
    ldr     x5, __SPSR_EL2
    msr     SPSR_EL2, x5
    ldr     x5, =el1_entry
    msr     ELR_EL2, x5
    eret

el1_entry:
    // configure EL1 system registers
    ldr     x5, __SCTLR_EL1
    msr     SCTLR_EL1, x5
    ldr     x5, __CPACR_EL1
    msr     CPACR_EL1, x5

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
