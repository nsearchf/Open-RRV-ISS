// Copyright (c) 2024, zhao.shaowei <nsearchf@yeah.net>
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use tracing::trace;

use crate::{
    core::Core,
    csr::{CsrAddrType, CsrError},
    GprUnsigned, MachineInstruction, RvCoreError,
};
use cpu_peripherals::bus::Bus;

use crate::decode::{self, disassemble::disasm_format_i, ExecutionReturnData};

pub(crate) fn execute_csrrc(
    raw: MachineInstruction,
    core: &mut Core,
    _bus: &mut Bus,
    disasm: bool,
) -> Result<Option<ExecutionReturnData>, RvCoreError> {
    execute_csr_instruction(raw, core, disasm, "CSRRC", |core, address, value| {
        core.get_csr_mut().csrrc(address, value as u32)
    })
}

pub(crate) fn execute_csrrci(
    raw: MachineInstruction,
    core: &mut Core,
    _bus: &mut Bus,
    disasm: bool,
) -> Result<Option<ExecutionReturnData>, RvCoreError> {
    execute_csr_instruction(raw, core, disasm, "CSRRCI", |core, address, value| {
        core.get_csr_mut().csrrc(address, value as u32)
    })
}

pub(crate) fn execute_csrrs(
    raw: MachineInstruction,
    core: &mut Core,
    _bus: &mut Bus,
    disasm: bool,
) -> Result<Option<ExecutionReturnData>, RvCoreError> {
    execute_csr_instruction(raw, core, disasm, "CSRRS", |core, address, value| {
        core.get_csr_mut().csrrs(address, value as u32)
    })
}

pub(crate) fn execute_csrrsi(
    raw: MachineInstruction,
    core: &mut Core,
    _bus: &mut Bus,
    disasm: bool,
) -> Result<Option<ExecutionReturnData>, RvCoreError> {
    execute_csr_instruction(raw, core, disasm, "CSRRSI", |core, address, value| {
        core.get_csr_mut().csrrs(address, value as u32)
    })
}

pub(crate) fn execute_csrrw(
    raw: MachineInstruction,
    core: &mut Core,
    _bus: &mut Bus,
    disasm: bool,
) -> Result<Option<ExecutionReturnData>, RvCoreError> {
    execute_csr_instruction(raw, core, disasm, "CSRRW", |core, address, value| {
        core.get_csr_mut().csrrw(address, value as u32)
    })
}

pub(crate) fn execute_csrrwi(
    raw: MachineInstruction,
    core: &mut Core,
    _bus: &mut Bus,
    disasm: bool,
) -> Result<Option<ExecutionReturnData>, RvCoreError> {
    execute_csr_instruction(raw, core, disasm, "CSRRWI", |core, address, value| {
        core.get_csr_mut().csrrw(address, value as u32)
    })
}

/// Common function for executing CSR instructions.
fn execute_csr_instruction<F>(
    raw: MachineInstruction,
    core: &mut Core,
    disasm: bool,
    mnemonic: &'static str,
    operation: F,
) -> Result<Option<ExecutionReturnData>, RvCoreError>
where
    F: FnOnce(&mut Core, CsrAddrType, GprUnsigned) -> Result<u32, CsrError>,
{
    let operands = decode::parse_i_type(raw);
    trace!("Executing {} with operands: {:?}", mnemonic, operands);

    let address = (operands.imm as GprUnsigned & 0xfff) as CsrAddrType;
    let value = if operands.rs1 == 0 {
        operands.rs1 as GprUnsigned
    } else {
        core.read_register(operands.rs1)?
    };

    let old_val = operation(core, address, value)?;
    core.write_register(operands.rd, old_val)?;

    if disasm {
        Ok(Some(ExecutionReturnData {
            pc: None,
            disasm: Some(disasm_format_i(mnemonic, &operands, core, false)),
        }))
    } else {
        Ok(None)
    }
}
