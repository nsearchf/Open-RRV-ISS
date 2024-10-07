// Copyright (c) 2024, zhao.shaowei <nsearchf@yeah.net>
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

// src/trap.rs

use thiserror::Error;

use crate::csr::Csr;
use crate::decode::ExecutionReturnData;
use crate::inst_csr_reg::*;
use crate::{ProgramCounter, RvCoreError};

#[derive(Error, Debug, PartialEq)]
pub enum Exception {
    #[error("instruction address misaligned")]
    InstructionAddressMisaligned,

    #[error("instruction access fault")]
    InstructionAccessFault,

    #[error("illegal instruction: {0}")]
    IllegalInstruction(u32),

    #[error("breakpoint")]
    Breakpoint, // Environment Breakpoint

    #[error("load address misaligned")]
    LoadAddressMisaligned,

    #[error("load access fault")]
    LoadAccessFault,

    #[error("store/AMO address misaligned")]
    StoreAmoAddressMisaligned,

    #[error("store/AMO access fault")]
    StoreAmoAccessFault,

    #[error("environment call from U-mode")]
    ECallFromUMode,

    #[error("environment call from S-mode")]
    ECallFromSMode,

    #[error("environment call from M-mode")]
    ECallFromMMode,

    #[error("instruction page fault")]
    InstructionPageFault,

    #[error("load page fault")]
    LoadPageFault,

    #[error("store/AMO page fault")]
    StoreAmoPageFault,
}

#[derive(Error, Debug, PartialEq)]
pub enum Interrupt {
    #[error("supervisor software interrupt")]
    SupervisorSoftwareInterrupt,

    #[error("machine software interrupt")]
    MachineSoftwareInterrupt,

    #[error("supervisor timer interrupt")]
    SupervisorTimerInterrupt,

    #[error("machine timer interrupt")]
    MachineTimerInterrupt,

    #[error("supervisor external interrupt")]
    SupervisorExternalInterrupt,

    #[error("machine external interrupt")]
    MachineExternalInterrupt,
}

#[derive(Debug, PartialEq)]
pub enum Trap {
    Exception(Exception),
    Interrupt(Interrupt),
}

// mtval
// mstatus
// mepc, mcause
// mtvec
// mie, mip
impl Trap {
    pub(crate) fn handle_trap(
        &self,
        csr: &mut Csr,
        current_pc: ProgramCounter,
        new_pc: ProgramCounter,
        // core: &mut Core,
    ) -> Result<Option<ExecutionReturnData>, RvCoreError> {
        let mut tvec = csr.read(CSR_MTVEC)?;
        let tvec_mode = tvec & 0b11;
        tvec &= !0b11;

        match self {
            Trap::Exception(exception) => {
                csr.write(CSR_MEPC, current_pc as u32)?;
                self.set_mcause_for_exception(csr, exception)?;

                // TODO:
                // csr.write(CSR_MSTATUS, mstatus)?;
            }
            Trap::Interrupt(interrupt) => {
                csr.write(CSR_MEPC, new_pc as u32)?;
                self.set_mcause_for_insterrupt(csr, interrupt)?;

                // TODO
                // csr.write(CSR_MSTATUS, mstatus)?;
            }
        }

        let new_pc = match tvec_mode {
            0 => {
                // direct mode
                tvec
            }
            1 => {
                // vectored mode
                let cause_no = csr.read(CSR_MCAUSE)? & !(1 << 31);
                tvec + (cause_no as u32) * 4
            }
            _ => {
                return Err(RvCoreError::InvalidTrapMode(tvec_mode));
            }
        };

        // self.set_mstatus(csr, current_mode)?;
        Ok(Some(ExecutionReturnData {
            pc: Some(new_pc as ProgramCounter),
            disasm: None,
        }))
    }

    fn set_mcause_for_exception(
        &self,
        csr: &mut Csr,
        exception: &Exception,
    ) -> Result<(), RvCoreError> {
        let value = match exception {
            Exception::InstructionAddressMisaligned => 0,
            Exception::InstructionAccessFault => 1,
            Exception::IllegalInstruction(_instruction) => 2,
            Exception::Breakpoint => 3,
            Exception::LoadAddressMisaligned => 4,
            Exception::LoadAccessFault => 5,
            Exception::StoreAmoAddressMisaligned => 6,
            Exception::StoreAmoAccessFault => 7,
            Exception::ECallFromUMode => 8,
            Exception::ECallFromSMode => 9,
            Exception::ECallFromMMode => 11,
            Exception::InstructionPageFault => 12,
            Exception::LoadPageFault => 13,
            Exception::StoreAmoPageFault => 15,
        };

        csr.write(CSR_MCAUSE, value)?; // cause

        Ok(())
    }

    fn set_mcause_for_insterrupt(
        &self,
        csr: &mut Csr,
        interrupt: &Interrupt,
    ) -> Result<(), RvCoreError> {
        let value = match interrupt {
            Interrupt::SupervisorSoftwareInterrupt => 1,
            Interrupt::MachineSoftwareInterrupt => 3,
            Interrupt::SupervisorTimerInterrupt => 5,
            Interrupt::MachineTimerInterrupt => 7,
            Interrupt::SupervisorExternalInterrupt => 9,
            Interrupt::MachineExternalInterrupt => 11,
        } | (1 << 31);

        csr.write(CSR_MCAUSE, value)?; // cause
        Ok(())
    }
}
