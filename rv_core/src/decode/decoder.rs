// Copyright (c) 2024, zhao.shaowei <nsearchf@yeah.net>
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

// rv_core/src/decoder.rs

// use std::collections::HashMap;
use tracing::info;

use crate::decode::all_instructions::ALL_INSTRUCTIONS;
use crate::decode::DecodedInstruction;
use crate::{MachineInstruction, RvCoreError};

pub struct Decoder {
    // instructions: HashMap<&'static str, InstructionsEntry>,
}

impl Decoder {
    pub fn new() -> Self {
        info!("Decoder created");
        Self {}
    }

    pub fn decode(&self, inst: MachineInstruction) -> Result<DecodedInstruction, RvCoreError> {
        for entry in ALL_INSTRUCTIONS {
            if (inst & entry.mask) != entry.match_val {
                continue;
            }

            return Ok(DecodedInstruction {
                name: entry.name,
                execute: entry.execute,
            });
        }
        Err(RvCoreError::InvalidInstruction(inst))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::execute::rv_i;
    #[test]
    fn test_decode_add() {
        let decoder = Decoder::new();
        let inst: u32 = 0b0000000_00010_00001_000_00011_0110011;
        let decoded = decoder.decode(inst).unwrap();
        assert_eq!(decoded.name, "ADD");
        if decoded.execute != rv_i::execute_add {
            debug_assert!(false, "Invalid execute function");
        }
    }
}
