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
    adr     x5, el2_entry
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
    adr     x5, el1_entry
    msr     ELR_EL2, x5
    eret

el1_entry:
    // configure EL1 system registers, initially MMU disabled
    ldr     x5, __SCTLR_EL1_MMU_DISABLED
    msr     SCTLR_EL1, x5
    ldr     x5, __CPACR_EL1
    msr     CPACR_EL1, x5
    ldr     x5, __CNTP_CTL_EL0
    msr     CNTP_CTL_EL0, x5
    
    msr     DAIFCLR, #0b1111 // enable all interrupts

    // reference exception vector table
    ldr     x5, =_vector_table
    msr     VBAR_EL1, x5

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

2:  // setup virtual address related system registers
    ldr     x5, __MAIR_EL1
    msr     MAIR_EL1, x5
    ldr     x5, __TCR_EL1
    msr     TCR_EL1, x5

    // load L0 table base address into system register, then link to L1 table base address
    adr     x5, __L0_TABLE
    msr     TTBR0_EL1, x5
    msr     TTBR1_EL1, x5
    adr     x6, __L1_TABLE
    orr     x7, x6, #0b11 // mark L0 entry as table descriptor
    str     x7, [x5]

    // identity map full physical memory space (0GB - 4GB)
    ldr     x5, __KERNEL_IDENTITY_MAP_ATTR
    mov     x7, #0x00000000
    orr     x8, x5, x7 // 0GB - 1GB
    str     x8, [x6, #0]

    mov     x7, #0x40000000
    orr     x8, x5, x7 // 1GB - 2GB
    str     x8, [x6, #8]

    mov     x7, #0x80000000
    orr     x8, x5, x7 // 2GB - 3GB
    str     x8, [x6, #16]

    mov     x7, #0xC0000000
    orr     x8, x5, x7 // 3GB - 4GB
    str     x8, [x6, #24]

    // enable the MMU
    ldr     x5, __SCTLR_EL1_MMU_ENABLED
    msr     SCTLR_EL1, x5
    isb

    // branch to an absolute virtual address
    ldr     x5, =virtual_addr_jump
    br      x5

virtual_addr_jump:
    // pc is now going off of virtual addresses, safe to reset TTBR0 identity map
    msr     TTBR0_EL1, xzr
    isb

    // and jump to rust code
    b       _kernel_main

// loop forever, send extra cpus here
// TODO: figure out what to do with them
halt:
    wfe
    b halt
