// constants used for configuring system registers, specific to the ARMv8-A architecture

// -------------------- EL3 System Registers -------------------- //

#[no_mangle]
/// Secure configuration register
static __SCR_EL3: u64 = SCR_EL3_RES1 | SCR_EL3_RW | SCR_EL3_NS;

const SCR_EL3_RES1: u64 = 1 << 5 | 1 << 4;
const SCR_EL3_RW: u64 = 1 << 10; // EL2 is aarch64, EL2 controls EL1 and EL0 behaviors
const SCR_EL3_NS: u64 = 1 << 0; // EL1 and EL0 are in non-secure state

#[no_mangle]
/// Saved program state register, used to move down from EL3 to EL2
static __SPSR_EL3: u64 = 0b_111100_1001;

// -------------------- EL2 System Registers -------------------- //

#[no_mangle]
/// Hypervisor configuration register
static __HCR_EL2: u64 = HCR_EL2_RW;

const HCR_EL2_RW: u64 = 1 << 31; // EL1 is aarch64, PSTATE.nRW controls EL0 behavior

#[no_mangle]
/// Saved program state register, used to move down from EL2 to EL1
static __SPSR_EL2: u64 = 0b_111100_0101;

// -------------------- EL1 System Registers -------------------- //

#[no_mangle]
/// System control register for EL1 using physical addresses
static __SCTLR_EL1_MMU_DISABLED: u64 =
    SCTLR_EL1_RES1 | SCTLR_EL1_EE | SCTLR_EL1_E0E | SCTLR_EL1_I | SCTLR_EL1_C | SCTLR_EL1_M_DISABLE;

#[no_mangle]
/// System control register for EL1 using virtual addresses
static __SCTLR_EL1_MMU_ENABLED: u64 =
    SCTLR_EL1_RES1 | SCTLR_EL1_EE | SCTLR_EL1_E0E | SCTLR_EL1_I | SCTLR_EL1_C | SCTLR_EL1_M_ENABLE;

const SCTLR_EL1_RES1: u64 = 1 << 29 | 1 << 28 | 1 << 23 | 1 << 22 | 1 << 20 | 1 << 11;
const SCTLR_EL1_EE: u64 = 0 << 25; // EL1 data access is little endian
const SCTLR_EL1_E0E: u64 = 0 << 24; // EL0 data access is little endian
const SCTLR_EL1_I: u64 = 0 << 12; // EL1 and EL0 instruction access is non-cacheable
const SCTLR_EL1_C: u64 = 0 << 2; // EL1 and EL0 data access is non-cacheable
const SCTLR_EL1_M_DISABLE: u64 = 0 << 0; // disable MMU
const SCTLR_EL1_M_ENABLE: u64 = 1 << 0; // enable MMU

#[no_mangle]
/// Architectural feature access control register
static __CPACR_EL1: u32 = CPACR_EL1_FPEN;
const CPACR_EL1_FPEN: u32 = 0b11 << 20; // EL1 and EL0 have access to SIMD instructions

#[no_mangle]
/// Memory attribute indirection register
static __MAIR_EL1: u64 = MAIR_EL1_ATTR0;

const MAIR_EL1_ATTR0: u64 = 0b1111_1111; // normal memory, cacheable

#[no_mangle]
/// Translation control register for EL1
static __TCR_EL1: u64 = TCR_EL1_IPS | TCR_EL1_TG0 | TCR_EL1_T0SZ;

const TCR_EL1_IPS: u64 = 0b101 << 32; // 48 bit intermediate physical address size
const TCR_EL1_TG0: u64 = 0b00 << 14; // 4KB user space translation granule
const TCR_EL1_T0SZ: u64 = 0b10000; // initial lookup level is L0, using all 48 bits of the virtual address

#[no_mangle]
/// Default attributes to be used for the initial kernel identity map, except the physical address
static __KERNEL_IDENTITY_MAP_ATTR: u64 = TABLE_DESCR_BLOCK
    | TABLE_DESCR_NG
    | TABLE_DESCR_AF
    | TABLE_DESCR_SH
    | TABLE_DESCR_AP
    | TABLE_DESCR_NS
    | TABLE_DESCR_ATTR_IDX_0;

const TABLE_DESCR_BLOCK: u64 = 0b01;
const TABLE_DESCR_NG: u64 = 0 << 11; // non-global bit, this entry only applies to the current ASID
const TABLE_DESCR_AF: u64 = 1 << 10; // access flag, this page is being accessed for the first time
const TABLE_DESCR_SH: u64 = 0b00 << 8; // sharability field, marked as non-sharable for now
const TABLE_DESCR_AP: u64 = 0b00 << 6; // access permission bits, read-write for EL1
const TABLE_DESCR_NS: u64 = 0 << 5; // non-secure bit
const TABLE_DESCR_ATTR_IDX_0: u64 = 0b000 << 2; // MAIR index 0
