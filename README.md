# Open-RRV-ISS

A functionally accurate RISC-V instruction set simulator developed in Rust.

## 项目介绍

- 目标: 用 Rust 编写一个 RISC-V 的功能精确 ISS，这个 ISS 要能够运行 FreeRTOS。
- 采用不严格的 TDD 来开发，开发过程中单元测试和集成测试要同步进行。
- 片上外设参考 SiFive FE310-G002。

## 参考的开源指令集模拟器

- [Spike, a RISC-V ISA Simulator](https://github.com/riscv-software-src/riscv-isa-sim)
- [riscv-rust](https://github.com/takahirox/riscv-rust) : RISC-V processor emulator written in Rust+WASM
