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
/// System control register for EL1
static __SCTLR_EL1: u64 =
    SCTLR_EL1_RES1 | SCTLR_EL1_EE | SCTLR_EL1_E0E | SCTLR_EL1_I | SCTLR_EL1_C | SCTLR_EL1_M;

const SCTLR_EL1_RES1: u64 = 1 << 29 | 1 << 28 | 1 << 23 | 1 << 22 | 1 << 20 | 1 << 11;
const SCTLR_EL1_EE: u64 = 0 << 25; // EL1 data access is little endian
const SCTLR_EL1_E0E: u64 = 0 << 24; // EL0 data access is little endian
const SCTLR_EL1_I: u64 = 0 << 12; // EL1 and EL0 instruction access is non-cacheable
const SCTLR_EL1_C: u64 = 0 << 2; // EL1 and EL0 data access is non-cacheable
const SCTLR_EL1_M: u64 = 0 << 0; // disable MMU

/// Architectural feature access control register
#[no_mangle]
static __CPACR_EL1: u32 = CPACR_EL1_FPEN;
const CPACR_EL1_FPEN: u32 = 0b11 << 20; // EL1 and EL0 have access to SIMD instructions
