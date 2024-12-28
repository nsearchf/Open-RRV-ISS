// Copyright (c) 2024, zhao.shaowei <nsearchf@yeah.net>
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

// cpu_peripherals/src/clint.rs

//
// Core-Local Interruptor (CLINT)
//

use std::{cell::RefCell, rc::Rc};
use tracing::info;

use crate::{
    csr::Csr,
    inst_csr_reg::{CSR_MIP, MIP_MSIP, MIP_MTIP},
};
use cpu_peripherals::{ClintDevice, CpuPeripheralsError, Device, DeviceAddress, DeviceType};

const CLINT_MSIP_OFFSET: DeviceAddress = 0x0000;
const CLINT_MTIMECMP_L_OFFSET: DeviceAddress = 0x4000;
const CLINT_MTIMECMP_H_OFFSET: DeviceAddress = CLINT_MTIMECMP_L_OFFSET + 4;
const CLINT_MTIME_L_OFFSET: DeviceAddress = 0xbff8;
const CLINT_MTIME_H_OFFSET: DeviceAddress = CLINT_MTIME_L_OFFSET + 4;

pub struct Clint {
    // Add necessary fields for Clint
    base_addr: DeviceAddress,

    msip: u32,
    mtimecmp: u64,
    mtime: u64,

    csr: Rc<RefCell<Csr>>,
}

impl Clint {
    pub fn new(csr: Rc<RefCell<Csr>>) -> Self {
        info!("Creating a new Clint device");
        Self {
            base_addr: 0,
            msip: 0,
            mtimecmp: 0,
            mtime: 0,
            csr,
        }
    }

    // MTIP is read-only in mip, and is cleared by writing to the memory-mapped machine-mode
    // timer compare register.
    fn clear_mtip_bit(&mut self) {
        let mip = {
            let csr = self.csr.borrow();
            let mip = csr.read(CSR_MIP).unwrap();
            mip
        };
        let mut csr = self.csr.borrow_mut();
        csr.write(CSR_MIP, mip & !MIP_MTIP)
            .expect("Write to MIP register failed!");
    }
}

impl Device for Clint {
    fn get_type(&self) -> DeviceType {
        DeviceType::Clint
    }
    fn set_base_addr(&mut self, base_addr: DeviceAddress) {
        self.base_addr = base_addr;
    }

    fn read_word(&self, address: DeviceAddress) -> Result<u32, CpuPeripheralsError> {
        let offset = address - self.base_addr;
        match offset {
            CLINT_MSIP_OFFSET => Ok(self.msip),
            CLINT_MTIMECMP_L_OFFSET => Ok(self.mtimecmp as u32),
            CLINT_MTIMECMP_H_OFFSET => Ok((self.mtimecmp >> 32) as u32),
            CLINT_MTIME_L_OFFSET => Ok(self.mtime as u32),
            CLINT_MTIME_H_OFFSET => Ok((self.mtime >> 32) as u32),

            _ => Err(CpuPeripheralsError::DeviceReadFailed(address as u64)),
        }
    }

    fn write_word(
        &mut self,
        address: DeviceAddress,
        value: u32,
    ) -> Result<(), CpuPeripheralsError> {
        let offset = address - self.base_addr;
        match offset {
            CLINT_MSIP_OFFSET => {
                self.msip = value;
                Ok(())
            }
            CLINT_MTIMECMP_L_OFFSET => {
                self.mtimecmp = (self.mtimecmp & 0xFFFF_FFFF_0000_0000) | (value as u64);
                self.clear_mtip_bit();
                Ok(())
            }
            CLINT_MTIMECMP_H_OFFSET => {
                self.mtimecmp = (self.mtimecmp & 0x0000_0000_FFFF_FFFF) | ((value as u64) << 32);
                Ok(())
            }
            CLINT_MTIME_L_OFFSET => {
                self.mtime = (self.mtime & 0xFFFF_FFFF_0000_0000) | (value as u64);
                Ok(())
            }
            CLINT_MTIME_H_OFFSET => {
                self.mtime = (self.mtime & 0x0000_0000_FFFF_FFFF) | ((value as u64) << 32);
                Ok(())
            }
            _ => Err(CpuPeripheralsError::DeviceWriteFailed(address as u64)),
        }
    }
}

impl ClintDevice for Clint {
    fn tick(&mut self, mip: &mut u32) {
        self.mtime = self.mtime.wrapping_add(1);

        if (self.msip & 1) != 0 {
            *mip |= MIP_MSIP;
        }

        if self.mtimecmp > 0 && self.mtime >= self.mtimecmp {
            *mip |= MIP_MTIP;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // use std::sync::{Arc, Mutex};
    //
    // fn create_csr() -> Arc<Mutex<Csr>> {
    //     Arc::new(Mutex::new(Csr::new()))
    // }

    fn create_csr() -> Rc<RefCell<Csr>> {
        Rc::new(RefCell::new(Csr::new()))
    }

    #[test]
    fn test_clint_new() {
        let clint = Clint::new(create_csr());
        assert_eq!(clint.base_addr, 0);
        assert_eq!(clint.msip, 0);
        assert_eq!(clint.mtimecmp, 0);
        assert_eq!(clint.mtime, 0);
    }

    fn do_test_read_write(clint: &mut Clint, base_addr: DeviceAddress, offset: DeviceAddress) {
        assert_eq!(clint.read_word(base_addr + offset), Ok(0));
        assert_eq!(clint.write_word(base_addr + offset, 0x1234_5678), Ok(()));
        assert_eq!(clint.read_word(base_addr + offset), Ok(0x1234_5678));
    }
    #[test]
    fn test_clint_read_write() {
        let mut clint = Clint::new(create_csr());
        let base_addr: DeviceAddress = 0x200_0000;
        clint.set_base_addr(base_addr);

        do_test_read_write(&mut clint, base_addr, CLINT_MSIP_OFFSET);
        do_test_read_write(&mut clint, base_addr, CLINT_MTIMECMP_L_OFFSET);
        do_test_read_write(&mut clint, base_addr, CLINT_MTIMECMP_H_OFFSET);
        do_test_read_write(&mut clint, base_addr, CLINT_MTIME_L_OFFSET);
        do_test_read_write(&mut clint, base_addr, CLINT_MTIME_H_OFFSET);

        let addr = base_addr + CLINT_MSIP_OFFSET;
        assert_eq!(
            clint.read_byte(addr),
            Err(CpuPeripheralsError::DeviceReadFailed(addr as u64))
        );
    }

    #[test]
    fn test_clint_tick_msip() {
        let mut clint = Clint::new(create_csr());
        clint.mtimecmp = 2;
        clint.msip = 1;
        // let mut core = Core::new();

        let mut mip: u32 = 0;
        clint.tick(&mut mip);
        assert_eq!(clint.mtime, 1);
        // assert_eq!(core.read_csr(CSR_MIP).unwrap(), MIP_MSIP);
        assert_eq!(mip, MIP_MSIP);

        clint.tick(&mut mip);
        // assert_eq!(core.read_csr(CSR_MIP).unwrap(), MIP_MSIP | MIP_MTIP);
        assert_eq!(mip, MIP_MSIP | MIP_MTIP)
    }
}
