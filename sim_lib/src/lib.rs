// Copyright (c) 2024, zhao.shaowei <nsearchf@yeah.net>
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

pub mod simulator;

pub mod loader;

use goblin::error::Error as GoblinError;
use thiserror::Error;

use cpu_peripherals::CpuPeripheralsError;
use rv_core::RvCoreError;

pub use rv_core::ProgramCounter;

/// Define error types for the simulator crate.
#[derive(Error, Debug)]
pub enum SimulatorError {
    /// Error for invalid configuration
    #[error("Invalid configuration: {0}")]
    InvalidConfiguration(String),

    /// Error for failed initialization
    #[error("Failed to initialize simulator: {0}")]
    InitializationFailed(String),

    /// Error for execution failure
    #[error("Execution failed: {0}")]
    ExecutionFailed(String),

    /// Error for unhandled trap
    #[error("Unhandled trap: {0}")]
    UnhandledTrap(String),

    #[error("Device not found")]
    DeviceNotFound,

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    RvCore(#[from] CpuPeripheralsError),

    #[error(transparent)]
    Bus(#[from] RvCoreError),

    #[error(transparent)]
    LoaderError(#[from] GoblinError),
}
