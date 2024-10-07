// Copyright (c) 2024, zhao.shaowei <nsearchf@yeah.net>
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

// rv_core/src/csr.rs

use std::collections::HashMap;

use tracing::info;

use crate::inst_csr_reg::*;
use crate::GprUnsigned;

pub type CsrAddrType = u16;

pub const MSTATUS_MIE: GprUnsigned = 0x00000008;
pub const MSTATUS_MPIE: GprUnsigned = 0x00000080;
pub const MSTATUS_MPP: GprUnsigned = 0x00001800;
pub const MSTATUS_MPRV: GprUnsigned = 0x00020000;

// Error type for CSR operations
#[derive(Debug, thiserror::Error, PartialEq)]
pub enum CsrError {
    #[error("Invalid CSR address")]
    InvalidAddress,
}

/// Represents a single CSR register
struct CsrRegister {
    value: u32,
    writable_bits: u32, // Bit mask indicating writable bits
    initial_value: u32,
}

impl CsrRegister {
    /// Creates a new CsrRegister with the specified writable bits and initial value
    fn new(writable_bits: u32, initial_value: u32) -> Self {
        info!("Creating new CSR");
        Self {
            value: initial_value,
            writable_bits,
            initial_value,
        }
    }

    /// Reads the value of the CSR register
    fn read(&self) -> u32 {
        self.value
    }

    /// Writes a value to the CSR register, only modifying writable bits
    fn write(&mut self, value: u32) {
        let masked_value = value & self.writable_bits;
        self.value = (self.value & !self.writable_bits) | masked_value;
    }

    fn reset(&mut self) {
        self.value = self.initial_value;
    }
}

/// Represents a RISC-V Control and Status Registers (CSR)
pub(crate) struct Csr {
    registers: HashMap<CsrAddrType, CsrRegister>,
}

impl Csr {
    /// Creates a new CSR instance with all registers initialized to their initial values
    pub fn new() -> Self {
        let mut registers = HashMap::new();

        // Initialize the registers with their writable bits and initial values
        registers.insert(CSR_MSTATUS, CsrRegister::new(0xFFFFFFFF, 0x00001800));
        registers.insert(CSR_MISA, CsrRegister::new(0xFFFFFFFF, 0x40001100));
        registers.insert(CSR_MIE, CsrRegister::new(0xFFFFFFFF, 0x00000000));
        registers.insert(CSR_MTVEC, CsrRegister::new(0xFFFFFFFF, 0x00000000));
        registers.insert(CSR_MSCRATCH, CsrRegister::new(0xFFFFFFFF, 0x00000000));
        registers.insert(CSR_MEPC, CsrRegister::new(0xFFFFFFFF, 0x00000000));
        registers.insert(CSR_MCAUSE, CsrRegister::new(0xFFFFFFFF, 0x00000000));
        registers.insert(CSR_MTVAL, CsrRegister::new(0xFFFFFFFF, 0x00000000));
        registers.insert(CSR_MIP, CsrRegister::new(0xFFFFFFFF, 0x00000000));

        Csr { registers }
    }

    /// Reads the value of a CSR register
    pub fn read(&self, address: CsrAddrType) -> Result<u32, CsrError> {
        if let Some(register) = self.registers.get(&address) {
            Ok(register.read())
        } else {
            Err(CsrError::InvalidAddress)
        }
    }

    /// Writes a value to a CSR register
    pub fn write(&mut self, address: CsrAddrType, value: u32) -> Result<(), CsrError> {
        if let Some(register) = self.registers.get_mut(&address) {
            register.write(value);
            Ok(())
        } else {
            Err(CsrError::InvalidAddress)
        }
    }

    pub fn reset(&mut self) {
        for register in self.registers.values_mut() {
            register.reset();
        }
    }
}

impl Csr {
    /// Reads the value of a CSR and writes a new value
    pub fn csrrw(&mut self, address: CsrAddrType, rs1_val: u32) -> Result<u32, CsrError> {
        let old_val = self.read(address)?;
        self.write(address, rs1_val)?;
        Ok(old_val)
    }

    /// Reads the value of a CSR and sets the bits specified by the mask
    pub fn csrrs(&mut self, address: CsrAddrType, rs1_val: u32) -> Result<u32, CsrError> {
        let old_val = self.read(address)?;
        let new_val = old_val | rs1_val;
        self.write(address, new_val)?;
        Ok(old_val)
    }

    /// Reads the value of a CSR and clears the bits specified by the mask
    pub fn csrrc(&mut self, address: CsrAddrType, rs1_val: u32) -> Result<u32, CsrError> {
        let old_val = self.read(address)?;
        let new_val = old_val & !rs1_val;
        self.write(address, new_val)?;
        Ok(old_val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_csr_read_write() {
        let mut csr = Csr::new();
        assert_eq!(csr.read(CSR_MSTATUS), Ok(0x00001800));
        csr.write(CSR_MSTATUS, 42).unwrap();
        assert_eq!(csr.read(CSR_MSTATUS), Ok(42)); // Only writable bits are modified
        assert!(csr.write(4096, 42).is_err());
        assert!(csr.read(4096).is_err());
    }

    #[test]
    fn test_csr_instructions() {
        let mut csr = Csr::new();
        csr.write(CSR_MSTATUS, 42).unwrap();
        assert_eq!(csr.csrrw(CSR_MSTATUS, 10).unwrap(), 42);
        assert_eq!(csr.read(CSR_MSTATUS).unwrap(), 10);
        assert_eq!(csr.csrrs(CSR_MSTATUS, 5).unwrap(), 10);
        assert_eq!(csr.read(CSR_MSTATUS).unwrap(), 10 | 5);
        assert_eq!(csr.csrrc(CSR_MSTATUS, 3).unwrap(), (10 | 5));
        assert_eq!(csr.read(CSR_MSTATUS).unwrap(), (10 | 5) & (!3));
    }
}
