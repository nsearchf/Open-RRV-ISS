// Copyright (c) 2024, zhao.shaowei <nsearchf@yeah.net>
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

// cpu_peripherals/src/uart.rs

use tracing::info;

use crate::{CpuPeripheralsError, Device, DeviceAddress, DeviceType};

// SiFive FE310-G002
// 0x1001_3000 0x1001_3FFF   RWA    UART 0
// 0x1002_3000 0x1002_3FFF   RWA    UART 1
// 0x00 txdata  Transmit data register
// 0x04 rxdata  Receive data register
pub const UART_TXDATA: DeviceAddress = 0x00;
pub const UART_RXDATA: DeviceAddress = 0x04;

pub struct Uart {
    // Add necessary fields for Uart
    name: &'static str,
    base_addr: DeviceAddress,
    tx_buffer: Vec<u8>,
}

impl Uart {
    pub fn new(name: &'static str) -> Self {
        info!("Creating a new UART device");
        let mut uart = Self {
            name,
            base_addr: 0,
            tx_buffer: vec![],
        };

        uart.add_head_to_tx_buffer();

        uart
    }

    fn is_txdata_addr(&self, address: DeviceAddress) -> bool {
        address == (UART_TXDATA + self.base_addr)
    }

    fn write_tx_buffer(&mut self, value: u8) {
        self.tx_buffer.push(value);
        if value == b'\n' {
            self.reset_tx_buffer();
        }
    }

    fn add_head_to_tx_buffer(&mut self) {
        self.tx_buffer.push(b'[' as u8);
        self.tx_buffer.extend_from_slice(self.name.as_bytes());
        self.tx_buffer.push(b']' as u8);
        self.tx_buffer.push(b' ' as u8);
    }

    fn reset_tx_buffer(&mut self) {
        // self.tx_buffer.push(b'\0');
        print!("{}", String::from_utf8_lossy(&self.tx_buffer));
        self.tx_buffer.clear();
        self.add_head_to_tx_buffer();
    }
}

impl Device for Uart {
    fn get_type(&self) -> DeviceType {
        DeviceType::Uart
    }

    fn set_base_addr(&mut self, base_addr: DeviceAddress) {
        self.base_addr = base_addr;
    }
    fn read_byte(&self, address: DeviceAddress) -> Result<u8, CpuPeripheralsError> {
        // Implementation of reading a byte from Uart
        Err(CpuPeripheralsError::DeviceReadFailed(address as u64))
    }

    fn write_byte(&mut self, address: DeviceAddress, value: u8) -> Result<(), CpuPeripheralsError> {
        // Implementation of writing a byte to Uart
        if self.is_txdata_addr(address) {
            self.write_tx_buffer(value);
            Ok(())
        } else {
            Err(CpuPeripheralsError::DeviceWriteFailed(address as u64))
        }
    }

    fn read_halfword(&self, address: DeviceAddress) -> Result<u16, CpuPeripheralsError> {
        // Implementation of reading a halfword from Uart
        Err(CpuPeripheralsError::DeviceReadFailed(address as u64))
    }

    fn write_halfword(
        &mut self,
        address: DeviceAddress,
        _value: u16,
    ) -> Result<(), CpuPeripheralsError> {
        // Implementation of writing a halfword to Uart
        Err(CpuPeripheralsError::DeviceWriteFailed(address as u64))
    }

    fn read_word(&self, address: DeviceAddress) -> Result<u32, CpuPeripheralsError> {
        // Implementation of reading a word from Uart
        Err(CpuPeripheralsError::DeviceReadFailed(address as u64))
    }

    fn write_word(
        &mut self,
        address: DeviceAddress,
        value: u32,
    ) -> Result<(), CpuPeripheralsError> {
        // Implementation of writing a word to Uart
        if self.is_txdata_addr(address) {
            self.write_tx_buffer((value & 0xFF) as u8);
            Ok(())
        } else {
            Err(CpuPeripheralsError::DeviceWriteFailed(address as u64))
        }
    }

    fn read(&self, address: DeviceAddress, _size: usize) -> Result<Vec<u8>, CpuPeripheralsError> {
        // Implementation of reading an arbitrary length from Uart
        // Ok(vec![0; size])
        Err(CpuPeripheralsError::DeviceReadFailed(address as u64))
    }

    fn write(&mut self, address: DeviceAddress, data: &[u8]) -> Result<(), CpuPeripheralsError> {
        // Implementation of writing an arbitrary length to Uart
        if self.is_txdata_addr(address) {
            for byte in data {
                self.write_tx_buffer(*byte);
            }
            Ok(())
        } else {
            Err(CpuPeripheralsError::DeviceWriteFailed(address as u64))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uart_new() {
        let uart = Uart::new("test");
        assert_eq!(uart.name, "test");
        assert_eq!(uart.base_addr, 0);
        assert_eq!(uart.tx_buffer.len(), "test".len() + 3);
    }

    #[test]
    fn uart_write_byte_to_txdata_address_should_succeed() {
        let mut uart = Uart::new("test_uart");
        uart.set_base_addr(0x1000);

        let address: DeviceAddress = 0x1000 + UART_TXDATA;
        let data = b'A';

        assert!(uart.write_byte(address, data).is_ok());
        assert_eq!(
            uart.tx_buffer,
            vec![b'[', b't', b'e', b's', b't', b'_', b'u', b'a', b'r', b't', b']', b' ', b'A']
        );
    }

    #[test]
    fn uart_write_byte_to_non_txdata_address_should_fail() {
        let mut uart = Uart::new("test_uart");
        uart.set_base_addr(0x1000);

        let address: DeviceAddress = 0x1000 + 0xFF;
        let data = b'A';

        assert!(uart.write_byte(address, data).is_err());
    }

    #[test]
    fn uart_write_word_to_txdata_address_should_succeed() {
        let mut uart = Uart::new("test_uart");
        uart.set_base_addr(0x1000);

        let address: DeviceAddress = 0x1000 + UART_TXDATA;
        let data = 0xABCD;

        assert!(uart.write_word(address, data).is_ok());
        assert_eq!(
            uart.tx_buffer,
            vec![b'[', b't', b'e', b's', b't', b'_', b'u', b'a', b'r', b't', b']', b' ', 0xcd]
        );
    }

    #[test]
    fn uart_write_arbitrary_length_to_txdata_address_should_succeed() {
        let mut uart = Uart::new("test_uart");
        uart.set_base_addr(0x1000);

        let address: DeviceAddress = 0x1000 + UART_TXDATA;
        let data = b"Hello World\n";

        assert!(uart.write(address, data).is_ok());
        assert_eq!(
            uart.tx_buffer,
            vec![b'[', b't', b'e', b's', b't', b'_', b'u', b'a', b'r', b't', b']', b' ']
        );
    }
}
