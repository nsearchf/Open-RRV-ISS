// Copyright (c) 2024, zhao.shaowei <nsearchf@yeah.net>
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

pub mod decoder;
pub(crate) mod disassemble;

mod all_instructions;
mod inst_rv32_i;
mod inst_rv_i;
mod inst_rv_system;
mod inst_rv_zicsr;

use crate::{
    core::Core, GprSigned, GprUnsigned, MachineInstruction, ProgramCounter, RegisterIndex,
    RvCoreError,
};
use cpu_peripherals::bus::Bus;

pub type ExecuteInstructionFn = fn(
    MachineInstruction,
    &mut Core,
    &mut Bus,
    bool,
) -> Result<Option<ExecutionReturnData>, RvCoreError>;

pub struct ExecutionReturnData {
    pub pc: Option<ProgramCounter>,
    pub disasm: Option<String>,
}

pub struct DecodedInstruction {
    pub name: &'static str,
    pub execute: ExecuteInstructionFn,
}

struct InstructionsEntry {
    pub name: &'static str,
    pub mask: MachineInstruction,
    pub match_val: MachineInstruction,
    pub execute: ExecuteInstructionFn,
}

#[derive(Debug, PartialEq)]
pub struct FormatR {
    pub rd: RegisterIndex,
    pub rs1: RegisterIndex,
    pub rs2: RegisterIndex,
}

#[derive(Debug, PartialEq)]
pub struct FormatI {
    pub rd: RegisterIndex,
    pub rs1: RegisterIndex,
    pub imm: GprSigned,
}

#[derive(Debug, PartialEq)]
pub struct FormatS {
    pub rs1: RegisterIndex,
    pub rs2: RegisterIndex,
    pub imm: GprSigned,
}

#[derive(Debug, PartialEq)]
pub struct FormatB {
    pub rs1: RegisterIndex,
    pub rs2: RegisterIndex,
    pub imm: GprSigned,
}

#[derive(Debug, PartialEq)]
pub struct FormatU {
    pub rd: RegisterIndex,
    pub imm: GprSigned,
}
#[derive(Debug, PartialEq)]
pub struct FormatJ {
    pub rd: RegisterIndex,
    pub imm: GprSigned,
}

pub(crate) fn parse_r_type(inst: MachineInstruction) -> FormatR {
    FormatR {
        rd: ((inst >> 7) & 0x1f) as RegisterIndex,
        rs1: ((inst >> 15) & 0x1f) as RegisterIndex,
        rs2: ((inst >> 20) & 0x1f) as RegisterIndex,
    }
}

// Constant representing the sign bit
const SIGN_BIT: u32 = 0x8000_0000;

pub(crate) fn sign_extend_clear_data_bits(data_bits_mask: u32) -> GprUnsigned {
    (-1 as GprSigned as GprUnsigned) & !(data_bits_mask as GprUnsigned)
}

pub(crate) fn parse_i_type(inst: MachineInstruction) -> FormatI {
    const DATA_BITS_MASK: u32 = 0x7ff;

    // sign-extend immediate
    // imm[xlen-1]:11] = inst[31]
    let sign_bits = if (inst & SIGN_BIT) != 0 {
        sign_extend_clear_data_bits(DATA_BITS_MASK)
    } else {
        0
    };

    // imm[10:0] = inst[30:20]
    let data_bits = ((inst >> 20) & DATA_BITS_MASK) as GprUnsigned;

    let imm = (sign_bits | data_bits) as GprSigned;

    FormatI {
        rd: ((inst >> 7) & 0x1f) as RegisterIndex,
        rs1: ((inst >> 15) & 0x1f) as RegisterIndex,
        imm,
    }
}

