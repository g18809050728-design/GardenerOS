#![no_std]

use core::panic::PanicInfo;

/// 供后续实验复用的极简接口，仅用于验证 riscv64 交叉编译工具链。
pub fn add(a: usize, b: usize) -> usize {
    a + b
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
