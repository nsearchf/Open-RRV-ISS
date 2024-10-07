// Copyright (c) 2024, zhao.shaowei <nsearchf@yeah.net>
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

// cpu_peripherals/src/lib.rs

pub mod bus;
pub mod clint;
pub mod mem;
pub mod uart;

use thiserror::Error;

/// Define error types for the cpu_peripherals crate.
#[derive(Error, Debug, PartialEq)]
pub enum CpuPeripheralsError {
    /// Error for failed device initialization
    #[error("Failed to initialize device: {0}")]
    DeviceInitializationFailed(String),

    /// Error for invalid device operation
    #[error("Invalid device operation at address {0:#x}")]
    InvalidDeviceOperation(u64),

    /// Error for device read failure
    #[error("Failed to read from device at address {0:#x}")]
    DeviceReadFailed(u64),

    /// Error for device write failure
    #[error("Failed to write to device at address {0:#x}")]
    DeviceWriteFailed(u64),

    #[error("Invalid size: {0}")]
    InvalidSize(usize),

    #[error("Invalid address: {0}")]
    InvalidAddress(DeviceAddress),

    #[error("Invalid device address: {0}")]
    InvalidDeviceAddress(DeviceAddress),
}

// Enum to define the type of Device
#[derive(Debug, PartialEq)]
pub enum DeviceType {
    Clint,
    Mem,
    Uart,
}

pub type DeviceAddress = usize;
pub type DeviceSize = usize;

// Trait to define the interface for a Device
pub trait Device {
    fn get_type(&self) -> DeviceType;
    fn set_base_addr(&mut self, base_addr: DeviceAddress);

    fn read_byte(&self, address: DeviceAddress) -> Result<u8, CpuPeripheralsError>;
    fn write_byte(&mut self, address: DeviceAddress, value: u8) -> Result<(), CpuPeripheralsError>;

    fn read_halfword(&self, address: DeviceAddress) -> Result<u16, CpuPeripheralsError>;
    fn write_halfword(
        &mut self,
        address: DeviceAddress,
        value: u16,
    ) -> Result<(), CpuPeripheralsError>;

    fn read_word(&self, address: DeviceAddress) -> Result<u32, CpuPeripheralsError>;
    fn write_word(&mut self, address: DeviceAddress, value: u32)
        -> Result<(), CpuPeripheralsError>;

    fn read(&self, address: DeviceAddress, size: usize) -> Result<Vec<u8>, CpuPeripheralsError>;
    fn write(&mut self, address: DeviceAddress, data: &[u8]) -> Result<(), CpuPeripheralsError>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{clint::Clint, mem::Mem, uart::Uart};

    #[test]
    fn test_clint_device() {
        let clint = Clint::new();
        assert_eq!(clint.get_type(), DeviceType::Clint);
    }

    #[test]
    fn test_mem_device() {
        let mem = Mem::new(256);
        assert_eq!(mem.get_type(), DeviceType::Mem);
    }

    #[test]
    fn test_uart_device() {
        let uart = Uart::new("UARTX");
        assert_eq!(uart.get_type(), DeviceType::Uart);
    }
}