pub(crate) fn parse_b_type(inst: MachineInstruction) -> FormatB {
    const DATA_BITS_MASK: u32 = 0xfff;

    // sign-extend immediate
    // imm[xlen-1]:12] = inst[31]
    let sign_bits = if (inst & SIGN_BIT) != 0 {
        sign_extend_clear_data_bits(DATA_BITS_MASK)
    } else {
        0
    };

    let data_bits = (((inst << 4) & 0x0800) | // imm[11] = inst[7]
        ((inst >> 20) & 0x07e0) | // imm[10:5] = inst[30:25]
        ((inst >> 7) & 0x001e)) // imm[4:1] = inst[11:8]
        as GprUnsigned;

    let imm = (sign_bits | data_bits) as GprSigned;

    FormatB {
        rs1: ((inst >> 15) & 0x1f) as RegisterIndex,
        rs2: ((inst >> 20) & 0x1f) as RegisterIndex,
        imm,
    }
}

pub(crate) fn parse_s_type(inst: MachineInstruction) -> FormatS {
    const DATA_BITS_MASK: u32 = 0x7ff;

    // sign-extend immediate
    // imm[xlen-1]:11] = inst[31]
    let sign_bits = if (inst & SIGN_BIT) != 0 {
        sign_extend_clear_data_bits(DATA_BITS_MASK)
    } else {
        0
    };

    let data_bits = (((inst >> 20) & 0x7e0) | // imm[10:5] = inst[30:25]
        ((inst >> 7) & 0x1f)) // imm[4:0] = inst[11:7]
        as GprUnsigned;

    let imm = (sign_bits | data_bits) as GprSigned;

    FormatS {
        rs1: ((inst >> 15) & 0x1f) as RegisterIndex,
        rs2: ((inst >> 20) & 0x1f) as RegisterIndex,
        imm,
    }
}

pub(crate) fn parse_u_type(inst: MachineInstruction) -> FormatU {
    const DATA_BITS_MASK: u32 = 0x7fff_ffff;

    // sign-extend immediate
    // imm[xlen-1]:31] = inst[31]
    let sign_bits = if (inst & SIGN_BIT) != 0 {
        sign_extend_clear_data_bits(DATA_BITS_MASK)
    } else {
        0
    };

    // imm[30:12] = inst[30:12]
    let data_bits = (inst & 0x7fff_f000) as GprUnsigned;

    let imm = (sign_bits | data_bits) as GprSigned;

    FormatU {
        rd: ((inst >> 7) & 0x1f) as RegisterIndex,
        imm,
    }
}

pub(crate) fn parse_j_type(inst: MachineInstruction) -> FormatJ {
    const DATA_BITS_MASK: u32 = 0xf_ffff;
    // sign-extend immediate
    // imm[xlen-1]:20] = inst[31]
    let sign_bits = if (inst & SIGN_BIT) != 0 {
        sign_extend_clear_data_bits(DATA_BITS_MASK)
    } else {
        0
    };

    let data_bits = ((inst & 0xf_f000) | // imm[19:12] = inst[19:12]
        ((inst & 0x0010_0000) >> 9) | // imm[11] = inst[20]
        ((inst & 0x7fe0_0000) >> 20)) // imm[10:1] = inst[30:21]
        as GprUnsigned;

    let imm = (sign_bits | data_bits) as GprSigned;

    FormatJ {
        rd: ((inst >> 7) & 0x1f) as RegisterIndex,
        imm,
    }
}

//
// Unit tests for parsing instruction formats
//
// temp_test:
//     # R, I, S, B, U, J
//     add x12, x13, x14 # add rd, rs1, rs2
// beq_target:
//     addi x15, x16, -2 # addi rd, rs1, imm
//     sb x5, -3(x6)     # sb rs2, offset(rs1)
// jal_target:
//     beq x18, x19, beq_target # beq rs1, rs2, offset
//     auipc x20, 0x1234        # auipc rd, imm
//     jal x1, jal_target       # jal rd, offset
//
// 00010014 <temp_test>:
// temp_test():
//    10014:	00e68633          	add	a2,a3,a4
// 00010018 <beq_target>:
// beq_target():
//    10018:	ffe80793          	addi	a5,a6,-2
//    1001c:	fe530ea3          	sb	t0,-3(t1)
// 00010020 <jal_target>:
// jal_target():
//    10020:	ff390ce3          	beq	s2,s3,10018 <beq_target>
//    10024:	01234a17          	auipc	s4,0x1234
//    10028:	ff9ff0ef          	jal	10020 <jal_target>
//
// println!("0x00e68633: {:#034b}", 0x00e68633 as u32);
// println!("0xffe80793: {:#034b}", 0xffe80793 as u32);
// println!("0xfe530ea3: {:#034b}", 0xfe530ea3 as u32);
// println!("0xff390ce3: {:#034b}", 0xff390ce3 as u32);
// println!("0x01234a17: {:#034b}", 0x01234a17 as u32);
// println!("0xff9ff0ef: {:#034b}", 0xff9ff0ef as u32);
//
#[cfg(test)]
mod tests {
    use super::*;

