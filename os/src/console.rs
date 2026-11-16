//! 基于 write 系统调用的最小输出实现。
use core::fmt::{self, Write};

use crate::sys_write;

struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        sys_write(1, s.as_bytes());
        Ok(())
    }
}

/// 把格式化参数写到标准输出。由 `print!` / `println!` 宏调用。
pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}