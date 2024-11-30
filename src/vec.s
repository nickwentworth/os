.macro vec_item type
    // align each item to 0x80 bytes
    .balign 0x80

    // to send the type to the kernel, store x0 first
    // so we can write the type to a register
    stp     x0, x0, [sp, #-0x10]!
    mov     x0, #\type
    b       exception_handler
.endm

exception_handler:
    // save current processor state
    // besides x0, as it has already been saved
    stp     x1, x2, [sp, #-0x10]!
    stp     x3, x4, [sp, #-0x10]!
    stp     x5, x6, [sp, #-0x10]!
    stp     x7, x8, [sp, #-0x10]!
    stp     x9, x10, [sp, #-0x10]!
    stp     x11, x12, [sp, #-0x10]!
    stp     x13, x14, [sp, #-0x10]!
    stp     x15, x16, [sp, #-0x10]!
    stp     x17, x18, [sp, #-0x10]!
    stp     x19, x20, [sp, #-0x10]!
    stp     x21, x22, [sp, #-0x10]!
    stp     x23, x24, [sp, #-0x10]!
    stp     x25, x26, [sp, #-0x10]!
    stp     x27, x28, [sp, #-0x10]!
    stp     x29, x30, [sp, #-0x10]!

    // branch to kernel exception handler
    bl      _handle_exception

    // load stored processor state in reverse order
    ldp     x29, x30, [sp, #-0x10]!
    ldp     x27, x28, [sp, #-0x10]!
    ldp     x25, x26, [sp, #-0x10]!
    ldp     x23, x24, [sp, #-0x10]!
    ldp     x21, x22, [sp, #-0x10]!
    ldp     x19, x20, [sp, #-0x10]!
    ldp     x17, x18, [sp, #-0x10]!
    ldp     x15, x16, [sp, #-0x10]!
    ldp     x13, x14, [sp, #-0x10]!
    ldp     x11, x12, [sp, #-0x10]!
    ldp     x9, x10, [sp, #-0x10]!
    ldp     x7, x8, [sp, #-0x10]!
    ldp     x5, x6, [sp, #-0x10]!
    ldp     x3, x4, [sp, #-0x10]!
    ldp     x1, x2, [sp, #-0x10]!
    ldp     x0, xzr, [sp, #-0x10]! // including x0 here

    // and finally return from the exception handler
    eret

.globl _vector_table
.balign 2048 // align entire table to 2KB
_vector_table:
    // when passing value to kernel,
    // bits 3:2 describe where the exception originated
    // bits 1:0 denote the exception type

    // Current EL with SP_EL0
    vec_item 0b0000 // synchronous
    vec_item 0b0001 // irq
    vec_item 0b0010 // fiq
    vec_item 0b0011 // serror

    // Current EL with SP_ELx
    vec_item 0b0100 // synchronous
    vec_item 0b0101 // irq
    vec_item 0b0110 // fiq
    vec_item 0b0111 // serror

    // Lower EL using aarch64
    vec_item 0b1000 // synchronous
    vec_item 0b1001 // irq
    vec_item 0b1010 // fiq
    vec_item 0b1011 // serror

    // Lower EL using aarch32
    vec_item 0b1100 // synchronous
    vec_item 0b1101 // irq
    vec_item 0b1110 // fiq
    vec_item 0b1111 // serror