    // add x12, x13, x14 # add rd, rs1, rs2
    // 0x00e68633: 0b00000000111001101000011000110011
    #[test]
    fn test_parse_r_type() {
        let inst = 0b0000000_01110_01101_000_01100_0110011;
        let expected = FormatR {
            rd: 12,
            rs1: 13,
            rs2: 14,
        };
        assert_eq!(parse_r_type(inst), expected);
    }

    // addi x15, x16, -2 # addi rd, rs1, imm
    // 0xffe80793: 0b11111111111010000000011110010011
    #[test]
    fn test_parse_i_type() {
        let inst = 0b111111111110_10000_000_01111_0010011;
        let expected = FormatI {
            rd: 15,
            rs1: 16,
            // imm[11:0], 0b111111111110
            imm: 0b111111111111111111111_11111111110u32 as i32 as GprSigned,
        };

        assert_eq!(expected.imm, -2);
        assert_eq!(parse_i_type(inst), expected);
    }

    // sb x5, -3(x6) # sb rs2, offset(rs1)
    // 0xfe530ea3: 0b11111110010100110000111010100011
    #[test]
    fn test_parse_s_type() {
        let inst = 0b1111111_00101_00110_000_11101_0100011;
        let expected = FormatS {
            rs1: 6,
            rs2: 5,
            // imm[11:5]: 0b1111111, imm[4:0]: 11101
            imm: 0b111111111111111111111_111111_1110_1u32 as i32 as GprSigned,
        };
        assert_eq!(expected.imm, -3);
        assert_eq!(parse_s_type(inst), expected);
    }

    // beq x18, x19, -8 # beq rs1, rs2, offset
    // 0xff390ce3: 0b11111111001110010000110011100011
    #[test]
    fn test_parse_b_type() {
        let inst = 0b1111111_10011_10010_000_11001_1100011;
        let expected = FormatB {
            rs1: 18,
            rs2: 19,
            // imm[12|10:5]: 1111111, imm[4:1|11]:11001
            imm: 0b11111111111111111111_1_111111_1100_0u32 as i32 as GprSigned,
        };
        assert_eq!(expected.imm, -8);
        assert_eq!(parse_b_type(inst), expected);
    }

    // auipc x20, 0x1234 # auipc rd, imm
    // 0x01234a17: 0b00000001001000110100101000010111
    #[test]
    fn test_parse_u_type() {
        let inst = 0b00000001001000110100_10100_0010111;
        let expected = FormatU {
            rd: 20,
            // imm[31:12]: 00000001001000110100
            imm: 0b00000001001000110100_000000000000 as i32 as GprSigned,
        };
        assert_eq!(expected.imm, 0x1234 << 12);
        assert_eq!(parse_u_type(inst), expected);
    }

    // jal x1, -8       # jal rd, offset
    // 0xff9ff0ef: 0b11111111100111111111000011101111
    #[test]
    fn test_parse_j_type() {
        let inst = 0b11111111100111111111_00001_1101111;
        let expected = FormatJ {
            rd: 1,
            // 1_1111111100_1_11111111
            // imm[20|10:1|11|19:12]
            imm: 0b111111111111_11111111_1_1111111100_0u32 as i32 as GprSigned,
        };

        assert_eq!(expected.imm, -8);
        assert_eq!(parse_j_type(inst), expected);
    }
}
