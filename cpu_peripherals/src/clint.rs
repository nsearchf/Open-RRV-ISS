// Copyright (c) 2024, zhao.shaowei <nsearchf@yeah.net>
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

// cpu_peripherals/src/clint.rs

#![allow(dead_code)]
#![allow(unused_variables)]

use tracing::info;

use crate::{CpuPeripheralsError, Device, DeviceAddress, DeviceType};

pub struct Clint {
    // Add necessary fields for Clint
    base_addr: DeviceAddress,
}

impl Clint {
    pub fn new() -> Self {
        info!("Creating a new Clint device");
        Self { base_addr: 0 }
    }
}

impl Device for Clint {
    fn get_type(&self) -> DeviceType {
        DeviceType::Clint
    }
    fn set_base_addr(&mut self, base_addr: DeviceAddress) {
        self.base_addr = base_addr;
    }

    fn read_byte(&self, address: DeviceAddress) -> Result<u8, CpuPeripheralsError> {
        // Implementation of reading a byte from Clint
        Ok(0)
    }

    fn write_byte(&mut self, address: DeviceAddress, value: u8) -> Result<(), CpuPeripheralsError> {
        // Implementation of writing a byte to Clint
        Ok(())
    }

    fn read_halfword(&self, address: DeviceAddress) -> Result<u16, CpuPeripheralsError> {
        // Implementation of reading a halfword from Clint
        Ok(0)
    }

    fn write_halfword(
        &mut self,
        address: DeviceAddress,
        value: u16,
    ) -> Result<(), CpuPeripheralsError> {
        // Implementation of writing a halfword to Clint
        Ok(())
    }

    fn read_word(&self, address: DeviceAddress) -> Result<u32, CpuPeripheralsError> {
        // Implementation of reading a word from Clint
        Ok(0)
    }

    fn write_word(
        &mut self,
        address: DeviceAddress,
        value: u32,
    ) -> Result<(), CpuPeripheralsError> {
        // Implementation of writing a word to Clint
        Ok(())
    }

    fn read(&self, address: DeviceAddress, size: usize) -> Result<Vec<u8>, CpuPeripheralsError> {
        // Implementation of reading an arbitrary length from Clint
        Ok(vec![0; size])
    }

    fn write(&mut self, address: DeviceAddress, data: &[u8]) -> Result<(), CpuPeripheralsError> {
        // Implementation of writing an arbitrary length to Clint
        Ok(())
    }
}
