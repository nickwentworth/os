// TODO: send the enumerated type to the kernel to handle different exceptions
.macro vec_item type
    // align each item to 0x80 bytes
    .balign 0x80
    bl      _handle_exception
.endm

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
