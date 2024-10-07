// Copyright (c) 2024, zhao.shaowei <nsearchf@yeah.net>
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

// cpu_peripheral/src/bus.rs

use std::collections::HashMap;

use tracing::info;

use crate::{CpuPeripheralsError, Device, DeviceAddress, DeviceSize};

// use std::sync::Arc;
// pub type DevicePointer<T> = Arc<T>;
// pub type DeviceHandler = DevicePointer<dyn Device + Send + Sync>;

pub type DevicePointer<T> = Box<T>;
pub type DeviceHandler = DevicePointer<dyn Device>;

pub struct Bus {
    devices: HashMap<(DeviceAddress, DeviceSize), DeviceHandler>,
}

impl Bus {
    pub fn new() -> Self {
        info!("Creating new bus");
        Self {
            devices: HashMap::new(),
        }
    }

    pub fn add_device(
        &mut self,
        base_addr: DeviceAddress,
        size: DeviceSize,
        mut device: DeviceHandler,
    ) -> Result<(), CpuPeripheralsError> {
        device.set_base_addr(base_addr);
        self.devices.insert((base_addr, base_addr + size), device);
        Ok(())
    }

    pub fn find_device(
        &self,
        address: DeviceAddress,
    ) -> Result<&DeviceHandler, CpuPeripheralsError> {
        for (&(start, end), device) in &self.devices {
            if address >= start && address < end {
                return Ok(device);
            }
        }
        Err(CpuPeripheralsError::InvalidDeviceAddress(address))
    }

    pub fn find_device_mut(
        &mut self,
        address: DeviceAddress,
    ) -> Result<&mut DeviceHandler, CpuPeripheralsError> {
        for (&(start, end), device) in &mut self.devices {
            if address >= start && address < end {
                return Ok(device);
            }
        }
        Err(CpuPeripheralsError::InvalidDeviceAddress(address))
    }

    pub fn read_byte(&self, address: DeviceAddress) -> Result<u8, CpuPeripheralsError> {
        let device = self.find_device(address)?;
        let val = device.read_byte(address)?;
        Ok(val)
    }

    pub fn write_byte(
        &mut self,
        address: DeviceAddress,
        value: u8,
    ) -> Result<(), CpuPeripheralsError> {
        let device = self.find_device_mut(address)?;
        device.write_byte(address, value)?;
        Ok(())
    }

    pub fn read_halfword(&self, address: DeviceAddress) -> Result<u16, CpuPeripheralsError> {
        let device = self.find_device(address)?;
        let val = device.read_halfword(address)?;
        Ok(val)
    }
    pub fn write_halfword(
        &mut self,
        address: DeviceAddress,
        value: u16,
    ) -> Result<(), CpuPeripheralsError> {
        let device = self.find_device_mut(address)?;
        device.write_halfword(address, value)?;
        Ok(())
    }

    pub fn read_word(&self, address: DeviceAddress) -> Result<u32, CpuPeripheralsError> {
        let device = self.find_device(address)?;
        let val = device.read_word(address)?;
        Ok(val)
    }
    pub fn write_word(
        &mut self,
        address: DeviceAddress,
        value: u32,
    ) -> Result<(), CpuPeripheralsError> {
        let device = self.find_device_mut(address)?;
        device.write_word(address, value)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{clint::Clint, mem::Mem, uart::Uart, DeviceType};

    #[test]
    fn test_bus_add_and_find_device() {
        let mut bus = Bus::new();
        let clint = DevicePointer::new(Clint::new());
        let mem = DevicePointer::new(Mem::new(256));
        let uart = DevicePointer::new(Uart::new("test_uart"));

        let _ = bus.add_device(0x0000_0000, 0x1000, clint);
        let _ = bus.add_device(0x1000_0000, 0x1000, mem);
        let _ = bus.add_device(0x2000_0000, 0x1000, uart);

        assert_eq!(
            bus.find_device(0x0000_0000).unwrap().get_type(),
            DeviceType::Clint
        );
        assert_eq!(
            bus.find_device(0x1000_0000).unwrap().get_type(),
            DeviceType::Mem
        );
        assert_eq!(
            bus.find_device(0x2000_0000).unwrap().get_type(),
            DeviceType::Uart
        );
    }

    #[test]
    fn test_bus_read_and_write_byte() {
        let mut bus = Bus::new();
        let mem = DevicePointer::new(Mem::new(256));
        let _ = bus.add_device(0x1000_0000, 256, mem);

        assert!(bus.write_byte(0x1000_0000, 0xFF).is_ok());
        assert_eq!(bus.read_byte(0x1000_0000), Ok(0xFF));

        assert!(bus.write_halfword(0x1000_0004, 0x1234).is_ok());
        assert_eq!(bus.read_halfword(0x1000_0004), Ok(0x1234));

        assert!(bus.write_word(0x1000_0008, 0x1234abcd).is_ok());
        assert_eq!(bus.read_word(0x1000_0008), Ok(0x1234abcd));
    }
}
