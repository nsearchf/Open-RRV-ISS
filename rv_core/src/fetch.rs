// Copyright (c) 2024, zhao.shaowei <nsearchf@yeah.net>
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

// rv_core/src/fetch.rs

use tracing::trace;

use crate::{MachineInstruction, ProgramCounter, RvCoreError};
use cpu_peripherals::Device;

pub struct Fetcher;

impl Fetcher {
    pub fn fetch(
        pc: ProgramCounter,
        mem: &Box<dyn Device>,
    ) -> Result<MachineInstruction, RvCoreError> {
        // Fetch the instruction from the bus
        trace!("Fetching instruction at PC: {:#010x}", pc);
        let instr = mem.read_word(pc.try_into().unwrap()).expect("fetch failed");
        Ok(instr)
    }
}
