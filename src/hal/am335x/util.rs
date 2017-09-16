/// Puts value at address
#[cfg(target_arch = "arm")]
#[inline(always)]
pub fn put32(address: usize, value : usize) {
    unsafe {
        asm!("str $0, [$1]" : : "r" (value), "r" (address) : : "volatile");
    };
}