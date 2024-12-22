// Copyright (c) 2024, zhao.shaowei <nsearchf@yeah.net>
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

// cpu_peripherals/src/lib.rs

pub mod bus;
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

    fn read_byte(&self, address: DeviceAddress) -> Result<u8, CpuPeripheralsError> {
        Err(CpuPeripheralsError::DeviceReadFailed(address as u64))
    }
    fn write_byte(
        &mut self,
        address: DeviceAddress,
        _value: u8,
    ) -> Result<(), CpuPeripheralsError> {
        Err(CpuPeripheralsError::DeviceWriteFailed(address as u64))
    }

    fn read_halfword(&self, address: DeviceAddress) -> Result<u16, CpuPeripheralsError> {
        Err(CpuPeripheralsError::DeviceReadFailed(address as u64))
    }
    fn write_halfword(
        &mut self,
        address: DeviceAddress,
        _value: u16,
    ) -> Result<(), CpuPeripheralsError> {
        Err(CpuPeripheralsError::DeviceWriteFailed(address as u64))
    }

    fn read_word(&self, address: DeviceAddress) -> Result<u32, CpuPeripheralsError> {
        Err(CpuPeripheralsError::DeviceReadFailed(address as u64))
    }
    fn write_word(
        &mut self,
        address: DeviceAddress,
        _value: u32,
    ) -> Result<(), CpuPeripheralsError> {
        Err(CpuPeripheralsError::DeviceWriteFailed(address as u64))
    }

    fn read(&self, address: DeviceAddress, _size: usize) -> Result<Vec<u8>, CpuPeripheralsError> {
        Err(CpuPeripheralsError::DeviceReadFailed(address as u64))
    }
    fn write(&mut self, address: DeviceAddress, _data: &[u8]) -> Result<(), CpuPeripheralsError> {
        Err(CpuPeripheralsError::DeviceWriteFailed(address as u64))
    }
}

pub trait ClintDevice: Device {
    fn tick(&mut self, mip: &mut u32);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{mem::Mem, uart::Uart};

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
