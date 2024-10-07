// Copyright (c) 2024, zhao.shaowei <nsearchf@yeah.net>
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

// cpu_peripherals/src/mem.rs

use tracing::{info, trace};

use crate::{CpuPeripheralsError, Device, DeviceAddress, DeviceType};

pub struct Mem {
    // Add necessary fields for Mem
    base_addr: DeviceAddress,
    data: Vec<u8>,
}

impl Mem {
    pub fn new(size: usize) -> Self {
        info!("Creating a new Memory device, size is {}", size);
        Self {
            // Initialize necessary fields for Mem
            base_addr: 0,
            data: vec![0; size],
        }
    }
}

impl Device for Mem {
    fn get_type(&self) -> DeviceType {
        DeviceType::Mem
    }

    fn set_base_addr(&mut self, base_addr: DeviceAddress) {
        self.base_addr = base_addr;
    }

    fn read_byte(&self, address: DeviceAddress) -> Result<u8, CpuPeripheralsError> {
        // Implementation of reading a byte from Mem
        let addr = address - self.base_addr;
        Ok(self.data[addr as usize])
    }

    fn write_byte(&mut self, address: DeviceAddress, value: u8) -> Result<(), CpuPeripheralsError> {
        // Implementation of writing a byte to Mem
        let addr = address - self.base_addr;
        self.data[addr as usize] = value;
        Ok(())
    }

    fn read_halfword(&self, address: DeviceAddress) -> Result<u16, CpuPeripheralsError> {
        // Implementation of reading a halfword from Mem
        let addr = address - self.base_addr;
        let halfword = u16::from_le_bytes(self.data[addr..(addr + 2)].try_into().unwrap());
        trace!(
            "Reading halfword from address {:#010x}: {:#06x}",
            address,
            halfword
        );
        Ok(halfword)
    }

    fn write_halfword(
        &mut self,
        address: DeviceAddress,
        value: u16,
    ) -> Result<(), CpuPeripheralsError> {
        // Implementation of writing a halfword to Mem
        let addr = address - self.base_addr;
        self.data[addr..(addr + 2)].copy_from_slice(&value.to_le_bytes());
        Ok(())
    }

    fn read_word(&self, address: DeviceAddress) -> Result<u32, CpuPeripheralsError> {
        let addr = address - self.base_addr;
        let word = u32::from_le_bytes(self.data[addr..(addr + 4)].try_into().unwrap());
        trace!(
            "Reading word from address {:#010x}: {:#010x}",
            address,
            word
        );
        Ok(word)
    }

    fn write_word(
        &mut self,
        address: DeviceAddress,
        value: u32,
    ) -> Result<(), CpuPeripheralsError> {
        // Implementation of writing a word to Mem
        let addr = address - self.base_addr;
        self.data[addr..(addr + 4)].copy_from_slice(&value.to_le_bytes());
        Ok(())
    }

    fn read(&self, address: DeviceAddress, size: usize) -> Result<Vec<u8>, CpuPeripheralsError> {
        // Implementation of reading an arbitrary length from Mem
        let addr = address - self.base_addr;
        if size > self.data.len() {
            return Err(CpuPeripheralsError::InvalidSize(size));
        }
        if addr + size > self.data.len() {
            return Err(CpuPeripheralsError::InvalidAddress(address));
        }
        Ok(self.data[addr..(addr + size)].to_vec())
    }

    fn write(&mut self, address: DeviceAddress, data: &[u8]) -> Result<(), CpuPeripheralsError> {
        // Implementation of writing an arbitrary length to Mem
        let addr = address - self.base_addr;
        let size = data.len();
        if size > self.data.len() {
            return Err(CpuPeripheralsError::InvalidSize(size));
        }
        if addr + size > self.data.len() {
            return Err(CpuPeripheralsError::InvalidAddress(address));
        }
        self.data[addr..(addr + size)].copy_from_slice(data);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_MEM_SIZE: usize = 256;

    #[test]
    fn mem_new() {
        let size = TEST_MEM_SIZE;
        let mem = Mem::new(size);
        assert_eq!(mem.data.len(), size);
    }

    #[test]
    fn mem_read_byte() {
        let mem = Mem::new(TEST_MEM_SIZE);
        let address = 0;
        let result = mem.read_byte(address);
        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    fn mem_write_byte() {
        let mut mem = Mem::new(TEST_MEM_SIZE);
        let address = 4;
        let value = 5u8;
        mem.write_byte(address, value).unwrap();
        assert_eq!(mem.data[4], value);
    }

    #[test]
    fn mem_read_halfword() {
        let mem = Mem::new(TEST_MEM_SIZE);
        let address = 8;
        let result = mem.read_halfword(address);
        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    fn mem_write_halfword() {
        let mut mem = Mem::new(TEST_MEM_SIZE);
        let address = 12;
        let value = 0x1234u16;
        mem.write_halfword(address, value).unwrap();
        assert_eq!(mem.data[12], value.to_le_bytes()[0]);
        assert_eq!(mem.data[13], value.to_le_bytes()[1]);
    }

    #[test]
    fn mem_read_word() {
        let mem = Mem::new(TEST_MEM_SIZE);
        let address = 16;
        let result = mem.read_word(address);
        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    fn mem_write_word() {
        let mut mem = Mem::new(TEST_MEM_SIZE);
        let address = 20;
        let value = 0xabcd1234u32;
        mem.write_word(address, value).unwrap();
        assert_eq!(mem.data[20], value.to_le_bytes()[0]);
        assert_eq!(mem.data[21], value.to_le_bytes()[1]);
        assert_eq!(mem.data[22], value.to_le_bytes()[2]);
        assert_eq!(mem.data[23], value.to_le_bytes()[3]);
    }
}
