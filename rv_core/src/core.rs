// Copyright (c) 2024, zhao.shaowei <nsearchf@yeah.net>
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

// rv_core/src/core.rs

use std::collections::HashMap;

use tracing::info;

use crate::decode::ExecutionReturnData;
use crate::inst_csr_reg::*;
use crate::trap::{Exception, Trap};
use crate::{
    csr::{self, Csr},
    GprUnsigned, ProgramCounter, RegisterIndex, RvCoreError,
};

const REGISTER_NUM: usize = 32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RegName {
    Zero,
    Ra,
    Sp,
    Gp,
    Tp,
    T0,
    T1,
    T2,
    S0,
    S1,
    A0,
    A1,
    A2,
    A3,
    A4,
    A5,
    A6,
    A7,
    S2,
    S3,
    S4,
    S5,
    S6,
    S7,
    S8,
    S9,
    S10,
    S11,
    T3,
    T4,
    T5,
    T6,
}

impl RegName {
    pub fn to_index(self) -> usize {
        self as usize
    }

    pub fn from_index(index: RegisterIndex) -> Option<RegName> {
        match index {
            0 => Some(RegName::Zero),
            1 => Some(RegName::Ra),
            2 => Some(RegName::Sp),
            3 => Some(RegName::Gp),
            4 => Some(RegName::Tp),
            5 => Some(RegName::T0),
            6 => Some(RegName::T1),
            7 => Some(RegName::T2),
            8 => Some(RegName::S0),
            9 => Some(RegName::S1),
            10 => Some(RegName::A0),
            11 => Some(RegName::A1),
            12 => Some(RegName::A2),
            13 => Some(RegName::A3),
            14 => Some(RegName::A4),
            15 => Some(RegName::A5),
            16 => Some(RegName::A6),
            17 => Some(RegName::A7),
            18 => Some(RegName::S2),
            19 => Some(RegName::S3),
            20 => Some(RegName::S4),
            21 => Some(RegName::S5),
            22 => Some(RegName::S6),
            23 => Some(RegName::S7),
            24 => Some(RegName::S8),
            25 => Some(RegName::S9),
            26 => Some(RegName::S10),
            27 => Some(RegName::S11),
            28 => Some(RegName::T3),
            29 => Some(RegName::T4),
            30 => Some(RegName::T5),
            31 => Some(RegName::T6),
            _ => None,
        }
    }

    pub fn to_string(self) -> &'static str {
        match self {
            RegName::Zero => "zero",
            RegName::Ra => "ra",
            RegName::Sp => "sp",
            RegName::Gp => "gp",
            RegName::Tp => "tp",
            RegName::T0 => "t0",
            RegName::T1 => "t1",
            RegName::T2 => "t2",
            RegName::S0 => "s0",
            RegName::S1 => "s1",
            RegName::A0 => "a0",
            RegName::A1 => "a1",
            RegName::A2 => "a2",
            RegName::A3 => "a3",
            RegName::A4 => "a4",
            RegName::A5 => "a5",
            RegName::A6 => "a6",
            RegName::A7 => "a7",
            RegName::S2 => "s2",
            RegName::S3 => "s3",
            RegName::S4 => "s4",
            RegName::S5 => "s5",
            RegName::S6 => "s6",
            RegName::S7 => "s7",
            RegName::S8 => "s8",
            RegName::S9 => "s9",
            RegName::S10 => "s10",
            RegName::S11 => "s11",
            RegName::T3 => "t3",
            RegName::T4 => "t4",
            RegName::T5 => "t5",
            RegName::T6 => "t6",
        }
    }
}

impl From<RegName> for usize {
    fn from(reg: RegName) -> Self {
        reg as usize
    }
}

pub struct Core {
    pc: ProgramCounter,
    registers: [GprUnsigned; REGISTER_NUM],
    csr: Csr,
    reg_name_map: HashMap<String, RegName>,
    trap: Option<Trap>,
    privilege_mode: PrivilegeMode,
}

pub enum PrivilegeMode {
    User,
    Supervisor,
    Reserved,
    Machine,
}

pub fn get_privilege_mode_name(mode: &PrivilegeMode) -> &'static str {
    match mode {
        PrivilegeMode::User => "User",
        PrivilegeMode::Supervisor => "Supervisor",
        PrivilegeMode::Reserved => "Reserved",
        PrivilegeMode::Machine => "Machine",
    }
}

// bigger number is higher privilege level
pub fn get_privilege_encoding(mode: &PrivilegeMode) -> u8 {
    match mode {
        PrivilegeMode::User => 0,
        PrivilegeMode::Supervisor => 1,
        PrivilegeMode::Reserved => panic!(),
        PrivilegeMode::Machine => 3,
    }
}

