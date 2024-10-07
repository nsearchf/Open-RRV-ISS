// Copyright (c) 2024, zhao.shaowei <nsearchf@yeah.net>
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use tracing::trace;

use crate::{core::Core, MachineInstruction, RvCoreError};
use crate::{GprSigned, GprUnsigned};
use cpu_peripherals::bus::Bus;

use crate::decode::{self, disassemble::disasm_format_i, ExecutionReturnData};
pub(crate) fn execute_slli(
    raw: MachineInstruction,
    core: &mut Core,
    _bus: &mut Bus,
    disasm: bool,
) -> Result<Option<ExecutionReturnData>, RvCoreError> {
    execute_shift_instruction(raw, core, disasm, "SLLI", |rs1, shamt| {
        (rs1 << shamt) as GprUnsigned
    })
}

pub(crate) fn execute_srai(
    raw: MachineInstruction,
    core: &mut Core,
    _bus: &mut Bus,
    disasm: bool,
) -> Result<Option<ExecutionReturnData>, RvCoreError> {
    execute_shift_instruction(raw, core, disasm, "SRAI", |rs1, shamt| {
        (rs1 >> shamt) as GprUnsigned
    })
}

pub(crate) fn execute_srli(
    raw: MachineInstruction,
    core: &mut Core,
    _bus: &mut Bus,
    disasm: bool,
) -> Result<Option<ExecutionReturnData>, RvCoreError> {
    execute_shift_instruction(raw, core, disasm, "SRLI", |rs1, shamt| {
        (rs1 as GprUnsigned) >> (shamt as GprUnsigned)
    })
}

fn check_shamt(shamt: GprUnsigned) -> Result<(), RvCoreError> {
    if (shamt & 0x20) != 0 {
        Err(RvCoreError::ShamtIsInvalid(shamt))
    } else {
        Ok(())
    }
}

/// Common function for shift instructions
fn execute_shift_instruction(
    raw: MachineInstruction,
    core: &mut Core,
    disasm: bool,
    mnemonic: &'static str,
    operation: fn(GprSigned, GprSigned) -> GprUnsigned,
) -> Result<Option<ExecutionReturnData>, RvCoreError> {
    let operands = decode::parse_i_type(raw);
    trace!("Executing {} with operands: {:?}", mnemonic, operands);

    let rs1 = core.read_register(operands.rs1)?;
    let shamt = (operands.imm as GprUnsigned) & 0x3f;
    check_shamt(shamt)?;

    let val = operation(rs1 as GprSigned, shamt as GprSigned);

    let _ = core.write_register(operands.rd, val)?;

    if disasm {
        Ok(Some(ExecutionReturnData {
            pc: None,
            disasm: Some(disasm_format_i(mnemonic, &operands, core, false)),
        }))
    } else {
        Ok(None)
    }
}
