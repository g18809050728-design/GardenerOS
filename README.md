# GardenerOS

操作系统课程实验仓库，当前为实验 0「配置环境」。

## 环境

- 容器：`gardeneros`，基于 `openeuler/openeuler` 镜像，系统为 openEuler 24.03 (LTS-SP3)，
  配置完成后已提交为镜像 `myopeneuler:latest`。
- 本地目录：`D:\Code\GardenerOS` 通过 bind mount 挂载到容器内 `/mnt`，
  宿主机与容器看到的是同一份文件。
- Rust：默认工具链 nightly (1.102.0)，仓库根目录的 `rust-toolchain` 固定为
  `nightly-2026-05-08` (1.97.0)，已安装 `riscv64gc-unknown-none-elf` 目标、
  `llvm-tools-preview`、`rust-src` 与 `cargo-binutils`。
- QEMU：11.0.0，编译目标 `riscv64-softmmu` 与 `riscv64-linux-user`，安装在 `/usr/local/bin`。

## 使用

```bash
docker start gardeneros
docker exec -it gardeneros bash
```

进入容器后 `cd /mnt` 即为本仓库。

## 目录

- `verify/`：no_std 静态库，用于验证 Rust 到 RISC-V 的交叉编译链路。
