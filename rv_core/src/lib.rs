// Copyright (c) 2024, zhao.shaowei <nsearchf@yeah.net>
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

// rv_core/src/lib.rs

pub mod core;
mod csr;
pub mod decode;
mod execute;
pub mod fetch;
pub mod trap;

pub mod inst_cause;
pub mod inst_csr_reg;

pub type ProgramCounter = u32;
pub type MachineInstruction = u32;
pub type GprUnsigned = u32;
pub type GprSigned = i32;

pub type RegisterIndex = usize;

use thiserror::Error;

/// Define error types for the rv_core crate.
#[derive(Error, Debug, PartialEq)]
pub enum RvCoreError {
    /// Error for invalid instruction
    #[error("Invalid instruction: {0:?}")]
    InvalidInstruction(MachineInstruction),

    #[error("Invalid register: {0:#x}")]
    InvalidRegisterIndex(RegisterIndex),

    #[error("Invalid register name: {0}")]
    InvalidRegisterName(String),

    #[error("Invalid program counter: {0:#x}")]
    InvalidProgramCounter(ProgramCounter),

    #[error("Invalid memory address: {0:#x}")]
    InvalidMemoryAddress(ProgramCounter),

    #[error("Invalid memory size: {0:#x}")]
    InvalidMemorySize(ProgramCounter),

    /// Error for invalid register access
    #[error("Invalid register access: {0}")]
    InvalidRegisterAccess(String),

    /// Error for illegal memory access
    #[error("Illegal memory access at address {0:#x}")]
    IllegalMemoryAccess(u64),

    /// Error for unimplemented instruction
    #[error("Unimplemented instruction: {0:#x}")]
    UnimplementedInstruction(u32),

    #[error("Shamt is invalid: {0:#x}")]
    ShamtIsInvalid(u32),

    #[error("Invalid Trap mode: {0}")]
    InvalidTrapMode(u32),

    #[error("CsrError: {0}")]
    CsrError(#[from] csr::CsrError),

    #[error("CpuPeripheralsError: {0}")]
    CpuPeripheralsError(#[from] cpu_peripherals::CpuPeripheralsError),
}
