.globl _vector_table
.globl _load_exception_frame

/*
    Exception Frame Layout
    
    All entries are 8-byte values
    0x000 : x0, regs[0]
    0x008 : x1, regs[1]
    ...
    0x0F0 : x30, regs[30]

    0x0F8 : sp
    0x100 : elr
    0x108 : spsr
    0x110 : esr
    0x118 : far
    0x120 : (kind)
*/

.equ    REG_OFFSET,     0x000
.equ    SP_OFFSET,      0x0F8
.equ    ELR_OFFSET,     0x100
.equ    SPSR_OFFSET,    0x108
.equ    ESR_OFFSET,     0x110
.equ    FAR_OFFSET,     0x118
.equ    KIND_OFFSET,    0x120

.equ    FRAME_SIZE,     296

.macro vec_item kind
    // align each item to 0x80 bytes
    .balign 0x80

    // allocate memory for exception frame
    sub     sp, sp, #FRAME_SIZE

    // store x0 initially, which will be overwritten by kind
    str     x0, [sp, #REG_OFFSET]
    mov     x0, #\kind

    // now branch to remaining store_exception_frame logic
    b       store_exception_frame
.endm

store_exception_frame:
    // from initial vec_item, we expect the stack space to be allocated
    // and x0 to be stored, with x0 now holding the exception kind

    // TEMP: dummy values to test struct layout
    // mov     x0, #0 // x0 no longer works, will just store whatever was in it initially
    // mov     x1, #1
    // mov     x2, #2
    // mov     x3, #3
    // mov     x4, #4
    // mov     x5, #5
    // mov     x6, #6
    // mov     x7, #7
    // mov     x8, #8
    // mov     x9, #9
    // mov     x10, #10
    // mov     x11, #11
    // mov     x12, #12
    // mov     x13, #13
    // mov     x14, #14
    // mov     x15, #15
    // mov     x16, #16
    // mov     x17, #17
    // mov     x18, #18
    // mov     x19, #19
    // mov     x20, #20
    // mov     x21, #21
    // mov     x22, #22
    // mov     x23, #23
    // mov     x24, #24
    // mov     x25, #25
    // mov     x26, #26
    // mov     x27, #27
    // mov     x28, #28
    // mov     x29, #29
    // mov     x30, #30
    // ----

    // store remaining x1-30 registers
    stp     x1,  x2,  [sp, #REG_OFFSET + 0x08]
    stp     x3,  x4,  [sp, #REG_OFFSET + 0x18]
    stp     x5,  x6,  [sp, #REG_OFFSET + 0x28]
    stp     x7,  x8,  [sp, #REG_OFFSET + 0x38]
    stp     x9,  x10, [sp, #REG_OFFSET + 0x48]
    stp     x11, x12, [sp, #REG_OFFSET + 0x58]
    stp     x13, x14, [sp, #REG_OFFSET + 0x68]
    stp     x15, x16, [sp, #REG_OFFSET + 0x78]
    stp     x17, x18, [sp, #REG_OFFSET + 0x88]
    stp     x19, x20, [sp, #REG_OFFSET + 0x98]
    stp     x21, x22, [sp, #REG_OFFSET + 0xA8]
    stp     x23, x24, [sp, #REG_OFFSET + 0xB8]
    stp     x25, x26, [sp, #REG_OFFSET + 0xC8]
    stp     x27, x28, [sp, #REG_OFFSET + 0xD8]
    stp     x29, x30, [sp, #REG_OFFSET + 0xE8]
    
    // store exception kind value (x0)
    str     x0, [sp, #KIND_OFFSET]

    // and store remaining system registers
    mov     x0, sp
    str     x0, [sp, #SP_OFFSET]
    mrs     x0, ELR_EL1
    str     x0, [sp, #ELR_OFFSET]
    mrs     x0, SPSR_EL1
    str     x0, [sp, #SPSR_OFFSET]
    mrs     x0, ESR_EL1
    str     x0, [sp, #ESR_OFFSET]
    mrs     x0, FAR_EL1
    str     x0, [sp, #FAR_OFFSET]

    // before swapping back to rust, we need to provide pointer
    // to exception frame in register x0, per C ABI
    mov     x0, sp
    bl      _handle_exception

_load_exception_frame:
    // we're expecting _handle_exception to return the new sp
    // which will be sitting in x0
    mov     sp, x0

    // now restore registers from exception frame
    ldr     x0, [sp, #ELR_OFFSET]
    msr     ELR_EL1, x0
    ldr     x0, [sp, #SPSR_OFFSET]
    msr     SPSR_EL1, x0
    ldr     x0, [sp, #ESR_OFFSET]
    msr     ESR_EL1, x0
    ldr     x0, [sp, #FAR_OFFSET]
    msr     FAR_EL1, x0
    // (kind doesn't matter here)
    
    ldp     x0,  x1,  [sp, #REG_OFFSET + 0x00]
    ldp     x2,  x3,  [sp, #REG_OFFSET + 0x10]
    ldp     x4,  x5,  [sp, #REG_OFFSET + 0x20]
    ldp     x6,  x7,  [sp, #REG_OFFSET + 0x30]
    ldp     x8,  x9,  [sp, #REG_OFFSET + 0x40]
    ldp     x10, x11, [sp, #REG_OFFSET + 0x50]
    ldp     x12, x13, [sp, #REG_OFFSET + 0x60]
    ldp     x14, x15, [sp, #REG_OFFSET + 0x70]
    ldp     x16, x17, [sp, #REG_OFFSET + 0x80]
    ldp     x18, x19, [sp, #REG_OFFSET + 0x90]
    ldp     x20, x21, [sp, #REG_OFFSET + 0xA0]
    ldp     x22, x23, [sp, #REG_OFFSET + 0xB0]
    ldp     x24, x25, [sp, #REG_OFFSET + 0xC0]
    ldp     x26, x27, [sp, #REG_OFFSET + 0xD0]
    ldp     x28, x29, [sp, #REG_OFFSET + 0xE0]
    ldr     x30,      [sp, #REG_OFFSET + 0xF0]

    // Deallocate frame from stack and return
    add     sp, sp, #FRAME_SIZE
    eret

.balign 2048 // align entire table to 2KB
_vector_table:
    // when passing value to kernel,
    // bits 3:2 describe where the exception originated
    // bits 1:0 denote the exception type

    // Current EL with SP_EL0
    vec_item 0 // synchronous
    vec_item 1 // irq
    vec_item 2 // fiq
    vec_item 3 // serror

    // Current EL with SP_ELx
    vec_item 4 // synchronous
    vec_item 5 // irq
    vec_item 6 // fiq
    vec_item 7 // serror

    // Lower EL using aarch64
    vec_item 8 // synchronous
    vec_item 9 // irq
    vec_item 10 // fiq
    vec_item 11 // serror

    // Lower EL using aarch32
    vec_item 12 // synchronous
    vec_item 13 // irq
    vec_item 14 // fiq
    vec_item 15 // serror