/// Returns `PrivilegeMode` from encoded privilege mode bits
pub fn get_privilege_mode(encoding: u8) -> PrivilegeMode {
    match encoding {
        0 => PrivilegeMode::User,
        1 => PrivilegeMode::Supervisor,
        3 => PrivilegeMode::Machine,
        _ => panic!("Unknown privilege uncoding"),
    }
}

impl Core {
    pub fn new() -> Self {
        info!("Creating a new core");
        Self {
            pc: 0,
            registers: [0; REGISTER_NUM],
            csr: Csr::new(),
            reg_name_map: Self::new_reg_name_map(),
            trap: None,
            privilege_mode: PrivilegeMode::Machine,
        }
    }

    pub fn reset(&mut self) {
        self.pc = 0;
        self.registers = [0; REGISTER_NUM];
        self.csr.reset();
        self.reg_name_map = Self::new_reg_name_map();
        self.trap = None;
        self.privilege_mode = PrivilegeMode::Machine;
    }

    pub fn get_reg_name_by_index(&self, index: RegisterIndex) -> Option<&'static str> {
        if let Some(reg_name) = RegName::from_index(index) {
            Some(reg_name.to_string())
        } else {
            None
        }
    }

    pub fn get_pc(&self) -> ProgramCounter {
        self.pc
    }
    pub fn set_pc(&mut self, pc: ProgramCounter) {
        self.pc = pc;
    }

    pub fn read_register(&self, index: RegisterIndex) -> Result<GprUnsigned, RvCoreError> {
        if index >= REGISTER_NUM {
            return Err(RvCoreError::InvalidRegisterIndex(index as RegisterIndex));
        } else if index == 0 {
            return Ok(0);
        }

        Ok(self.registers[index])
    }

    pub fn write_register(
        &mut self,
        index: RegisterIndex,
        value: GprUnsigned,
    ) -> Result<(), RvCoreError> {
        if index >= REGISTER_NUM {
            return Err(RvCoreError::InvalidRegisterIndex(index as RegisterIndex));
        } else if index == 0 {
            return Ok(());
        }

        self.registers[index] = value;
        Ok(())
    }

    // Read register by name
    pub fn read_reg_by_name(&self, name: &str) -> Result<GprUnsigned, RvCoreError> {
        let tmp = self
            .reg_name_map
            .get(name)
            .map(|&reg| self.registers[reg.to_index()]);
        if let Some(value) = tmp {
            return Ok(value);
        }
        Err(RvCoreError::InvalidRegisterName(name.to_string()))
    }

    // Write register by name
    pub fn write_reg_by_name(&mut self, name: &str, value: GprUnsigned) -> Result<(), RvCoreError> {
        if let Some(&reg) = self.reg_name_map.get(name) {
            if reg != RegName::Zero {
                self.registers[reg.to_index()] = value;
            }
            return Ok(());
        }

        Err(RvCoreError::InvalidRegisterName(name.to_string()))
    }

    pub fn read_csr(&self, addr: csr::CsrAddrType) -> Result<u32, csr::CsrError> {
        self.csr.read(addr)
    }

    pub(crate) fn set_trap(&mut self, trap: Trap, tval: u32) -> Result<(), RvCoreError> {
        self.csr.write(CSR_MTVAL, tval)?;
        self.trap = Some(trap);
        Ok(())
    }

    pub fn is_ecall(trap: &Trap) -> bool {
        match trap {
            Trap::Exception(Exception::ECallFromUMode)
            | Trap::Exception(Exception::ECallFromSMode)
            | Trap::Exception(Exception::ECallFromMMode) => true,
            _ => false,
        }
    }

    pub fn take_trap(&mut self) -> Option<Trap> {
        self.trap.take()
    }

    pub fn handle_trap(
        &mut self,
        trap: &Trap,
        new_pc: ProgramCounter,
    ) -> Result<Option<ExecutionReturnData>, RvCoreError> {
        self.set_mstatus_before_handle_trap()?;
        let current_pc = self.get_pc();
        trap.handle_trap(&mut self.csr, current_pc, new_pc)
    }

    pub(crate) fn get_csr_mut(&mut self) -> &mut Csr {
        &mut self.csr
    }

    pub(crate) fn set_privilege_mode(&mut self, mode: PrivilegeMode) {
        self.privilege_mode = mode;
    }

    pub(crate) fn get_privilege_mode(&self) -> &PrivilegeMode {
        &self.privilege_mode
    }

    fn set_mstatus_before_handle_trap(&mut self) -> Result<(), RvCoreError> {
        let old_val = self.csr.read(CSR_MSTATUS)?;
        let current_mode = self.get_privilege_mode();
        // Save the privilege mode before the trap into mstatus.MPP
        let mut new_value =
            (old_val & !(csr::MSTATUS_MPP)) | ((get_privilege_encoding(current_mode) as u32) << 11);

        // Change the privilege mode to Machine mode.
        self.set_privilege_mode(PrivilegeMode::Machine);

        // Save the previous mstatus.MIE into mstatus.MPIE
        new_value = (new_value & !(csr::MSTATUS_MPIE)) | ((old_val & csr::MSTATUS_MIE) << 4);

        // Set mstatus.MIE to zero to disable interrupts
        new_value &= !(csr::MSTATUS_MIE);

        self.csr.write(CSR_MSTATUS, new_value)?; // cause

        Ok(())
    }

    fn new_reg_name_map() -> HashMap<String, RegName> {
        let mut reg_map = HashMap::new();
        reg_map.insert("zero".to_string(), RegName::Zero);
        reg_map.insert("ra".to_string(), RegName::Ra);
        reg_map.insert("sp".to_string(), RegName::Sp);
        reg_map.insert("gp".to_string(), RegName::Gp);
        reg_map.insert("tp".to_string(), RegName::Tp);
        reg_map.insert("t0".to_string(), RegName::T0);
        reg_map.insert("t1".to_string(), RegName::T1);
        reg_map.insert("t2".to_string(), RegName::T2);
        reg_map.insert("s0".to_string(), RegName::S0);
        reg_map.insert("s1".to_string(), RegName::S1);
        reg_map.insert("a0".to_string(), RegName::A0);
        reg_map.insert("a1".to_string(), RegName::A1);
        reg_map.insert("a2".to_string(), RegName::A2);
        reg_map.insert("a3".to_string(), RegName::A3);
        reg_map.insert("a4".to_string(), RegName::A4);
        reg_map.insert("a5".to_string(), RegName::A5);
        reg_map.insert("a6".to_string(), RegName::A6);
        reg_map.insert("a7".to_string(), RegName::A7);
        reg_map.insert("s2".to_string(), RegName::S2);
        reg_map.insert("s3".to_string(), RegName::S3);
        reg_map.insert("s4".to_string(), RegName::S4);
        reg_map.insert("s5".to_string(), RegName::S5);
        reg_map.insert("s6".to_string(), RegName::S6);
        reg_map.insert("s7".to_string(), RegName::S7);
        reg_map.insert("s8".to_string(), RegName::S8);
        reg_map.insert("s9".to_string(), RegName::S9);
        reg_map.insert("s10".to_string(), RegName::S10);
        reg_map.insert("s11".to_string(), RegName::S11);
        reg_map.insert("t3".to_string(), RegName::T3);
        reg_map.insert("t4".to_string(), RegName::T4);
        reg_map.insert("t5".to_string(), RegName::T5);
        reg_map.insert("t6".to_string(), RegName::T6);
        reg_map
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::trap::{Exception, Trap};
    use crate::RvCoreError;

    #[test]
    fn test_core_new() {
        let core = Core::new();
        assert_eq!(core.get_pc(), 0);
        for register in core.registers.iter() {
            assert_eq!(*register, 0);
        }
    }

    #[test]
    fn test_set_take_trap() {
        let mut core = Core::new();
        assert_eq!(core.take_trap(), None);
        let _ = core.set_trap(Trap::Exception(Exception::InstructionAddressMisaligned), 0);
        assert_eq!(
            core.take_trap(),
            Some(Trap::Exception(Exception::InstructionAddressMisaligned))
        );
        assert_eq!(core.take_trap(), None);
    }

    #[test]
    fn test_read_register_zero_index() {
        let core = Core::new();
        let result = core.read_register(0);
        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    fn test_write_register_zero_index() {
        let mut core = Core::new();
        core.write_register(0, 42).unwrap();
        let result = core.read_register(0);
        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    fn test_read_register_valid_index() {
        let mut core = Core::new();
        core.registers[1] = 42;
        let result = core.read_register(1);
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn test_read_register_invalid_index() {
        let core = Core::new();
        let result = core.read_register(32);
        assert_eq!(
            result.unwrap_err(),
            RvCoreError::InvalidRegisterIndex(32 as RegisterIndex)
        );
    }

    #[test]
    fn test_write_register_valid_index() {
        let mut core = Core::new();
        core.write_register(1, 42).unwrap();
        assert_eq!(core.registers[1], 42);
    }

    #[test]
    fn test_write_register_invalid_index() {
        let mut core = Core::new();
        let result = core.write_register(32, 42);
        assert_eq!(
            result.unwrap_err(),
            RvCoreError::InvalidRegisterIndex(32 as RegisterIndex)
        );
    }

    #[test]
    fn test_new_reg_name_map() {
        let map = Core::new_reg_name_map();
        assert_eq!(map.get("zero"), Some(&RegName::Zero));
        assert_eq!(map.get("ra"), Some(&RegName::Ra));
        assert_eq!(map.get("sp"), Some(&RegName::Sp));
        assert_eq!(map.get("gp"), Some(&RegName::Gp));
        assert_eq!(map.get("tp"), Some(&RegName::Tp));
        assert_eq!(map.get("t0"), Some(&RegName::T0));
        assert_eq!(map.get("t1"), Some(&RegName::T1));
        assert_eq!(map.get("t2"), Some(&RegName::T2));
        assert_eq!(map.get("s0"), Some(&RegName::S0));
        assert_eq!(map.get("s1"), Some(&RegName::S1));
        assert_eq!(map.get("a0"), Some(&RegName::A0));
        assert_eq!(map.get("a1"), Some(&RegName::A1));
        assert_eq!(map.get("a2"), Some(&RegName::A2));
        assert_eq!(map.get("a3"), Some(&RegName::A3));
        assert_eq!(map.get("a4"), Some(&RegName::A4));
        assert_eq!(map.get("a5"), Some(&RegName::A5));
        assert_eq!(map.get("a6"), Some(&RegName::A6));
        assert_eq!(map.get("a7"), Some(&RegName::A7));
        assert_eq!(map.get("s2"), Some(&RegName::S2));
        assert_eq!(map.get("s3"), Some(&RegName::S3));
        assert_eq!(map.get("s4"), Some(&RegName::S4));
        assert_eq!(map.get("s5"), Some(&RegName::S5));
        assert_eq!(map.get("s6"), Some(&RegName::S6));
        assert_eq!(map.get("s7"), Some(&RegName::S7));
        assert_eq!(map.get("s8"), Some(&RegName::S8));
        assert_eq!(map.get("s9"), Some(&RegName::S9));
        assert_eq!(map.get("s10"), Some(&RegName::S10));
        assert_eq!(map.get("s11"), Some(&RegName::S11));
        assert_eq!(map.get("t3"), Some(&RegName::T3));
        assert_eq!(map.get("t4"), Some(&RegName::T4));
        assert_eq!(map.get("t5"), Some(&RegName::T5));
        assert_eq!(map.get("t6"), Some(&RegName::T6));
    }

    #[test]
    fn test_reg_name_to_index() {
        let cases = [
            (RegName::Zero, 0),
            (RegName::Ra, 1),
            (RegName::Sp, 2),
            (RegName::Gp, 3),
            (RegName::Tp, 4),
            (RegName::T0, 5),
            (RegName::T1, 6),
            (RegName::T2, 7),
            (RegName::S0, 8),
            (RegName::S1, 9),
            (RegName::A0, 10),
            (RegName::A1, 11),
            (RegName::A2, 12),
            (RegName::A3, 13),
            (RegName::A4, 14),
            (RegName::A5, 15),
            (RegName::A6, 16),
            (RegName::A7, 17),
            (RegName::S2, 18),
            (RegName::S3, 19),
            (RegName::S4, 20),
            (RegName::S5, 21),
            (RegName::S6, 22),
            (RegName::S7, 23),
            (RegName::S8, 24),
            (RegName::S9, 25),
            (RegName::S10, 26),
            (RegName::S11, 27),
            (RegName::T3, 28),
            (RegName::T4, 29),
            (RegName::T5, 30),
            (RegName::T6, 31),
        ];

        for (reg_name, expected_index) in cases {
            assert_eq!(reg_name.to_index(), expected_index);
        }
    }

    #[test]
    fn test_read_reg_by_regname_enum() {
        let core = Core::new();
        assert_eq!(core.read_register(RegName::Zero.into()), Ok(0));
    }

    #[test]
    fn test_read_write_reg_by_name() {
        let mut core = Core::new();
        let test_val: GprUnsigned = 42;
        let reg_name = "t0";
        let _ = core.write_reg_by_name(reg_name, test_val);
        assert_eq!(core.read_reg_by_name(reg_name), Ok(test_val));
    }
}
