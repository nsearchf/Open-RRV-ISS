// Copyright (c) 2024, zhao.shaowei <nsearchf@yeah.net>
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use tracing::{trace, warn};

use crate::inst_csr_reg::*;
use crate::{
    core::{self, Core},
    csr,
    decode::ExecutionReturnData,
    MachineInstruction, ProgramCounter, RvCoreError,
};
use cpu_peripherals::bus::Bus;

// Sets the pc to CSRs[mepc],
// the privilege mode to CSRs[mstatus].MPP,
// CSRs[mstatus].MIE to CSRs[mstatus].MPIE, and
// CSRs[mstatus].MPIE to 1;
// and, if user mode is supported, sets CSRs[mstatus].MPP to 0.

pub(crate) fn execute_mret(
    _raw: MachineInstruction,
    core: &mut Core,
    _bus: &mut Bus,
    disasm: bool,
) -> Result<Option<ExecutionReturnData>, RvCoreError> {
    trace!("Executing MRET instruction");
    let csr = core.get_csr_mut();

    let epc = csr.read(CSR_MEPC)?;
    let status = csr.read(CSR_MSTATUS)?;

    let mpie = (status & csr::MSTATUS_MPIE) >> 7; // (status >> 7) & 1;
    let mpp = (status & csr::MSTATUS_MPP) >> 11; //(status >> 11) & 0x3;

    // MPRV (Modify PRiVilege) bit
    let mprv = match core::get_privilege_mode((mpp >> 11) as u8) {
        core::PrivilegeMode::Machine => (status & csr::MSTATUS_MPRV) >> 17,
        _ => 0,
    };
    // Override MIE[3] with MPIE[7], set MPIE[7] to 1, set MPP[12:11] to 0
    // and override MPRV[17]
    let new_status = (status & !0x21888) | (mprv << 17) | (mpie << 3) | (1 << 7);
    csr.write(CSR_MSTATUS, new_status)?;

    let privilege_mode = match mpp {
        0 => core::PrivilegeMode::User,
        1 => core::PrivilegeMode::Supervisor,
        3 => core::PrivilegeMode::Machine,
        _ => panic!(), // Shouldn't happen
    };

    core.set_privilege_mode(privilege_mode);

    let new_pc = epc as ProgramCounter;
    if disasm {
        Ok(Some(ExecutionReturnData {
            pc: Some(new_pc),
            disasm: Some("MRET".to_string()),
        }))
    } else {
        Ok(Some(ExecutionReturnData {
            pc: Some(new_pc),
            disasm: None,
        }))
    }
}

pub(crate) fn execute_wfi(
    _raw: MachineInstruction,
    _core: &mut Core,
    _bus: &mut Bus,
    disasm: bool,
) -> Result<Option<ExecutionReturnData>, RvCoreError> {
    trace!("Executing WFI");

    warn!("WFI not implemented");

    if disasm {
        Ok(Some(ExecutionReturnData {
            pc: None,
            disasm: Some("WFI".to_string()),
        }))
    } else {
        Ok(None)
    }
}
