// Wannes

//! Routines for initialization of AM335X.

/// Enables VFP using NEON coprocessor
/// When compiling with ARM hard float, this needs to be called before any VFP instr
#[cfg(target_arch = "arm")]
#[inline(always)]
pub fn enable_vfp() {
    // todo maybe rewrite in Rust
    unsafe {
        asm!(" MRC   p15, #0, r3, c1, c0, #2    @ Read CPACR\n\t
		ORR   r3, r3, #0x00F00000        @ Enable access to CP10 and CP11\n\t
		MCR   p15, #0, r3, c1, c0, #2    @ Write CPACR\n\t
		MOV   r3, #0\n\t
      	MOV   r0,#0x40000000\n\t
        FMXR  FPEXC,r0                   @ Set FPEXC bit 30 to enable VFP\n\t
		ISB   SY    @ flush prefetch buffer because of FMXR above" : : : "r3", "r0" : "volatile")
    };
}