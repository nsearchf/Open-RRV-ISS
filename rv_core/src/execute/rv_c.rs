// Copyright (c) 2024, zhao.shaowei <nsearchf@yeah.net>
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use tracing::trace;

// use crate::trap::{Exception, Trap};
// use crate::GprSigned;
use crate::{core::Core, MachineInstruction, ProgramCounter, RvCoreError};
use cpu_peripherals::bus::Bus;

// use crate::decode::{
//     self,
//     disassemble::{
//         disasm_format_b, disasm_format_i, disasm_format_j, disasm_format_r, disasm_format_s,
//         disasm_format_u,
//     },
//     ExecutionReturnData,
// };
use crate::decode::ExecutionReturnData;

// pub(crate) fn execute_c_add(
//   raw: MachineInstruction,
//   core: &mut Core,
//   _bus: &mut Bus,
//   disasm: bool,
// ) -> Result<Option<ExecutionReturnData>, RvCoreError> {
//   todo!("Not implemented")
// }
// pub(crate) fn execute_c_addi(
//   raw: MachineInstruction,
//   core: &mut Core,
//   _bus: &mut Bus,
//   disasm: bool,
// ) -> Result<Option<ExecutionReturnData>, RvCoreError> {
//   todo!("Not implemented")
// }
// pub(crate) fn execute_c_addi16sp(
//   raw: MachineInstruction,
//   core: &mut Core,
//   _bus: &mut Bus,
//   disasm: bool,
// ) -> Result<Option<ExecutionReturnData>, RvCoreError> {
//   todo!("Not implemented")
// }
// pub(crate) fn execute_c_addi4spn(
//   raw: MachineInstruction,
//   core: &mut Core,
//   _bus: &mut Bus,
//   disasm: bool,
// ) -> Result<Option<ExecutionReturnData>, RvCoreError> {
//   todo!("Not implemented")
// }
// pub(crate) fn execute_c_and(
//   raw: MachineInstruction,
//   core: &mut Core,
//   _bus: &mut Bus,
//   disasm: bool,
// ) -> Result<Option<ExecutionReturnData>, RvCoreError> {
//   todo!("Not implemented")
// }
// pub(crate) fn execute_c_andi(
//   raw: MachineInstruction,
//   core: &mut Core,
//   _bus: &mut Bus,
//   disasm: bool,
// ) -> Result<Option<ExecutionReturnData>, RvCoreError> {
//   todo!("Not implemented")
// }
// pub(crate) fn execute_c_beqz(
//   raw: MachineInstruction,
//   core: &mut Core,
//   _bus: &mut Bus,
//   disasm: bool,
// ) -> Result<Option<ExecutionReturnData>, RvCoreError> {
//   todo!("Not implemented")
// }
// pub(crate) fn execute_c_bnez(
//   raw: MachineInstruction,
//   core: &mut Core,
//   _bus: &mut Bus,
//   disasm: bool,
// ) -> Result<Option<ExecutionReturnData>, RvCoreError> {
//   todo!("Not implemented")
// }
// pub(crate) fn execute_c_ebreak(
//   raw: MachineInstruction,
//   core: &mut Core,
//   _bus: &mut Bus,
//   disasm: bool,
// ) -> Result<Option<ExecutionReturnData>, RvCoreError> {
//   todo!("Not implemented")
// }
// pub(crate) fn execute_c_j(
//   raw: MachineInstruction,
//   core: &mut Core,
//   _bus: &mut Bus,
//   disasm: bool,
// ) -> Result<Option<ExecutionReturnData>, RvCoreError> {
//   todo!("Not implemented")
// }
// pub(crate) fn execute_c_jalr(
//   raw: MachineInstruction,
//   core: &mut Core,
//   _bus: &mut Bus,
//   disasm: bool,
// ) -> Result<Option<ExecutionReturnData>, RvCoreError> {
//   todo!("Not implemented")
// }
// pub(crate) fn execute_c_jr(
//   raw: MachineInstruction,
//   core: &mut Core,
//   _bus: &mut Bus,
//   disasm: bool,
// ) -> Result<Option<ExecutionReturnData>, RvCoreError> {
//   todo!("Not implemented")
// }
// pub(crate) fn execute_c_li(
//   raw: MachineInstruction,
//   core: &mut Core,
//   _bus: &mut Bus,
//   disasm: bool,
// ) -> Result<Option<ExecutionReturnData>, RvCoreError> {
//   todo!("Not implemented")
// }
// pub(crate) fn execute_c_lui(
//   raw: MachineInstruction,
//   core: &mut Core,
//   _bus: &mut Bus,
//   disasm: bool,
// ) -> Result<Option<ExecutionReturnData>, RvCoreError> {
//   todo!("Not implemented")
// }
// pub(crate) fn execute_c_lw(
//   raw: MachineInstruction,
//   core: &mut Core,
//   _bus: &mut Bus,
//   disasm: bool,
// ) -> Result<Option<ExecutionReturnData>, RvCoreError> {
//   todo!("Not implemented")
// }
// pub(crate) fn execute_c_lwsp(
//   raw: MachineInstruction,
//   core: &mut Core,
//   _bus: &mut Bus,
//   disasm: bool,
// ) -> Result<Option<ExecutionReturnData>, RvCoreError> {
//   todo!("Not implemented")
// }
// pub(crate) fn execute_c_mv(
//   raw: MachineInstruction,
//   core: &mut Core,
//   _bus: &mut Bus,
//   disasm: bool,
// ) -> Result<Option<ExecutionReturnData>, RvCoreError> {
//   todo!("Not implemented")
// }
pub(crate) fn execute_c_nop(
    _raw: MachineInstruction,
    core: &mut Core,
    _bus: &mut Bus,
    disasm: bool,
) -> Result<Option<ExecutionReturnData>, RvCoreError> {
    trace!("Executing c.nop");
    let current_pc = core.get_pc();
    let new_pc = current_pc.wrapping_add(2) as ProgramCounter;
    if disasm {
        Ok(Some(ExecutionReturnData {
            pc: Some(new_pc),
            disasm: Some("C.NOP".to_string()),
        }))
    } else {
        Ok(Some(ExecutionReturnData {
            pc: Some(new_pc),
            disasm: None,
        }))
    }
}
// pub(crate) fn execute_c_or(
//   raw: MachineInstruction,
//   core: &mut Core,
//   _bus: &mut Bus,
//   disasm: bool,
// ) -> Result<Option<ExecutionReturnData>, RvCoreError> {
//   todo!("Not implemented")
// }
// pub(crate) fn execute_c_sub(
//   raw: MachineInstruction,
//   core: &mut Core,
//   _bus: &mut Bus,
//   disasm: bool,
// ) -> Result<Option<ExecutionReturnData>, RvCoreError> {
//   todo!("Not implemented")
// }
// pub(crate) fn execute_c_sw(
//   raw: MachineInstruction,
//   core: &mut Core,
//   _bus: &mut Bus,
//   disasm: bool,
// ) -> Result<Option<ExecutionReturnData>, RvCoreError> {
//   todo!("Not implemented")
// }
// pub(crate) fn execute_c_swsp(
//   raw: MachineInstruction,
//   core: &mut Core,
//   _bus: &mut Bus,
//   disasm: bool,
// ) -> Result<Option<ExecutionReturnData>, RvCoreError> {
//   todo!("Not implemented")
// }
// pub(crate) fn execute_c_xor(
//   raw: MachineInstruction,
//   core: &mut Core,
//   _bus: &mut Bus,
//   disasm: bool,
// ) -> Result<Option<ExecutionReturnData>, RvCoreError> {
//   todo!("Not implemented")
// }
