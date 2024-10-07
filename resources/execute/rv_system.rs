// Copyright (c) 2024, zhao.shaowei <nsearchf@yeah.net>
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.



pub(crate) fn execute_mret(
  raw: MachineInstruction,
  core: &mut Core,
  _bus: &mut Bus,
  disasm: bool,
) -> Result<Option<ExecutionReturnData>, RvCoreError> {
  todo!("Not implemented")
}
pub(crate) fn execute_wfi(
  raw: MachineInstruction,
  core: &mut Core,
  _bus: &mut Bus,
  disasm: bool,
) -> Result<Option<ExecutionReturnData>, RvCoreError> {
  todo!("Not implemented")
}
