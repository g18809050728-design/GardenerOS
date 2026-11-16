#!/usr/bin/env bash
# 验证 Rust 到 riscv64 的交叉编译链路：编译 -> 生成目标文件 -> 查看格式
set -e
cd "$(dirname "$0")"
export PATH="$HOME/.cargo/bin:$PATH"

cargo build --release --target riscv64gc-unknown-none-elf
rustc --target riscv64gc-unknown-none-elf --crate-type lib --emit=obj src/lib.rs -o /tmp/verify.o
rust-objdump -f /tmp/verify.o
